//! # Cindy Frontend
//!
//! This crate implements a Yew application that is the frontend of Cindy. This frontend is mainly
//! used to query, tag and label that has been loaded into Cindy using the command-line interface.

mod component;
mod hooks;
mod request;

/// Commonly used imports, re-exported for convenience.
mod prelude {
    pub use crate::{component::*, hooks::*, request::use_get};
    pub use cindy_common::{api::*, hash::*, tag::*};
    pub use std::borrow::Cow;
    pub use web_sys::HtmlInputElement;
    pub use yew::{
        events::{InputEvent, TargetCast},
        prelude::*,
    };
    pub use yew_hooks::prelude::*;
    pub use yew_router::prelude::*;
}

fn main() {
    wasm_logger::init(Default::default());
    yew::Renderer::<component::App>::new().render();
}
