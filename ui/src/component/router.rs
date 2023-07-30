use crate::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/file/:hash")]
    File { hash: RcHash },
}

impl Route {
    pub fn home() -> Self {
        Self::Home
    }

    pub fn file(hash: RcHash) -> Self {
        Self::File { hash }
    }
}

#[function_component]
pub fn HomeView() -> Html {
    let filters = use_state(|| vec![]);
    let onchange = {
        let filters = filters.clone();
        move |new: Vec<TagPredicate<'static>>| {
            filters.set(new);
        }
    };
    html! {
        <div>
            <NavBar>
                <Search {onchange} />
            </NavBar>
            <SidebarLayout>
                <SidebarLayoutSidebar>
                    <QuerySidebar />
                </SidebarLayoutSidebar>
                <SidebarLayoutContent>
                    <FilesGridLoader query={(*filters).clone()} />
                </SidebarLayoutContent>
            </SidebarLayout>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileViewProps {
    pub hash: RcHash,
}

#[function_component]
pub fn FileView(props: &FileViewProps) -> Html {
    let filters = use_state(|| vec![]);
    let onchange = {
        let filters = filters.clone();
        move |new: Vec<TagPredicate<'static>>| {
            filters.set(new);
        }
    };
    html! {
        <div>
            <NavBar>
                <InspectNavigate />
            </NavBar>
            <SidebarLayout>
                <SidebarLayoutSidebar>
                    <FileSidebar file={props.hash.clone()} />
                </SidebarLayoutSidebar>
                <SidebarLayoutContent>
                    <FileInspect file={props.hash.clone()} overlays=true />
                </SidebarLayoutContent>
            </SidebarLayout>
        </div>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomeView /> },
        Route::File { hash } => html! { <FileView {hash} /> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <CacheProvider>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </CacheProvider>
    }
}
