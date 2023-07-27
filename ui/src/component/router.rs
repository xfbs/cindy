use crate::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/file/:hash")]
    File { hash: RcHash },
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
            <NavBar {onchange} />
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
            <NavBar {onchange} />
            <SidebarLayout>
                <SidebarLayoutSidebar>
                    <FileSidebar />
                </SidebarLayoutSidebar>
                <SidebarLayoutContent>
                <p></p>
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
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
