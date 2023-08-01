use crate::prelude::*;

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
