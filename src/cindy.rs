use crate::{
    cli::{Command, Options},
    config::Config,
    database::Database,
    hash::{Digester, Hash},
};
use anyhow::{anyhow, bail, Result};
use rusqlite::Connection;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs::{create_dir, create_dir_all, metadata, read_to_string, try_exists, write},
    io::ErrorKind,
    sync::{Mutex, OwnedMutexGuard},
};

mod command;

const CINDY_CONFIG: &str = "cindy.toml";

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
            Command::Initialize(command) => {
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
        self.root.join(self.config.data.data_path(hash))
    }

    /// Initialize new Cindy project.
    pub async fn initialize(path: &Path, config: &Config) -> Result<Self> {
        let exists = match metadata(path).await {
            Ok(info) if info.is_dir() => true,
            Ok(info) => bail!(
                "Path {path:?} exists but is not a directory ({:?})",
                info.file_type()
            ),
            Err(error) if error.kind() == ErrorKind::NotFound => false,
            Err(error) => return Err(error.into()),
        };

        if exists {
            if try_exists(path.join(CINDY_CONFIG)).await? {
                bail!("Cindy is already initialized");
            }
        } else {
            create_dir(path).await?;
        }

        // write config
        let config_string = toml::to_string(config)?;
        write(path.join(CINDY_CONFIG), config_string).await?;

        create_dir_all(path.join(&config.data.path)).await?;
        create_dir_all(path.join(&config.thumbs.path)).await?;

        let database: Database = Connection::open(path.join(&config.index.path))?.into();
        database.migrate()?;

        Ok(Self {
            root: path.into(),
            config: config.clone().into(),
            hasher: Arc::new(config.data.hash.clone()),
            database: Arc::new(Mutex::new(database)),
        })
    }

    /// Load Cindy project, will parse Config file.
    pub async fn load(config: &Path) -> Result<Self> {
        let path = config.parent().ok_or(anyhow!("Path has no parent"))?;
        let config_string = read_to_string(config).await?;
        let config: Config = toml::from_str(&config_string)?;
        Self::open(path, &config).await
    }

    /// Open Cindy project with supplied configuration.
    pub async fn open(path: &Path, config: &Config) -> Result<Self> {
        let database = Connection::open(path.join(&config.index.path))?;
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
            let path = ancestor.join(CINDY_CONFIG);
            if try_exists(&path).await? {
                return Self::load(&path).await;
            }
        }
        bail!("No cindy project found");
    }
}
