use cindy::{cli::AddCommand, hash::DataHasher, Cindy, Command, Config, Tag, TagFilter};
use std::{fs::*, path::Path};
use tempfile::tempdir;

fn assert_file(path: &Path) {
    let metadata = metadata(path).unwrap();
    assert!(metadata.is_file());
}

fn assert_dir(path: &Path) {
    let metadata = metadata(path).unwrap();
    assert!(metadata.is_dir());
}

fn assert_config(path: &Path, config: &Config) {
    let contents = read_to_string(path).unwrap();
    let parsed: Config = toml::from_str(&contents).unwrap();
    assert_eq!(config, &parsed);
}

#[tokio::test]
async fn test_initialize() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let _cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    assert_dir(&dir.path());
    assert_dir(&dir.path().join(&config.data.path));
    assert_dir(&dir.path().join(&config.thumbs.path));
    assert_config(&dir.path().join("cindy.toml"), &config);
}

#[tokio::test]
async fn test_discover_root() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let _cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();
    let cindy = Cindy::discover(&dir.path()).await.unwrap();
    assert_eq!(cindy.config().as_ref(), &config);
    assert_eq!(cindy.root(), dir.path());
}

#[tokio::test]
async fn test_discover_subfolder() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let _cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();
    let path = dir.path().join("sub").join("folder");
    create_dir_all(&path).unwrap();
    let cindy = Cindy::discover(&path).await.unwrap();
    assert_eq!(cindy.config().as_ref(), &config);
    assert_eq!(cindy.root(), dir.path());
}

#[tokio::test]
async fn test_load() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let _cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();
    let cindy = Cindy::load(&dir.path().join("cindy.toml")).await.unwrap();
    assert_eq!(cindy.config().as_ref(), &config);
    assert_eq!(cindy.root(), dir.path());
}

#[tokio::test]
async fn test_add_file() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    // create file
    let content = "hello";
    create_dir(dir.path().join("folder")).unwrap();
    let file_path = dir.path().join("folder").join("file.txt");
    write(&file_path, content).unwrap();

    // add single file
    cindy
        .command(&Command::Add(AddCommand {
            paths: vec![file_path],
            recursive: false,
        }))
        .await
        .unwrap();

    let hash = cindy.hasher().hash_data(content.as_bytes());

    let database = cindy.database().await;
    let tags = database.hash_tags(&hash).unwrap();
    drop(database);

    assert_eq!(
        tags,
        [
            Tag::new("filename".into(), "file.txt".into()),
            Tag::new("filesize".into(), content.len().to_string()),
            Tag::new("path".into(), "/folder/file.txt".into()),
            Tag::new("pathprefix".into(), "/folder".into()),
            Tag::new("pathprefix".into(), "/".into()),
        ]
        .into()
    );
}

#[tokio::test]
async fn test_add_files_recursively() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    // create files
    let file1 = "hello";
    let file2 = "world";
    let file3 = "test";
    let folder = dir.path().join("folder");
    create_dir(&folder).unwrap();
    write(&folder.join("file1.txt"), file1).unwrap();
    write(&folder.join("file2.txt"), file2).unwrap();
    write(&folder.join("file3.txt"), file3).unwrap();

    // add files recursively
    cindy
        .command(&Command::Add(AddCommand {
            paths: vec![dir.path().join("folder")],
            recursive: true,
        }))
        .await
        .unwrap();

    let file1 = cindy.hasher().hash_data(&file1.as_bytes());
    let file2 = cindy.hasher().hash_data(&file2.as_bytes());
    let file3 = cindy.hasher().hash_data(&file3.as_bytes());

    let database = cindy.database().await;
    let tags = database.hash_tags(&file1).unwrap();
    drop(database);

    assert_eq!(
        tags,
        [
            Tag::new("filename".into(), "file1.txt".into()),
            Tag::new("filesize".into(), "5".into()),
            Tag::new("path".into(), "/folder/file1.txt".into()),
            Tag::new("pathprefix".into(), "/folder".into()),
            Tag::new("pathprefix".into(), "/".into()),
        ]
        .into()
    );

    let database = cindy.database().await;
    let tags = database.hash_tags(&file2).unwrap();
    drop(database);

    assert_eq!(
        tags,
        [
            Tag::new("filename".into(), "file2.txt".into()),
            Tag::new("filesize".into(), "5".into()),
            Tag::new("path".into(), "/folder/file2.txt".into()),
            Tag::new("pathprefix".into(), "/folder".into()),
            Tag::new("pathprefix".into(), "/".into()),
        ]
        .into()
    );

    let database = cindy.database().await;
    let hashes = database
        .hash_query(&mut [TagFilter::new(Some("pathprefix"), Some("/folder")).into()].iter())
        .unwrap();
    drop(database);

    assert_eq!(hashes, [file1, file2.clone(), file3].into());

    let database = cindy.database().await;
    let hashes = database
        .hash_query(&mut [TagFilter::new(Some("filename"), Some("file2.txt")).into()].iter())
        .unwrap();
    drop(database);

    assert_eq!(hashes, [file2].into());
}
