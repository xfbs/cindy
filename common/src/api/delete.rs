use crate::{api::InputFormat, hash::*};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};

pub trait DeleteRequest {
    fn path(&self) -> Cow<'_, str>;
}

pub struct TagDelete<S: Borrow<str>> {
    pub name: S,
    pub value: S,
}

impl<S: Borrow<str>> DeleteRequest for TagDelete<S> {
    fn path(&self) -> Cow<'_, str> {
        let name = self.name.borrow();
        let value = self.value.borrow();
        format!("api/v1/tags/{name}/{value}").into()
    }
}

pub struct FileTagDelete<H: Borrow<Hash>, S: Borrow<str>> {
    pub hash: H,
    pub name: S,
    pub value: S,
}

impl<H: Borrow<Hash>, S: Borrow<str>> DeleteRequest for FileTagDelete<H, S> {
    fn path(&self) -> Cow<'_, str> {
        let hash = self.hash.borrow();
        let name = self.name.borrow();
        let value = self.value.borrow();
        format!("api/v1/file/{hash}/tag/{name}/{value}").into()
    }
}
