use cindy_common::{
    api::{FileContent, FileQuery, FileTags, GetRequest},
    hash::BoxHash,
    tag::Tag,
};
use std::borrow::Cow;
use yew::prelude::*;
use yew_router::prelude::*;

mod request;
use request::use_get;

mod component;
use component::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/query")]
    Query,
    #[at("/file")]
    File,
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
        <NavBar />
        <Content />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
