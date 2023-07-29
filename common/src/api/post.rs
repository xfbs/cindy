use crate::{api::InputFormat, hash::*};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};

pub trait PostRequest {
    type Input: InputFormat;

    fn path(&self) -> Cow<'_, str>;

    fn body(&self) -> Option<Self::Input> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagCreate<S: Borrow<str>> {
    pub name: S,
    pub value: S,
}

impl<S: Borrow<str>> PostRequest for TagCreate<S> {
    type Input = ();

    fn path(&self) -> Cow<'_, str> {
        format!("api/v1/tags/{}/{}", self.name.borrow(), self.value.borrow()).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTagCreate<H: Borrow<Hash>, S: Borrow<str>> {
    pub hash: H,
    pub name: S,
    pub value: S,
}

impl<H: Borrow<Hash>, S: Borrow<str>> PostRequest for FileTagCreate<H, S> {
    type Input = ();

    fn path(&self) -> Cow<'_, str> {
        let hash = self.hash.borrow();
        let name = self.name.borrow();
        let value = self.value.borrow();
        format!("api/v1/file/{hash}/tag/{name}/{value}").into()
    }
}
