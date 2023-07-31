use super::{Json, OutputFormat};
use crate::{
    api::query::TagQuery,
    cache::*,
    tag::{TagNameInfo, TagValueInfo},
    BoxHash, Hash, Mutation, Tag, TagPredicate,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    collections::BTreeMap,
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

pub trait GetRequest {
    type Output: OutputFormat;
    type Query<'a>: Serialize
    where
        Self: 'a;

    fn path(&self) -> Cow<'_, str>;

    fn query(&self) -> Option<Self::Query<'_>> {
        None
    }

    fn uri(&self) -> String {
        let mut path = self.path().into_owned();
        if let Some(query) = self.query() {
            let query_string = serde_qs::to_string(&query).unwrap();
            if !query_string.is_empty() {
                path.push('?');
                path.push_str(&query_string);
            }
        }
        path
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileContent<H: Borrow<Hash> = BoxHash> {
    pub hash: H,
}

impl<H: Borrow<Hash>> GetRequest for FileContent<H> {
    type Output = Bytes;
    type Query<'a> = () where H: 'a;

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
    type Output = Json<Vec<Tag>>;
    type Query<'a> = TagQuery<&'a str> where H: 'a, S: 'a;

    fn path(&self) -> Cow<'_, str> {
        format!("api/v1/file/{}/tags", self.hash.borrow(),).into()
    }

    fn query(&self) -> Option<Self::Query<'_>> {
        Some(TagQuery {
            name: self.name.as_ref().map(Borrow::borrow),
            value: self.value.as_ref().map(Borrow::borrow),
        })
    }
}

impl<H: Borrow<Hash>, S: Borrow<str>> Invalidatable for FileTags<H, S> {
    fn invalidated_by(&self, mutation: &Mutation) -> bool {
        match mutation {
            //Mutation::File(file) => file == self.hash.borrow(),
            _ => true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileQuery<'a> {
    #[serde(default)]
    pub query: Cow<'a, [TagPredicate<'a>]>,
}

impl<'a> GetRequest for FileQuery<'a> {
    type Output = Json<Vec<BoxHash>>;
    type Query<'b> = &'b Self where 'a: 'b;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query".into()
    }

    fn query(&self) -> Option<Self::Query<'_>> {
        Some(&self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagNames;

impl GetRequest for TagNames {
    type Output = Json<BTreeMap<String, TagNameInfo>>;
    type Query<'a> = ();

    fn path(&self) -> Cow<'_, str> {
        "api/v1/tags/names".into()
    }
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
    type Output = Json<BTreeMap<Tag, TagValueInfo>>;
    type Query<'a> = TagQuery<&'a str> where N: 'a, V: 'a;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/tags/values".into()
    }

    fn query(&self) -> Option<Self::Query<'_>> {
        Some(TagQuery {
            name: self.name.as_ref().map(Borrow::borrow),
            value: self.value.as_ref().map(Borrow::borrow),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrontendFile<P: Borrow<Path>> {
    pub path: P,
}

impl<P: Borrow<Path>> GetRequest for FrontendFile<P> {
    type Output = Bytes;
    type Query<'a> = () where P: 'a;

    fn path(&self) -> Cow<'_, str> {
        self.path.borrow().display().to_string().into()
    }
}
