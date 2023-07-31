use crate::{api::TagQuery, hash::*};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};

pub trait DeleteRequest {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagDelete<S: Borrow<str>> {
    pub name: Option<S>,
    pub value: Option<S>,
}

impl<S: Borrow<str>> DeleteRequest for TagDelete<S> {
    type Query<'a> = TagQuery<&'a str> where S: 'a;
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
pub struct FileTagDelete<H: Borrow<Hash>, S: Borrow<str>> {
    pub hash: H,
    pub name: Option<S>,
    pub value: Option<S>,
}

impl<H: Borrow<Hash>, S: Borrow<str>> DeleteRequest for FileTagDelete<H, S> {
    type Query<'a> = TagQuery<&'a str> where S: 'a, H: 'a;

    fn path(&self) -> Cow<'_, str> {
        let hash = self.hash.borrow();
        format!("api/v1/file/{hash}/tags").into()
    }

    fn query(&self) -> Option<Self::Query<'_>> {
        Some(TagQuery {
            name: self.name.as_ref().map(Borrow::borrow),
            value: self.value.as_ref().map(Borrow::borrow),
        })
    }
}
