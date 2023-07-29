use cindy::{cli::*, hash::DataHasher, Cindy, Command, Config, Tag, TagFilter};
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
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    assert_eq!(cindy.root(), dir.path());
    assert_eq!(cindy.cindy_folder(), dir.path().join(".cindy"));
    assert_eq!(
        cindy.data_path(),
        dir.path().join(".cindy").join(&config.data.path)
    );
    assert_eq!(
        cindy.thumbs_path(),
        dir.path().join(".cindy").join(&config.thumbs.path)
    );
    assert_eq!(
        cindy.config_path(),
        dir.path().join(".cindy").join("config.toml")
    );

    assert_dir(cindy.root());
    assert_dir(&cindy.data_path());
    assert_dir(&cindy.thumbs_path());
    assert_file(&cindy.database_path());
    assert_config(&cindy.config_path(), &config);
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
    let cindy = Cindy::load(&dir.path()).await.unwrap();
    assert_eq!(cindy.config().as_ref(), &config);
    assert_eq!(cindy.root(), dir.path());
}

#[tokio::test]
async fn test_command_init() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();
    cindy
        .command(&Command::Init(InitCommand { path: ".".into() }))
        .await
        .unwrap();
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

    // determine hash of file
    let hash = cindy.hasher().hash_data(content.as_bytes());

    // make sure the path is in the data index
    let path = cindy.hash_path(&hash);
    assert_eq!(path.exists(), true);

    // make sure the tags are in the database
    let database = cindy.database().await;
    let tags = database.hash_tags(&hash, None, None).unwrap();
    drop(database);
    assert!(tags.contains(&Tag::new("filename".into(), "file.txt".into())));
    assert!(tags.contains(&Tag::new("filesize".into(), content.len().to_string())));
    assert!(tags.contains(&Tag::new("path".into(), "/folder/file.txt".into())));
    assert!(tags.contains(&Tag::new("ancestor".into(), "/folder".into())));
    assert!(tags.contains(&Tag::new("ancestor".into(), "/".into())));
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

    // make sure that the files are in the data index
    assert!(cindy.hash_path(&file1).exists());
    assert!(cindy.hash_path(&file2).exists());
    assert!(cindy.hash_path(&file3).exists());

    // make sure the right tags are in the database
    let database = cindy.database().await;
    let tags = database.hash_tags(&file1, None, None).unwrap();
    drop(database);

    assert!(tags.contains(&Tag::new("filename".into(), "file1.txt".into())));
    assert!(tags.contains(&Tag::new("filesize".into(), "5".into())));
    assert!(tags.contains(&Tag::new("path".into(), "/folder/file1.txt".into())));
    assert!(tags.contains(&Tag::new("ancestor".into(), "/folder".into())));
    assert!(tags.contains(&Tag::new("ancestor".into(), "/".into())));

    let database = cindy.database().await;
    let tags = database.hash_tags(&file2, None, None).unwrap();
    drop(database);

    assert!(tags.contains(&Tag::new("filename".into(), "file2.txt".into())));
    assert!(tags.contains(&Tag::new("filesize".into(), "5".into())));
    assert!(tags.contains(&Tag::new("path".into(), "/folder/file2.txt".into())));
    assert!(tags.contains(&Tag::new("ancestor".into(), "/folder".into())));
    assert!(tags.contains(&Tag::new("ancestor".into(), "/".into())));

    let database = cindy.database().await;
    let hashes = database
        .hash_query(&mut [TagFilter::new(Some("ancestor"), Some("/folder")).into()].iter())
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

#[cfg(feature = "ffmpeg")]
#[tokio::test]
async fn test_media_info_tags() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    // copy media
    copy("samples/image1.jpg", dir.path().join("image.jpg")).unwrap();
    copy("samples/image1.png", dir.path().join("image.png")).unwrap();
    copy("samples/video1.avi", dir.path().join("image.avi")).unwrap();

    // add media
    cindy
        .command(&Command::Add(AddCommand {
            paths: vec![dir.path().into()],
            recursive: true,
        }))
        .await
        .unwrap();

    let database = cindy.database().await;
    let hashes = database
        .hash_query(&mut [TagFilter::new(Some("media"), Some("image")).into()].iter())
        .unwrap();
    drop(database);
    assert_eq!(hashes.len(), 2);
}

#[tokio::test]
async fn test_query() {
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

    // determine hash of file
    let hash = cindy.hasher().hash_data(content.as_bytes());

    // make sure the path is in the data index
    let path = cindy.hash_path(&hash);
    assert_eq!(path.exists(), true);

    // TODO: actually test output?
    cindy
        .command(&Command::Query(QueryCommand {
            filters: vec![],
            paths: false,
            tags: false,
        }))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_tags_list() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    cindy
        .command(&Command::Tags(TagsCommand::List(TagsListCommand {
            tags: vec![],
        })))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_tags_create() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();
    let tag1 = Tag::new("name".into(), "value".into());
    let tag2 = Tag::new("name".into(), "other".into());

    cindy
        .command(&Command::Tags(TagsCommand::Create(TagsCreateCommand {
            tags: vec![tag1.clone(), tag2.clone()],
        })))
        .await
        .unwrap();

    let database = cindy.database().await;
    let tags = database.tag_list(None, None).unwrap();
    drop(database);
    assert!(tags.contains_key(&tag1));
    assert!(tags.contains_key(&tag2));
}
