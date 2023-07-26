use crate::{BoxHash, Hash, Tag, TagPredicate};
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize, Deserialize};
use std::borrow::Cow;

pub trait OutputFormat: Sized {
    fn decode(data: Bytes) -> Result<Self, serde_json::Error>;
}

#[derive(Clone, Debug)]
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> OutputFormat for Json<T> {
    fn decode(data: Bytes) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&data[..]).map(Json)
    }
}

impl OutputFormat for Bytes {
    fn decode(data: Bytes) -> Result<Self, serde_json::Error> {
        Ok(data)
    }
}

pub trait GetRequest {
    type Output: OutputFormat;
    type Query: Serialize;

    fn path(&self) -> Cow<'_, str>;

    fn query(&self) -> Option<&Self::Query> {
        None
    }

    fn uri(&self) -> String {
        let mut path = self.path().into_owned();
        if let Some(query) = self.query() {
            let query_string = serde_qs::to_string(query).unwrap();
            if !query_string.is_empty() {
                path.push('?');
                path.push_str(&query_string);
            }
        }
        path
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileContent<'a> {
    pub hash: Cow<'a, Hash>,
}

impl<'a> GetRequest for FileContent<'a> {
    type Output = Bytes;
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        format!("api/v1/file/{}", self.hash).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTags<'a> {
    pub hash: Cow<'a, Hash>,
    pub name: Option<Cow<'a, str>>,
    pub value: Option<Cow<'a, str>>,
}

impl<'a> GetRequest for FileTags<'a> {
    type Output = Json<Vec<Tag>>;
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        format!(
            "api/v1/file/{}/tag/{}/{}",
            self.hash,
            self.name.as_ref().map(Cow::as_ref).unwrap_or("*"),
            self.value.as_ref().map(Cow::as_ref).unwrap_or("*")
        ).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileQuery<'a> {
    #[serde(default)]
    pub query: Cow<'a, [TagPredicate<'a>]>,
}

impl<'a> GetRequest for FileQuery<'a> {
    type Output = Json<Vec<BoxHash>>;
    type Query = Self;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query".into()
    }

    fn query(&self) -> Option<&Self::Query> {
        Some(&self)
    }
}
