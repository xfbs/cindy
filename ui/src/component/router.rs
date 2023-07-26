use crate::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/query")]
    Query,
    #[at("/file")]
    File,
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

#[function_component]
pub fn App() -> Html {
    html! {
        <HomeView />
    }
}
