use crate::cache::*;
use async_trait::async_trait;
use cindy_common::{api::*, cache::RcValue};
use gloo_net::{
    http::{Request, RequestBuilder, Response},
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Get<T: GetRequest>(pub T);

#[async_trait(?Send)]
impl<T: GetRequest> GlooRequest for Get<T>
where
    T::Response: GlooDecodable,
    <T::Response as ResponseEncoding>::Target: Clone + 'static,
{
    type Response = <T::Response as ResponseEncoding>::Target;
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.uri());
        let response = Request::get(&path).send().await?;
        <T::Response as GlooDecodable>::decode(&response).await
    }
}

#[derive(Clone, Debug)]
pub struct Post<T: PostRequest>(pub T);

#[async_trait(?Send)]
impl<T: PostRequest> GlooRequest for Post<T>
where
    T::Request: GlooEncodable,
{
    type Response = ();
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.path());
        let builder = Request::post(&path);
        self.0.body().gloo_encode(builder)?.send().await?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Delete<T: DeleteRequest>(pub T);

#[async_trait(?Send)]
impl<T: DeleteRequest> GlooRequest for Delete<T> {
    type Response = ();
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.uri());
        Request::delete(&path).send().await?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Patch<T: PatchRequest>(pub T);

#[async_trait(?Send)]
impl<T: PatchRequest> GlooRequest for Patch<T>
where
    T::Request: GlooEncodable,
{
    type Response = ();
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.path());
        let builder = Request::patch(&path);
        self.0.body().gloo_encode(builder)?.send().await?;
        Ok(())
    }
}

#[hook]
pub fn use_request<R: GlooRequest + 'static>(request: R) -> UseAsyncHandle<R::Response, Rc<Error>> {
    let cache = use_context::<Cache>().expect("Cache not present");
    let handle = use_async(async move {
        let result = request.send().await.map_err(Rc::new);
        cache.invalidate_all();
        result
    });
    handle
}

#[hook]
pub fn use_post<R: PostRequest + 'static>(request: R) -> UseAsyncHandle<(), Rc<Error>>
where
    R::Request: GlooEncodable,
{
    use_request(Post(request))
}

#[hook]
pub fn use_patch<R: PatchRequest + 'static>(request: R) -> UseAsyncHandle<(), Rc<Error>>
where
    R::Request: GlooEncodable,
{
    use_request(Patch(request))
}

#[hook]
pub fn use_delete<R: DeleteRequest + 'static>(request: R) -> UseAsyncHandle<(), Rc<Error>> {
    use_request(Delete(request))
}

#[hook]
pub fn use_get_cached<R: GetRequest>(data: R) -> RcValue<<R::Response as ResponseEncoding>::Target>
where
    R::Response: GlooDecodable,
    <R::Response as ResponseEncoding>::Target: PartialEq + Clone + 'static,
    Get<R>: CacheItem,
{
    use_cached(Get(data))
}
