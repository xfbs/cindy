use async_trait::async_trait;
use cindy_common::api::{GetRequest, Json};
use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;
use std::rc::Rc;
use yew::functional::hook;
use yew_hooks::prelude::{use_async, use_effect_once, UseAsyncHandle};

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

#[hook]
pub fn use_get<R: GetRequest + 'static>(
    request: R,
) -> UseAsyncHandle<<R::Output as Decodable>::Target, Rc<Error>>
where
    R::Output: Decodable,
{
    let handle = use_async(async move {
        let path = format!("/{}", request.uri());
        let response = Request::get(&path)
            .send()
            .await
            .map_err(Error::from)
            .map_err(Rc::new)?;
        <R::Output as Decodable>::decode(&response)
            .await
            .map_err(Rc::new)
    });
    let handle_clone = handle.clone();
    use_effect_once(move || {
        handle_clone.run();
        || {}
    });
    handle
}
