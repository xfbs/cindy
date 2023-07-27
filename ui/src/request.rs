use async_trait::async_trait;
use cindy_common::api::{GetRequest, Json};
use gloo_net::http::{Request, Response};
use log::info;
use serde::de::DeserializeOwned;
use std::{fmt::Debug, rc::Rc};
use yew::functional::{hook, use_effect, use_state};
use yew_hooks::prelude::{use_async_with_options, UseAsyncHandle, UseAsyncOptions};

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
pub fn use_get<R: GetRequest + Eq + Debug + 'static>(
    request: R,
) -> UseAsyncHandle<<R::Output as Decodable>::Target, Rc<Error>>
where
    R::Output: Decodable,
{
    let request = Rc::new(request);
    let current = use_state({
        let request = request.clone();
        move || request
    });
    let request_clone = request.clone();
    let handle = use_async_with_options(
        async move {
            //info!("GET {:?}", *request_clone);
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
