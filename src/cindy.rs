use crate::{
    cli::{Command, Options},
    config::Config,
    database::Database,
    hash::{Digester, Hash},
};
use anyhow::{bail, Result};
use rusqlite::Connection;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs::{create_dir, create_dir_all, read_to_string, try_exists, write},
    sync::{Mutex, OwnedMutexGuard},
};

const CINDY_CONFIG: &str = "config.toml";
const CINDY_FOLDER: &str = ".cindy";

#[derive(Clone, Debug)]
pub struct Cindy {
    /// Root of the Cindy project.
    root: PathBuf,
    /// Configuration.
    config: Arc<Config>,
    /// Hasher.
    hasher: Arc<dyn Digester + Send + Sync>,
    /// Database handle.
    database: Arc<Mutex<Database>>,
}

impl Cindy {
    /// Create or open Cindy project, depending on command.
    pub async fn new(options: &Options) -> Result<Self> {
        match &options.command {
            Command::Init(command) => {
                let config = Config::default();
                Cindy::initialize(&command.path, &config).await
            }
            _ => Cindy::discover(&std::env::current_dir()?).await,
        }
    }

    /// Root of Cindy project.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Cindy folder
    pub fn cindy_folder(&self) -> PathBuf {
        self.root.join(CINDY_FOLDER)
    }

    pub fn config_path(&self) -> PathBuf {
        self.cindy_folder().join(CINDY_CONFIG)
    }

    pub fn database_path(&self) -> PathBuf {
        self.cindy_folder().join(&self.config.index.path)
    }

    pub fn data_path(&self) -> PathBuf {
        self.cindy_folder().join(&self.config.data.path)
    }

    pub fn thumbs_path(&self) -> PathBuf {
        self.cindy_folder().join(&self.config.thumbs.path)
    }

    /// Config of Cindy.
    pub fn config(&self) -> &Arc<Config> {
        &self.config
    }

    /// Hasher, this is used to calculate hashes of files.
    pub fn hasher(&self) -> &Arc<dyn Digester + Send + Sync> {
        &self.hasher
    }

    /// Get a handle to the database.
    pub async fn database(&self) -> OwnedMutexGuard<Database> {
        self.database.clone().lock_owned().await
    }

    /// Given a hash, determine a path.
    pub fn hash_path(&self, hash: &Hash) -> PathBuf {
        self.cindy_folder().join(self.config.data.data_path(hash))
    }

    /// Initialize new Cindy project.
    pub async fn initialize(path: &Path, config: &Config) -> Result<Self> {
        if !try_exists(path).await? {
            create_dir(path).await?;
        }

        let cindy_dir = path.join(CINDY_FOLDER);
        create_dir(&cindy_dir).await?;

        // write config
        let config_string = toml::to_string(config)?;
        write(cindy_dir.join(CINDY_CONFIG), config_string).await?;

        create_dir_all(cindy_dir.join(&config.data.path)).await?;
        create_dir_all(cindy_dir.join(&config.thumbs.path)).await?;

        let database: Database = Connection::open(cindy_dir.join(&config.index.path))?.into();
        database.migrate()?;

        Ok(Self {
            root: path.into(),
            config: config.clone().into(),
            hasher: Arc::new(config.data.hash.clone()),
            database: Arc::new(Mutex::new(database)),
        })
    }

    /// Load Cindy project, will parse Config file.
    pub async fn load(path: &Path) -> Result<Self> {
        let config_string = read_to_string(&path.join(CINDY_FOLDER).join(CINDY_CONFIG)).await?;
        let config: Config = toml::from_str(&config_string)?;
        Self::open(path, &config).await
    }

    /// Open Cindy project with supplied configuration.
    pub async fn open(path: &Path, config: &Config) -> Result<Self> {
        let database = Connection::open(path.join(CINDY_FOLDER).join(&config.index.path))?;
        Ok(Self {
            root: path.into(),
            config: config.clone().into(),
            hasher: Arc::new(config.data.hash.clone()),
            database: Arc::new(Mutex::new(database.into())),
        })
    }

    /// Discover a Cindy project starting at the supplied path.
    pub async fn discover(path: &Path) -> Result<Self> {
        let path = path.canonicalize()?;
        for ancestor in path.ancestors() {
            if try_exists(&ancestor.join(CINDY_FOLDER)).await? {
                return Self::load(&ancestor).await;
            }
        }
        bail!("No cindy project found");
    }
}
