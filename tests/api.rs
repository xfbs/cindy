use anyhow::{bail, Result};
use async_trait::async_trait;
use axum::{
    http::{Method, Request},
    Router,
};
use cindy::{cli::AddCommand, hash::DataHasher, Cindy, Command, Config};
use cindy_common::{api::*, tag::*};
use hyper::Body;
use std::{borrow::Cow, fs::*};
use tempfile::tempdir;
use tower::{Service, ServiceExt};

#[async_trait(?Send)]
trait RouterExt {
    async fn get<R: GetRequest>(&self, request: R) -> Result<R::Output>;
}

#[async_trait(?Send)]
impl RouterExt for Router {
    async fn get<R: GetRequest>(&self, request: R) -> Result<R::Output> {
        let mut path = format!("/{}", request.path());
        if let Some(query) = request.query() {
            path.push('?');
            path.push_str(&serde_qs::to_string(query)?);
        }
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
    let hash = cindy.hasher().hash_data(&content.as_bytes());
    let router = cindy.router();
    let tags = router
        .get(FileTags {
            hash: Cow::Owned(hash),
            name: None,
            value: None,
        })
        .await
        .unwrap()
        .0;

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
    let receiver = router
        .get(FileContent {
            hash: Cow::Owned(hash),
        })
        .await
        .unwrap();

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
            query: Default::default(),
        })
        .await
        .unwrap()
        .0;

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
        .unwrap()
        .0;

    // validate tags
    assert_eq!(tags, vec![cindy.hasher().hash_data(&file1.as_bytes())]);
}
