use anyhow::{bail, Result};
use async_trait::async_trait;
use axum::{
    http::{Method, Request},
    Router,
};
use cindy::{cli::AddCommand, hash::DataHasher, Cindy, Command, Config};
use cindy_common::{api::*, tag::*};
use hyper::Body;
use std::{fs::*, path::PathBuf};
use tempfile::tempdir;
use tower::ServiceExt;

#[async_trait(?Send)]
trait RouterExt {
    async fn get<R: GetRequest>(&self, request: R) -> Result<<R::Output as OutputFormat>::Target>;
}

#[async_trait(?Send)]
impl RouterExt for Router {
    async fn get<R: GetRequest>(&self, request: R) -> Result<<R::Output as OutputFormat>::Target> {
        let path = format!("/{}", request.uri());
        println!("path {path}");
        let response = self
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&path)
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await?;
        let status = response.status();
        if !status.is_success() {
            bail!("Unsuccessful status: {status}");
        }
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        R::Output::decode(body).map_err(Into::into)
    }
}

#[tokio::test]
async fn test_file_list_tags() {
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
    let hash = cindy.hasher().hash_data(&content.as_bytes());
    let router = cindy.router();
    let tags = router
        .get(FileTags {
            hash: hash,
            name: None,
            value: None::<String>,
        })
        .await
        .unwrap();

    // validate tags
    assert!(!tags.is_empty());
}

#[tokio::test]
async fn test_file_content() {
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
    let hash = cindy.hasher().hash_data(&content.as_bytes());
    let router = cindy.router();
    let receiver = router.get(FileContent { hash: hash }).await.unwrap();

    assert_eq!(receiver, content);
}

#[tokio::test]
async fn test_query_empty() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    // create file
    let content = "hello";
    let file_path = dir.path().join("file.txt");
    write(&file_path, content).unwrap();

    // add single file
    cindy
        .command(&Command::Add(AddCommand {
            paths: vec![file_path],
            recursive: false,
        }))
        .await
        .unwrap();

    // query
    let router = cindy.router();
    let tags = router
        .get(FileQuery {
            query: vec![].into(),
        })
        .await
        .unwrap();

    // validate tags
    assert_eq!(tags, vec![cindy.hasher().hash_data(&content.as_bytes())]);
}

#[tokio::test]
async fn test_query_filename() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    // create file
    let file1 = "hello";
    let file2 = "world";
    write(&dir.path().join("file1.txt"), file1).unwrap();
    write(&dir.path().join("file2.txt"), file2).unwrap();

    // add single file
    cindy
        .command(&Command::Add(AddCommand {
            paths: vec![dir.path().into()],
            recursive: true,
        }))
        .await
        .unwrap();

    // query
    let router = cindy.router();
    let tags = router
        .get(FileQuery {
            query: vec![TagPredicate::Exists(TagFilter::new(
                Some("filename"),
                Some("file1.txt"),
            ))]
            .into(),
        })
        .await
        .unwrap();

    // validate tags
    assert_eq!(tags, vec![cindy.hasher().hash_data(&file1.as_bytes())]);
}

#[tokio::test]
async fn test_list_tag_names() {
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
    let router = cindy.router();
    let tags = router.get(TagNames).await.unwrap();

    // validate tags
    assert!(!tags.is_empty());
    assert!(tags.get("filename").is_some());
    assert!(tags.get("filesize").is_some());
    assert!(tags.get("directory").is_some());
    assert!(tags.get("ancestor").is_some());
    assert!(tags.get("path").is_some());
}

#[tokio::test]
async fn test_list_tags() {
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
    let router = cindy.router();
    let tags = router
        .get(TagList {
            name: Some("filename"),
            value: None::<&str>,
        })
        .await
        .unwrap();

    // validate tags
    assert!(!tags.is_empty());
    assert!(tags.contains(&Tag::new("filename".into(), "file.txt".into())));
}

#[tokio::test]
async fn test_frontend_index() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    // query
    let router = cindy.router();
    let index = router
        .get(FrontendFile {
            path: PathBuf::from("index.html"),
        })
        .await
        .unwrap();

    assert_eq!(index, include_str!("../ui/dist/index.html"));
}

#[tokio::test]
async fn test_frontend_nonexisting() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    // query
    let router = cindy.router();
    let index = router
        .get(FrontendFile {
            path: PathBuf::from("file/abc"),
        })
        .await
        .unwrap();

    assert_eq!(index, include_str!("../ui/dist/index.html"));
}
