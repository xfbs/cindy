use crate::{
    api::{InputFormat, TagQuery},
    hash::*,
};
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

pub struct TagDelete<S: Borrow<str>> {
    pub name: S,
    pub value: S,
}

impl<S: Borrow<str>> DeleteRequest for TagDelete<S> {
    type Query<'a> = TagQuery<&'a str> where S: 'a;
    fn path(&self) -> Cow<'_, str> {
        let name = self.name.borrow();
        let value = self.value.borrow();
        format!("api/v1/tags").into()
    }

    fn query(&self) -> Option<Self::Query<'_>> {
        Some(TagQuery {
            name: Some(self.name.borrow()),
            value: Some(self.value.borrow()),
        })
    }
}

pub struct FileTagDelete<H: Borrow<Hash>, S: Borrow<str>> {
    pub hash: H,
    pub name: S,
    pub value: S,
}

impl<H: Borrow<Hash>, S: Borrow<str>> DeleteRequest for FileTagDelete<H, S> {
    type Query<'a> = TagQuery<&'a str> where S: 'a, H: 'a;

    fn path(&self) -> Cow<'_, str> {
        let hash = self.hash.borrow();
        let name = self.name.borrow();
        let value = self.value.borrow();
        format!("api/v1/file/{hash}/tags").into()
    }

    fn query(&self) -> Option<Self::Query<'_>> {
        Some(TagQuery {
            name: Some(self.name.borrow()),
            value: Some(self.value.borrow()),
        })
    }
}
