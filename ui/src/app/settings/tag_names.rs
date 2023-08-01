use super::*;

#[derive(Properties, PartialEq)]
struct TagNameTableProps {
    #[prop_or_default]
    children: Children,
}

#[function_component]
fn TagNameTable(props: &TagNameTableProps) -> Html {
    html! {
        <div class="relative overflow-x-auto shadow-md sm:rounded-lg my-4">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="px-6 py-3">
                        {"Name"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                        {"Slug"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                        {"Values"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                        {"System"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                            <span class="sr-only">{"Edit"}</span>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    { for props.children.iter() }
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TagNameRowProps {
    name: String,
    info: TagNameInfo,
}

#[function_component]
fn TagNameRow(props: &TagNameRowProps) -> Html {
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                {&props.info.display}
            </th>
            <td class="px-6 py-4">
                {&props.name}
            </td>
            <td class="px-6 py-4">
                {&props.info.values}
            </td>
            <td class="px-6 py-4">
                if props.info.system {
                    {"system"}
                } else {
                    {"regular"}
                }
            </td>
            <td class="px-6 py-4 text-right">
                if !props.info.system {
                    <Link<SettingsRoute> to={SettingsRoute::tag(&props.name)} classes="font-medium text-blue-600 dark:text-blue-500 hover:underline">
                        {"Edit"}
                    </Link<SettingsRoute>>
                }
            </td>
        </tr>
    }
}

#[function_component]
pub fn SettingsTags() -> Html {
    let tags = use_get_cached(TagNames);
    html! {
        <>
            <SidebarHeading>{"Tags"}</SidebarHeading>
            <TagNameTable>
                if let Some(tags) = tags.data() {
                    {
                        for (**tags)
                            .clone()
                            .into_iter()
                            .map(|(name, info)|
                                html!{
                                    <TagNameRow {name} {info} />
                                })
                    }
                }
            </TagNameTable>
        </>
    }
}
