use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChildrenProps {
    pub children: Children,
}

#[function_component]
pub fn SidebarLayout(props: &ChildrenProps) -> Html {
    html! {
        <div class="min-h-screen md:relative my-auto">
            { for props.children.iter() }
        </div>
    }
}

#[function_component]
pub fn SidebarLayoutSidebar(props: &ChildrenProps) -> Html {
    html! {
        <div class="md:min-w-200 md:fixed top-16 right-0 h-full md:max-h-screen md:overflow-scroll md:pb-16">
            { for props.children.iter() }
        </div>
    }
}

#[function_component]
pub fn SidebarLayoutContent(props: &ChildrenProps) -> Html {
    html! {
        <div class="md:mr-96">
            { for props.children.iter() }
        </div>
    }
}
