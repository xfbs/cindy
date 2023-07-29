use async_trait::async_trait;
use cindy_common::api::{GetRequest, InputFormat, Json, PostRequest, DeleteRequest};
use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, rc::Rc};
use yew::functional::{hook, use_effect, use_state};
use yew_hooks::prelude::{use_async, use_async_with_options, UseAsyncHandle, UseAsyncOptions};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] gloo_net::Error),
}

#[async_trait(?Send)]
pub trait Decodable {
    type Target: Clone + 'static;
    async fn decode(response: &Response) -> Result<Self::Target, Error>;
}

#[async_trait(?Send)]
impl<T: DeserializeOwned + Clone + 'static> Decodable for Json<T> {
    type Target = T;
    async fn decode(response: &Response) -> Result<Self::Target, Error> {
        response.json::<T>().await.map_err(Error::from)
    }
}

#[async_trait(?Send)]
impl Decodable for () {
    type Target = ();
    async fn decode(response: &Response) -> Result<Self::Target, Error> {
        Ok(())
    }
}

#[async_trait(?Send)]
pub trait HttpRequest {
    type Response: Clone;
    async fn send(&self) -> Result<Self::Response, Error>;
}

#[derive(Clone, Debug)]
pub struct Get<T: GetRequest>(pub T);

#[async_trait(?Send)]
impl<T: GetRequest> HttpRequest for Get<T>
where
    T::Output: Decodable,
{
    type Response = <T::Output as Decodable>::Target;
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.uri());
        let response = Request::get(&path).send().await?;
        <T::Output as Decodable>::decode(&response).await
    }
}

#[derive(Clone, Debug)]
pub struct Post<T: PostRequest>(pub T);

#[async_trait(?Send)]
impl<T: PostRequest> HttpRequest for Post<T> {
    type Response = ();
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.path());
        let response = Request::post(&path).send().await?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Delete<T: DeleteRequest>(pub T);

#[async_trait(?Send)]
impl<T: DeleteRequest> HttpRequest for Delete<T> {
    type Response = ();
    async fn send(&self) -> Result<Self::Response, Error> {
        let path = format!("/{}", self.0.path());
        let response = Request::delete(&path).send().await?;
        Ok(())
    }
}

#[hook]
pub fn use_get<R: GetRequest + Clone + Eq + Debug + 'static>(
    request: R,
) -> UseAsyncHandle<<R::Output as Decodable>::Target, Rc<Error>>
where
    R::Output: Decodable + Clone,
{
    let request = Rc::new(request);
    let current = use_state({
        let request = request.clone();
        move || request
    });
    let request_clone = request.clone();
    let handle = use_async_with_options(
        async move {
            let path = format!("/{}", request_clone.uri());
            let response = Request::get(&path)
                .send()
                .await
                .map_err(Error::from)
                .map_err(Rc::new)?;
            <R::Output as Decodable>::decode(&response)
                .await
                .map_err(Rc::new)
        },
        UseAsyncOptions::enable_auto(),
    );
    let handle_clone = handle.clone();
    use_effect(move || {
        if *current != request {
            current.set(request);
            handle_clone.run();
        }
        || {}
    });
    handle
}

#[hook]
pub fn use_request<R: HttpRequest + 'static>(request: R) -> UseAsyncHandle<R::Response, Rc<Error>> {
    let handle = use_async(async move { request.send().await.map_err(Rc::new) });
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
