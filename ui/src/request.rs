use crate::cache::*;
use async_trait::async_trait;
use cindy_common::{api::*, cache::RcValue};
use gloo_net::{
    http::{Method as GlooMethod, Request, RequestBuilder, Response},
    Error as GlooError,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, rc::Rc};
use yew::functional::{hook, use_context};
use yew_hooks::prelude::{use_async, UseAsyncHandle};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] gloo_net::Error),
}

#[async_trait(?Send)]
pub trait GlooDecodable: ResponseEncoding {
    async fn decode(response: &Response) -> Result<Self::Target, Error>;
}

#[async_trait(?Send)]
impl<T: DeserializeOwned + Clone + 'static> GlooDecodable for Json<T> {
    async fn decode(response: &Response) -> Result<Self::Target, Error> {
        response.json::<T>().await.map_err(Error::from)
    }
}

#[async_trait(?Send)]
impl GlooDecodable for () {
    async fn decode(_response: &Response) -> Result<Self::Target, Error> {
        Ok(())
    }
}

pub trait GlooEncodable: RequestEncoding {
    fn gloo_encode(&self, builder: RequestBuilder) -> Result<Request, GlooError>;
}

impl GlooEncodable for () {
    fn gloo_encode(&self, builder: RequestBuilder) -> Result<Request, GlooError> {
        builder.build()
    }
}

impl<T: Serialize> GlooEncodable for Json<T> {
    fn gloo_encode(&self, builder: RequestBuilder) -> Result<Request, GlooError> {
        builder
            .header("Content-Type", "application/json")
            .json(&self.0)
    }
}

#[async_trait(?Send)]
pub trait GlooRequest {
    type Response: Clone + 'static;
    async fn send(&self) -> Result<Self::Response, Error>;
}

#[async_trait(?Send)]
impl<T: HttpRequest> GlooRequest for T
where
    T::Response: GlooDecodable,
    <T::Response as ResponseEncoding>::Target: Clone + 'static,
    T::Request: GlooEncodable,
{
    type Response = <T::Response as ResponseEncoding>::Target;
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.uri());
        let method = match self.method() {
            Method::Post => GlooMethod::POST,
            Method::Get => GlooMethod::GET,
            Method::Patch => GlooMethod::PATCH,
            Method::Delete => GlooMethod::DELETE,
        };
        let request = RequestBuilder::new(&path).method(method);
        let response = self.body().gloo_encode(request)?.send().await?;
        <T::Response as GlooDecodable>::decode(&response).await
    }
}

#[hook]
pub fn use_request_then<R: GlooRequest + 'static, F: Fn(&Result<R::Response, Error>) + 'static>(
    request: R,
    after: F,
) -> UseAsyncHandle<R::Response, Rc<Error>> {
    let cache = use_context::<Cache>().expect("Cache not present");
    let handle = use_async(async move {
        let result = request.send().await;
        if let Err(error) = &result {
            log::error!("Error: {error:?}");
        }
        cache.invalidate_all();
        after(&result);
        result.map_err(Rc::new)
    });
    handle
}

#[hook]
pub fn use_request<R: GlooRequest + 'static>(request: R) -> UseAsyncHandle<R::Response, Rc<Error>> {
    use_request_then(request, |_| ())
}
