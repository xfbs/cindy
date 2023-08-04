use anyhow::Result;
use async_trait::async_trait;
use axum::{
    http::{Method, Request},
    Router,
};
use cindy::{cli::AddCommand, hash::DataHasher, Cindy, Command, Config};
use cindy_common::{api::*, tag::*, ErrorResponse};
use hyper::{Body, StatusCode};
use std::{fs::*, path::PathBuf};
use tempfile::tempdir;
use tower::ServiceExt;

#[async_trait(?Send)]
trait RouterExt {
    async fn get<R: GetRequest>(
        &self,
        request: R,
    ) -> Result<<R::Response as ResponseEncoding>::Target>;
    async fn post<R: PostRequest>(&self, request: R) -> Result<()>;
    async fn delete<R: DeleteRequest>(&self, request: R) -> Result<()>;
}

#[async_trait(?Send)]
impl RouterExt for Router {
    async fn get<R: GetRequest>(
        &self,
        request: R,
    ) -> Result<<R::Response as ResponseEncoding>::Target> {
        let path = format!("/{}", request.uri());
        println!("GET {path}");
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
            println!("Unsuccessful status: {status}");
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            if let Ok(string) = std::str::from_utf8(&body) {
                println!("Body: {string}");
            }
            panic!("Error in GET request");
        }
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        R::Response::decode(body).map_err(Into::into)
    }

    async fn post<R: PostRequest>(&self, request: R) -> Result<()> {
        let path = format!("/{}", request.path());
        println!("POST {path}");
        let body: Body = request.body().encode().into();
        let response = self
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&path)
                    .method(Method::POST)
                    .header("Content-Type", "application/json")
                    .body(body)
                    .unwrap(),
            )
            .await?;
        let status = response.status();
        if !status.is_success() {
            println!("Unsuccessful status: {status}");
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            if let Ok(string) = std::str::from_utf8(&body) {
                println!("Body: {string}");
            }
            panic!("Error in GET request");
        }
        Ok(())
    }

    async fn delete<R: DeleteRequest>(&self, request: R) -> Result<()> {
        let path = format!("/{}", request.uri());
        println!("DELETE {path}");
        let response = self
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&path)
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await?;
        let status = response.status();
        if !status.is_success() {
            println!("Unsuccessful status: {status}");
            let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
            if let Ok(string) = std::str::from_utf8(&body) {
                println!("Body: {string}");
            }
            panic!("Error in GET request");
        }
        Ok(())
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
async fn tag_create() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    let router = cindy.router();

    router
        .post(TagNameCreate {
            name: "name",
            display: None,
        })
        .await
        .unwrap();

    router
        .post(TagValueCreate {
            name: "name",
            value: "value",
            display: Some("Name Value"),
        })
        .await
        .unwrap();

    let tags = router
        .get(TagList {
            name: None::<&str>,
            value: None::<&str>,
        })
        .await
        .unwrap();

    assert_eq!(
        tags[&Tag::new("name".into(), "value".into())],
        TagValueInfo {
            files: 0,
            system: false,
            display: "Name Value".into(),
        }
    );
}

#[tokio::test]
async fn tag_delete_one() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    let router = cindy.router();
    router
        .post(TagNameCreate {
            name: "name",
            display: None,
        })
        .await
        .unwrap();

    router
        .post(TagValueCreate {
            name: "name",
            value: "value",
            display: Some("Name Value"),
        })
        .await
        .unwrap();
    router
        .post(TagValueCreate {
            name: "name",
            value: "other",
            display: Some("Other Value"),
        })
        .await
        .unwrap();

    router
        .delete(TagDelete {
            name: Some("name"),
            value: Some("value"),
        })
        .await
        .unwrap();

    let tags = router
        .get(TagList {
            name: None::<&str>,
            value: None::<&str>,
        })
        .await
        .unwrap();

    assert_eq!(
        tags.contains_key(&Tag::new("name".into(), "value".into())),
        false
    );
    assert_eq!(
        tags.contains_key(&Tag::new("name".into(), "other".into())),
        true
    );
}

#[tokio::test]
async fn tag_delete_all_labels() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    let router = cindy.router();
    router
        .post(TagNameCreate {
            name: "name",
            display: None,
        })
        .await
        .unwrap();

    router
        .post(TagValueCreate {
            name: "name",
            value: "value",
            display: Some("Name Value"),
        })
        .await
        .unwrap();
    router
        .post(TagValueCreate {
            name: "name",
            value: "other",
            display: Some("Other Value"),
        })
        .await
        .unwrap();

    router
        .delete(TagDelete {
            name: Some("name"),
            value: None,
        })
        .await
        .unwrap();

    let tags = router
        .get(TagList {
            name: None::<&str>,
            value: None::<&str>,
        })
        .await
        .unwrap();

    assert_eq!(
        tags.contains_key(&Tag::new("name".into(), "value".into())),
        false
    );
    assert_eq!(
        tags.contains_key(&Tag::new("name".into(), "other".into())),
        false
    );
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
    assert!(tags.contains_key(&Tag::new("filename".into(), "file.txt".into())));
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

#[tokio::test]
async fn file_tag_create() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    let router = cindy.router();
    router
        .post(TagNameCreate {
            name: "name",
            display: None,
        })
        .await
        .unwrap();

    router
        .post(TagValueCreate {
            name: "name",
            value: "value",
            display: Some("Name Value"),
        })
        .await
        .unwrap();

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
    let hash = cindy.hasher().hash_data(&content.as_bytes());
    router
        .post(FileTagCreate {
            hash: hash.clone(),
            name: "name",
            value: "value",
        })
        .await
        .unwrap();

    let tags = router
        .get(FileTags {
            hash: hash,
            name: None,
            value: None::<String>,
        })
        .await
        .unwrap();

    assert!(tags.contains(&Tag::new("name".into(), "value".into())));
}

#[tokio::test]
async fn file_tag_delete() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();

    let router = cindy.router();
    router
        .post(TagValueCreate {
            name: "name",
            value: "value",
            display: Some("Name Value"),
        })
        .await
        .unwrap();

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
    let hash = cindy.hasher().hash_data(&content.as_bytes());
    router
        .post(FileTagCreate {
            hash: hash.clone(),
            name: "name",
            value: "value",
        })
        .await
        .unwrap();

    router
        .delete(FileTagDelete {
            hash: hash.clone(),
            name: Some("name"),
            value: Some("value"),
        })
        .await
        .unwrap();

    let tags = router
        .get(FileTags {
            hash: hash,
            name: None,
            value: None::<String>,
        })
        .await
        .unwrap();

    assert!(!tags.contains(&Tag::new("name".into(), "value".into())));
}

#[tokio::test]
async fn api_notfound() {
    let dir = tempdir().unwrap();
    let config = Config::default();
    let cindy = Cindy::initialize(&dir.path(), &config).await.unwrap();
    let response = cindy
        .router()
        .oneshot(
            Request::builder()
                .uri("/api/v1/unknown")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response: ErrorResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(response.error, "not found");
    assert_eq!(response.cause, None);
}
