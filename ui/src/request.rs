use crate::cache::*;
use async_trait::async_trait;
use cindy_common::{
    api::{DeleteRequest, GetRequest, Json, PostRequest, ResponseEncoding},
    cache::RcValue,
};
use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, rc::Rc};
use yew::functional::{hook, use_context};
use yew_hooks::prelude::{use_async, UseAsyncHandle};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] gloo_net::Error),
}

#[async_trait(?Send)]
pub trait Decodable: ResponseEncoding {
    async fn decode(response: &Response) -> Result<Self::Target, Error>;
}

#[async_trait(?Send)]
impl<T: DeserializeOwned + Clone + 'static> Decodable for Json<T> {
    async fn decode(response: &Response) -> Result<Self::Target, Error> {
        response.json::<T>().await.map_err(Error::from)
    }
}

#[async_trait(?Send)]
impl Decodable for () {
    async fn decode(_response: &Response) -> Result<Self::Target, Error> {
        Ok(())
    }
}

#[async_trait(?Send)]
pub trait HttpRequest {
    type Response: Clone + 'static;
    async fn send(&self) -> Result<Self::Response, Error>;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Get<T: GetRequest>(pub T);

#[async_trait(?Send)]
impl<T: GetRequest> HttpRequest for Get<T>
where
    T::Response: Decodable,
    <T::Response as ResponseEncoding>::Target: Clone + 'static,
{
    type Response = <T::Response as ResponseEncoding>::Target;
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.uri());
        let response = Request::get(&path).send().await?;
        <T::Response as Decodable>::decode(&response).await
    }
}

#[derive(Clone, Debug)]
pub struct Post<T: PostRequest>(pub T);

#[async_trait(?Send)]
impl<T: PostRequest> HttpRequest for Post<T> {
    type Response = ();
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.path());
        Request::post(&path)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Delete<T: DeleteRequest>(pub T);

#[async_trait(?Send)]
impl<T: DeleteRequest> HttpRequest for Delete<T> {
    type Response = ();
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.uri());
        Request::delete(&path).send().await?;
        Ok(())
    }
}

#[hook]
pub fn use_request<R: HttpRequest + 'static>(request: R) -> UseAsyncHandle<R::Response, Rc<Error>> {
    let cache = use_context::<Cache>().expect("Cache not present");
    let handle = use_async(async move {
        let result = request.send().await.map_err(Rc::new);
        cache.invalidate_all();
        result
    });
    handle
}

#[hook]
pub fn use_post<R: PostRequest + 'static>(request: R) -> UseAsyncHandle<(), Rc<Error>> {
    use_request(Post(request))
}

#[hook]
pub fn use_delete<R: DeleteRequest + 'static>(request: R) -> UseAsyncHandle<(), Rc<Error>> {
    use_request(Delete(request))
}

#[hook]
pub fn use_get_cached<R: GetRequest>(data: R) -> RcValue<<R::Response as ResponseEncoding>::Target>
where
    R::Response: Decodable,
    <R::Response as ResponseEncoding>::Target: PartialEq + Clone + 'static,
    Get<R>: CacheItem,
{
    use_cached(Get(data))
}
