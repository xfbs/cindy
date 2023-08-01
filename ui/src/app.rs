use crate::prelude::*;

mod home;
use home::*;

mod file;
use file::*;

mod settings;
use settings::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/file/:hash")]
    File { hash: RcHash },
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    SettingsChild,
}

impl Route {
    pub fn home() -> Self {
        Self::Home
    }

    pub fn file(hash: RcHash) -> Self {
        Self::File { hash }
    }

    fn switch(self) -> Html {
        match self {
            Route::Home => html! { <HomeView /> },
            Route::File { hash } => html! { <FileView {hash} /> },
            Route::SettingsRoot | Route::SettingsChild => html! { <SettingsView />  },
        }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <CacheProvider>
            <BrowserRouter>
                <Switch<Route> render={Route::switch} />
            </BrowserRouter>
        </CacheProvider>
    }
}
