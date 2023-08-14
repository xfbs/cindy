use crate::cache::Cache;
use restless::clients::{
    gloo::GlooRequest,
    yew::{UseRequestHandle, use_request as use_restless_request},
};
use yew::functional::{hook, use_context};

#[hook]
pub fn use_request_then<R: GlooRequest + 'static, F: Fn(&Result<R::Response, R::Error>) + 'static>(
    request: R,
    after: F,
) -> UseRequestHandle<
    R::Response,
    R::Error,
>
where
    R::Response: Clone + 'static,
{
    let cache = use_context::<Cache>().expect("Cache not present");
    use_restless_request(request, move |result| {
        if let Err(error) = &result {
            log::error!("Error: {error:?}");
        }
        cache.invalidate_all();
        after(&result);
    }, false)
}

#[hook]
pub fn use_request<R: GlooRequest + 'static>(
    request: R,
) -> UseRequestHandle<R::Response, R::Error>
where
    R::Response: Clone + 'static,
{
    use_request_then(request, |_| ())
}
