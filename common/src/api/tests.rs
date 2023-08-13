use super::*;
use crate::hash::Hash;
use restless::*;
use std::path::Path;

trait IntoUri {
    fn uri(&self) -> String;
}

impl<T: GetRequest> IntoUri for T {
    fn uri(&self) -> String {
        restless::methods::Get::from(self).uri()
    }
}

#[test]
fn test_uri() {
    let pairs: &[(&dyn IntoUri, &str)] = &[
        (
            &FrontendFile {
                path: Path::new("index.html"),
            },
            "index.html",
        ),
        (&TagNames, "api/v1/tags"),
        (
            &FileContent {
                hash: Hash::new(&[0xab]),
            },
            "api/v1/file/ab",
        ),
        (
            &FileTags {
                hash: Hash::new(&[0xab]),
                name: None::<&str>,
                value: None::<&str>,
            },
            "api/v1/file/ab/tags",
        ),
    ];

    for (request, uri) in pairs {
        assert_eq!(&request.uri(), uri);
    }
}
