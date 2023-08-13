use crate::{api::TagQuery, hash::*, tag::TagPredicate};
use restless::{methods::Delete, query::Qs, DeleteRequest, RequestMethod};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagDelete<S: Borrow<str>> {
    pub name: Option<S>,
    pub value: Option<S>,
}

impl<S: Borrow<str>> DeleteRequest for TagDelete<S> {
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

impl<S: Borrow<str>> RequestMethod for TagDelete<S> {
    type Method = Delete<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileTagDelete<H: Borrow<Hash>, S: Borrow<str>> {
    pub hash: H,
    pub name: Option<S>,
    pub value: Option<S>,
}

impl<H: Borrow<Hash>, S: Borrow<str>> DeleteRequest for FileTagDelete<H, S> {
    type Query = Qs<TagQuery<String>>;

    fn path(&self) -> Cow<'_, str> {
        let hash = self.hash.borrow();
        format!("api/v1/file/{hash}/tags").into()
    }

    fn query(&self) -> Self::Query {
        TagQuery {
            name: self.name.as_ref().map(Borrow::borrow).map(Into::into),
            value: self.value.as_ref().map(Borrow::borrow).map(Into::into),
        }
        .into()
    }
}

impl<H: Borrow<Hash>, S: Borrow<str>> RequestMethod for FileTagDelete<H, S> {
    type Method = Delete<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryTagRemove<S: Borrow<str>> {
    pub query: Vec<TagPredicate<'static>>,
    pub name: Option<S>,
    pub value: Option<S>,
}

impl<S: Borrow<str>> DeleteRequest for QueryTagRemove<S> {
    type Query = Qs<QueryTagRemove<String>>;

    fn path(&self) -> Cow<'_, str> {
        "api/v1/query/tags".into()
    }

    fn query(&self) -> Self::Query {
        QueryTagRemove {
            query: self.query.clone(),
            name: self.name.as_ref().map(|n| n.borrow().into()),
            value: self.value.as_ref().map(|n| n.borrow().into()),
        }
        .into()
    }
}

impl<S: Borrow<str>> RequestMethod for QueryTagRemove<S> {
    type Method = Delete<Self>;
}
