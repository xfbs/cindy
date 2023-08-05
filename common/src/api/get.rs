use crate::{
    api::{query::TagQuery, GetRequest, Json},
    cache::*,
    tag::{TagNameInfo, TagValueInfo},
    BoxHash, Hash, Tag, TagPredicate,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct QueryState {
    #[serde(default)]
    pub query: Vec<TagPredicate<'static>>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileContent<H: Borrow<Hash> = BoxHash> {
    pub hash: H,
}

impl<H: Borrow<Hash>> GetRequest for FileContent<H> {
    type Response = Bytes;
    type Query = ();

    fn query(&self) -> Self::Query {}

    fn path(&self) -> Cow<'_, str> {
        format!("api/v1/file/{}", self.hash.borrow()).into()
    }
}

impl<H: Borrow<Hash>> Invalidatable for FileContent<H> {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTags<H: Borrow<Hash> = BoxHash, S: Borrow<str> = String> {
    pub hash: H,
    pub name: Option<S>,
    pub value: Option<S>,
}

impl<H: Borrow<Hash>, S: Borrow<str>> GetRequest for FileTags<H, S> {
    type Response = Json<Vec<Tag>>;
    type Query = TagQuery<String>;

    fn path(&self) -> Cow<'_, str> {
        format!("api/v1/file/{}/tags", self.hash.borrow(),).into()
    }

    fn query(&self) -> Self::Query {
        TagQuery {
            name: self.name.as_ref().map(Borrow::borrow).map(Into::into),
            value: self.value.as_ref().map(Borrow::borrow).map(Into::into),
        }
    }
}

impl<H: Borrow<Hash>, S: Borrow<str>> Invalidatable for FileTags<H, S> {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileQuery<'a> {
    #[serde(default)]
    pub query: Cow<'a, [TagPredicate<'a>]>,
}

impl<'a> GetRequest for FileQuery<'a> {
    type Response = Json<Vec<BoxHash>>;
    type Query = Self;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query".into()
    }

    fn query(&self) -> Self::Query {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagNames;

impl GetRequest for TagNames {
    type Response = Json<BTreeMap<String, TagNameInfo>>;
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        "api/v1/tags".into()
    }

    fn query(&self) -> Self::Query {}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagList<N: Borrow<str> = String, V: Borrow<str> = String> {
    pub name: Option<N>,
    pub value: Option<V>,
}

impl<N, V> GetRequest for TagList<N, V>
where
    N: Borrow<str>,
    V: Borrow<str>,
{
    type Response = Json<BTreeMap<Tag, TagValueInfo>>;
    type Query = TagQuery<String>;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/tags/values".into()
    }

    fn query(&self) -> Self::Query {
        TagQuery {
            name: self.name.as_ref().map(Borrow::borrow).map(Into::into),
            value: self.value.as_ref().map(Borrow::borrow).map(Into::into),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrontendFile<P: Borrow<Path>> {
    pub path: P,
}

impl<P: Borrow<Path>> GetRequest for FrontendFile<P> {
    type Response = Bytes;
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        self.path.borrow().display().to_string().into()
    }

    fn query(&self) -> Self::Query {}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryTagsResponse {
    pub tags: BTreeSet<Tag>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryTagsUnion {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub query: Vec<TagPredicate<'static>>,
}

impl GetRequest for QueryTagsUnion {
    type Response = Json<QueryTagsResponse>;
    type Query = Self;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query/tags/union".into()
    }

    fn query(&self) -> Self::Query {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryTagsIntersection {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub query: Vec<TagPredicate<'static>>,
}

impl GetRequest for QueryTagsIntersection {
    type Response = Json<QueryTagsResponse>;
    type Query = Self;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query/tags/intersection".into()
    }

    fn query(&self) -> Self::Query {
        self.clone()
    }
}
