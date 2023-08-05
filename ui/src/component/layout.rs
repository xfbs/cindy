use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChildrenProps {
    pub children: Children,
}

#[function_component]
pub fn SidebarLayout(props: &ChildrenProps) -> Html {
    html! {
        <div class="h-full min-h-full md:relative my-auto">
            { for props.children.iter() }
        </div>
    }
}

#[function_component]
pub fn SidebarLayoutSidebar(props: &ChildrenProps) -> Html {
    html! {
        <div class="md:min-w-200 md:fixed top-16 right-0 h-full md:overflow-scroll md:pb-16">
            { for props.children.iter() }
        </div>
    }
}

#[function_component]
pub fn SidebarLayoutContent(props: &ChildrenProps) -> Html {
    html! {
        <div class="md:mr-96 h-full">
            { for props.children.iter() }
        </div>
    }
}

#[function_component]
pub fn Grid(props: &ChildrenProps) -> Html {
    html! {
        <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 gap-3 p-3">
        { for props.children.iter() }
        </div>
    }
}
