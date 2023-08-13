use crate::{
    api::query::TagQuery,
    cache::*,
    tag::{TagNameInfo, TagValueInfo},
    BoxHash, Hash, Tag, TagPredicate,
};
use bytes::Bytes;
use restless::{data::Json, methods::Get, query::Qs, GetRequest, RequestMethod};
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

impl<H: Borrow<Hash>> RequestMethod for FileContent<H> {
    type Method = Get<Self>;
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
    type Query = Qs<TagQuery<String>>;

    fn path(&self) -> Cow<'_, str> {
        format!("api/v1/file/{}/tags", self.hash.borrow(),).into()
    }

    fn query(&self) -> Self::Query {
        TagQuery {
            name: self.name.as_ref().map(Borrow::borrow).map(Into::into),
            value: self.value.as_ref().map(Borrow::borrow).map(Into::into),
        }
        .into()
    }
}

impl<H: Borrow<Hash>, S: Borrow<str>> Invalidatable for FileTags<H, S> {}

impl<H: Borrow<Hash>, S: Borrow<str>> RequestMethod for FileTags<H, S> {
    type Method = Get<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryFiles<'a> {
    #[serde(default)]
    pub query: Cow<'a, [TagPredicate<'a>]>,
}

impl<'a> GetRequest for QueryFiles<'a> {
    type Response = Json<Vec<BoxHash>>;
    type Query = Qs<Self>;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query".into()
    }

    fn query(&self) -> Self::Query {
        self.clone().into()
    }
}

impl<'a> RequestMethod for QueryFiles<'a> {
    type Method = Get<Self>;
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

impl RequestMethod for TagNames {
    type Method = Get<Self>;
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
    type Query = Qs<TagQuery<String>>;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/tags/values".into()
    }

    fn query(&self) -> Self::Query {
        TagQuery {
            name: self.name.as_ref().map(Borrow::borrow).map(Into::into),
            value: self.value.as_ref().map(Borrow::borrow).map(Into::into),
        }
        .into()
    }
}

impl<N: Borrow<str>, V: Borrow<str>> RequestMethod for TagList<N, V> {
    type Method = Get<Self>;
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

impl<P: Borrow<Path>> RequestMethod for FrontendFile<P> {
    type Method = Get<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum QueryTagsMode {
    Union,
    Intersection,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryTags {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub query: Vec<TagPredicate<'static>>,
    pub mode: QueryTagsMode,
}

impl GetRequest for QueryTags {
    type Response = Json<BTreeSet<Tag>>;
    type Query = Qs<Self>;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query/tags".into()
    }

    fn query(&self) -> Self::Query {
        self.clone().into()
    }
}

impl RequestMethod for QueryTags {
    type Method = Get<Self>;
}
