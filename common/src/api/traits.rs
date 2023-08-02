use serde::{Deserialize, Serialize};
use super::*;
use std::{
    borrow::{Borrow, Cow},
};

pub trait PostRequest {
    type Request: RequestEncoding;

    fn path(&self) -> Cow<'_, str>;
    fn body(&self) -> Self::Request;
}

pub trait DeleteRequest {
    type Query: Serialize;

    fn path(&self) -> Cow<'_, str>;
    fn query(&self) -> Self::Query;
    fn uri(&self) -> String {
        let mut path = self.path().into_owned();
        let query_string = serde_qs::to_string(&self.query()).unwrap();
        if !query_string.is_empty() {
            path.push('?');
            path.push_str(&query_string);
        }
        path
    }
}

pub trait GetRequest {
    type Response: ResponseEncoding;
    type Query: Serialize;

    fn path(&self) -> Cow<'_, str>;
    fn query(&self) -> Self::Query;
    fn uri(&self) -> String {
        let mut path = self.path().into_owned();
        let query_string = serde_qs::to_string(&self.query()).unwrap();
        if !query_string.is_empty() {
            path.push('?');
            path.push_str(&query_string);
        }
        path
    }
}

pub(crate) struct Get<T: GetRequest>(T);

impl<T: GetRequest> From<T> for Get<T> {
    fn from(request: T) -> Self {
        Get(request)
    }
}

pub(crate) struct Post<T: PostRequest>(T);

impl<T: PostRequest> From<T> for Post<T> {
    fn from(request: T) -> Self {
        Post(request)
    }
}

pub(crate) struct Delete<T: DeleteRequest>(T);

impl<T: DeleteRequest> From<T> for Delete<T> {
    fn from(request: T) -> Self {
        Delete(request)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    Post,
    Get,
    Delete,
}

pub trait HttpRequest {
    type Request: RequestEncoding;
    type Response: ResponseEncoding;
    type Query: Serialize;

    fn path(&self) -> Cow<'_, str>;
    fn body(&self) -> Self::Request;
    fn query(&self) -> Self::Query;
    fn method(&self) -> Method;

    fn uri(&self) -> String {
        let mut path = self.path().into_owned();
        let query_string = serde_qs::to_string(&self.query()).unwrap();
        if !query_string.is_empty() {
            path.push('?');
            path.push_str(&query_string);
        }
        path
    }
}

impl<T: GetRequest> HttpRequest for Get<T> {
    type Request = ();
    type Response = T::Response;
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        self.0.path()
    }

    fn body(&self) -> Self::Request {
        ()
    }

    fn query(&self) -> Self::Query {
        self.0.query()
    }

    fn method(&self) -> Method {
        Method::Get
    }
}

impl<T: PostRequest> HttpRequest for Post<T> {
    type Request = T::Request;
    type Response = ();
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        self.0.path()
    }

    fn body(&self) -> Self::Request {
        self.0.body()
    }

    fn query(&self) -> Self::Query {
        ()
    }

    fn method(&self) -> Method {
        Method::Post
    }
}

impl<T: DeleteRequest> HttpRequest for Delete<T> {
    type Request = ();
    type Response = ();
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        self.0.path()
    }

    fn body(&self) -> Self::Request {
        ()
    }

    fn query(&self) -> Self::Query {
        self.0.query()
    }

    fn method(&self) -> Method {
        Method::Delete
    }
}

