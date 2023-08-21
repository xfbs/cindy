//! # Cindy Frontend
//!
//! This crate implements a Yew application that is the frontend of Cindy. This frontend is mainly
//! used to query, tag and label that has been loaded into Cindy using the command-line interface.

mod app;
mod cache;
mod component;
mod hooks;
mod query;
mod request;

/// Commonly used imports, re-exported for convenience.
mod prelude {
    pub use crate::{
        app::*,
        cache::{use_cached, CacheProvider},
        component::*,
        hooks::*,
        query::*,
        request::*,
    };
    pub use cindy_common::{api::*, hash::*, label::*, restless::Request, tag::*};
    pub use implicit_clone::unsync::IString;
    pub use std::borrow::Cow;
    pub use std::rc::Rc;
    pub use web_sys::HtmlInputElement;
    pub use yew::{
        events::{InputEvent, TargetCast},
        prelude::*,
    };
    pub use yew_hooks::prelude::*;
    pub use yew_router::prelude::{use_location, *};

    pub type Link<R = Route, Q = Rc<RawQuery>> = yew_router::prelude::Link<R, Q>;
}

fn main() {
    wasm_logger::init(Default::default());
    yew::Renderer::<app::App>::new().render();
}
