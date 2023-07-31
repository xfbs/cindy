use crate::{
    api::{Json, RequestEncoding},
    hash::*,
};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};

pub trait PostRequest {
    type Request<'a>: RequestEncoding
    where
        Self: 'a;

    fn path(&self) -> Cow<'_, str>;

    fn body(&self) -> Option<Self::Request<'_>> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagCreateBody<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
    pub display: Option<Cow<'a, str>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagCreate<S: Borrow<str>> {
    pub name: S,
    pub value: S,
    pub display: Option<S>,
}

impl<S: Borrow<str>> PostRequest for TagCreate<S> {
    type Request<'a> = Json<TagCreateBody<'a>> where S: 'a;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/tags/values".into()
    }

    fn body(&self) -> Option<Self::Request<'_>> {
        Some(Json(TagCreateBody {
            name: self.name.borrow().into(),
            value: self.value.borrow().into(),
            display: self.display.as_ref().map(Borrow::borrow).map(Into::into),
        }))
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
    type Request<'a> = Json<FileTagCreateBody<'a>> where H: 'a, S: 'a;

    fn path(&self) -> Cow<'_, str> {
        let hash = self.hash.borrow();
        format!("api/v1/file/{hash}/tags").into()
    }

    fn body(&self) -> Option<Self::Request<'_>> {
        Some(Json(FileTagCreateBody {
            name: self.name.borrow().into(),
            value: self.value.borrow().into(),
        }))
    }
}
