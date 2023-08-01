use crate::prelude::*;

mod tag_names;
use tag_names::*;

mod tag_values;
use tag_values::*;

#[derive(Clone, Routable, PartialEq)]
pub enum SettingsRoute {
    #[at("/settings")]
    Root,
    #[at("/settings/tags")]
    Tags,
    #[at("/settings/tag/:name")]
    Tag { name: String },
}

impl SettingsRoute {
    fn tag<S: Into<String>>(name: S) -> Self {
        Self::Tag { name: name.into() }
    }

    fn switch(self) -> Html {
        match self {
            Self::Root => html! { <SettingsRoot /> },
            Self::Tags => html! { <SettingsTags /> },
            Self::Tag { name } => html! { <SettingsTag {name} /> },
        }
    }
}

#[function_component]
fn SettingsRoot() -> Html {
    html! {
        <>
            <p>{"Settings"}</p>
            <Link<SettingsRoute> to={SettingsRoute::Tags}>{"Tags"}</Link<SettingsRoute>>
        </>
    }
}

#[function_component]
pub fn SettingsView() -> Html {
    html! {
        <>
            <NavBar>
            </NavBar>

            <SidebarLayout>
                <SidebarLayoutSidebar>
                    <Sidebar>
                        <SidebarHeading>{"Settings"}</SidebarHeading>
                    </Sidebar>
                </SidebarLayoutSidebar>
                <SidebarLayoutContent>
                    <div class="md:w-1/2 bg-white m-auto p-6">
                    <Switch<SettingsRoute> render={SettingsRoute::switch} />
                    </div>
                </SidebarLayoutContent>
            </SidebarLayout>
        </>
    }
}
