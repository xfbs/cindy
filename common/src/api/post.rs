use crate::{
    api::{Json, PostRequest},
    hash::*,
    TagPredicate,
};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagValueCreateRequest<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
    pub display: Option<Cow<'a, str>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagNameCreateRequest<'a> {
    pub name: Cow<'a, str>,
    pub display: Option<Cow<'a, str>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagNameCreate<S: Borrow<str>> {
    pub name: S,
    pub display: Option<S>,
}

impl<S: Borrow<str>> PostRequest for TagNameCreate<S> {
    type Request = Json<TagNameCreateRequest<'static>>;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/tags".into()
    }

    fn body(&self) -> Self::Request {
        Json(TagNameCreateRequest {
            name: self.name.borrow().to_string().into(),
            display: self
                .display
                .as_ref()
                .map(|value| value.borrow().to_string().into()),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagValueCreate<S: Borrow<str>> {
    pub name: S,
    pub value: S,
    pub display: Option<S>,
}

impl<S: Borrow<str>> PostRequest for TagValueCreate<S> {
    type Request = Json<TagValueCreateRequest<'static>>;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/tags/values".into()
    }

    fn body(&self) -> Self::Request {
        Json(TagValueCreateRequest {
            name: self.name.borrow().to_string().into(),
            value: self.value.borrow().to_string().into(),
            display: self
                .display
                .as_ref()
                .map(|value| value.borrow().to_string().into()),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTagCreate<H: Borrow<Hash>, S: Borrow<str>> {
    pub hash: H,
    pub name: S,
    pub value: S,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTagCreateBody<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<H: Borrow<Hash>, S: Borrow<str>> PostRequest for FileTagCreate<H, S> {
    type Request = Json<FileTagCreateBody<'static>>;

    fn path(&self) -> Cow<'_, str> {
        let hash = self.hash.borrow();
        format!("api/v1/file/{hash}/tags").into()
    }

    fn body(&self) -> Self::Request {
        Json(FileTagCreateBody {
            name: self.name.borrow().to_string().into(),
            value: self.value.borrow().to_string().into(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryTagCreate<S: Borrow<str>> {
    pub query: Vec<TagPredicate<'static>>,
    pub name: S,
    pub value: S,
}

impl<S: Borrow<str>> PostRequest for QueryTagCreate<S> {
    type Request = Json<QueryTagCreate<String>>;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query/tags".into()
    }

    fn body(&self) -> Self::Request {
        Json(QueryTagCreate {
            query: self.query.clone(),
            name: self.name.borrow().into(),
            value: self.value.borrow().into(),
        })
    }
}
