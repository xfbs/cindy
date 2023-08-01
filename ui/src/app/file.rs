use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileViewProps {
    pub hash: RcHash,
}

#[function_component]
pub fn FileView(props: &FileViewProps) -> Html {
    //let filters = use_state(|| vec![]);
    /*
    let onchange = {
        let filters = filters.clone();
        move |new: Vec<TagPredicate<'static>>| {
            filters.set(new);
        }
    };
    */
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
