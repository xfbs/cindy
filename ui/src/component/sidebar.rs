use crate::prelude::*;
use uuid::Uuid;
use web_sys::{HtmlElement, HtmlSelectElement};

#[derive(Properties, PartialEq)]
pub struct TagsListHeaderProps {
    #[prop_or_default]
    pub actions: bool,
}

#[function_component]
pub fn TagsListHeader(props: &TagsListHeaderProps) -> Html {
    html! {
        <thead class="text-xs text-gray-700 uppercase bg-gray-200 dark:bg-gray-700 dark:text-gray-400">
            <tr>
                <th scope="col" class="px-3 py-3 pl-1">
                    {"Name"}
                </th>
                if props.actions {
                    <th scope="col" class="px-3 py-3">
                        {"Value"}
                    </th>
                    <th scope="col" class="px-3 py-3 pr-1">
                    </th>
                } else {
                    <th scope="col" class="px-3 py-3 pr-1">
                        {"Value"}
                    </th>
                }
            </tr>
        </thead>
    }
}

#[derive(Properties, PartialEq)]
pub struct RowDeleteButtonProps {
    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component]
fn RowDeleteButton(props: &RowDeleteButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let onclick = move |_| onclick.emit(());
    html! {
        <button class="w-4 h-4 bg-red-500 rounded hover:border hover:border-red-600" {onclick}>
            <svg viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M6.79289 7.49998L4.14645 4.85353L4.85355 4.14642L7.5 6.79287L10.1464 4.14642L10.8536 4.85353L8.20711 7.49998L10.8536 10.1464L10.1464 10.8535L7.5 8.20708L4.85355 10.8535L4.14645 10.1464L6.79289 7.49998Z" fill="#000000"/>
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct RowSubmitButtonProps {
    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component]
fn RowSubmitButton(props: &RowSubmitButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let onclick = move |_| onclick.emit(());
    html! {
        <button class="w-4 h-4 bg-green-500 rounded hover:border hover:border-green-600" {onclick}>
            <svg viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M6.79289 7.49998L4.14645 4.85353L4.85355 4.14642L7.5 6.79287L10.1464 4.14642L10.8536 4.85353L8.20711 7.49998L10.8536 10.1464L10.1464 10.8535L7.5 8.20708L4.85355 10.8535L4.14645 10.1464L6.79289 7.49998Z" fill="#000000"/>
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TagsListRowProps {
    pub tag: Tag,
}

#[function_component]
pub fn TagsListRow(props: &TagsListRowProps) -> Html {
    let tag_value = use_cached(
        TagList {
            name: props.tag.name().to_string().into(),
            value: props.tag.value().to_string().into(),
        }
        .request(),
    );
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                {props.tag.name()}
            </th>
            <td class="px-6 py-4 pr-1">
            if let Some((_, info)) = tag_value.data().iter().flat_map(|d| d.iter()).next() {
                {&info.display}
            } else {
                {props.tag.value()}
            }
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTagsRowProps {
    pub file: RcHash,
    pub name: String,
    pub tag: Tag,
}

#[function_component]
pub fn FileTagsRow(props: &FileTagsRowProps) -> Html {
    let tag_value = use_cached(
        TagList {
            name: props.tag.name().to_string().into(),
            value: props.tag.value().to_string().into(),
        }
        .request(),
    );
    let delete = use_request(
        FileTagDelete {
            hash: props.file.clone(),
            name: Some(props.tag.name().to_string()),
            value: Some(props.tag.value().to_string()),
        }
        .request(),
    );
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
            <th scope="row" class="px-3 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                {&props.name}
            </th>
            <td class="px-3 py-4">
                if let Some((_, info)) = tag_value.data().iter().flat_map(|d| d.iter()).next() {
                    {&info.display}
                } else {
                    {props.tag.value()}
                }
            </td>
            <td class="px-3 py-4 pr-1">
                <RowDeleteButton onclick={move |_| delete.run()} />
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTagsCreateRowProps {
    pub file: RcHash,
}

#[function_component]
pub fn FileTagsCreateRow(props: &FileTagsCreateRowProps) -> Html {
    let name = use_state(String::new);
    let value = use_state(String::new);

    let tag_names = use_cached(TagNames.request());

    if let Some(tag_names) = tag_names.data() {
        if !tag_names.contains_key(&*name) {
            if let Some((first_name, _)) = tag_names.iter().find(|(_, info)| !info.system) {
                name.set(first_name.clone());
            }
        }
    }

    let tag_values = use_cached(
        TagList {
            name: Some((**name).to_string()),
            value: None::<String>,
        }
        .request(),
    );

    let create = use_request(
        FileTagCreate {
            hash: props.file.clone(),
            name: (**name).to_string(),
            value: (**value).to_string(),
        }
        .request(),
    );

    let onkeydown = {
        let create = create.clone();
        move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                create.run();
                event.prevent_default();
            }
        }
    };

    let name_onchange = {
        let name = name.clone();
        move |event: Event| {
            let target: HtmlSelectElement = event.target_dyn_into().unwrap();
            name.set(target.value());
        }
    };

    let value_onchange = {
        let value = value.clone();
        move |event: Event| {
            let target: HtmlSelectElement = event.target_dyn_into().unwrap();
            value.set(target.value());
        }
    };
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
            <th scope="row" class="px-3 py-3 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                <select id="countries" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-1 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={name_onchange}>
                {
                    tag_names.data()
                        .iter()
                        .flat_map(|v| v.iter())
                        .filter(|(_, info)| !info.system)
                        .map(|(tag, info)| html!{
                            <option selected={tag == &*name} value={tag.clone()}>
                                {&info.display}
                            </option>
                        }).collect::<Html>()
                }
                </select>
            </th>
            <td class="px-3 py-4">
                <select id="countries" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-1 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" onchange={value_onchange}>
                {
                    tag_values.data()
                        .iter()
                        .flat_map(|v| v.iter())
                        .map(|(tag, info)| html!{
                            <option selected={tag.value() == &*value} value={tag.value().to_string()}>
                                {&info.display}
                            </option>
                        }).collect::<Html>()
                }
                </select>
            </td>
            <td class="px-3 py-4 pr-1">
                <RowSubmitButton onclick={move |_| create.run()} />
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTagCreateButtonProps {
    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component]
pub fn FileTagCreateButton(props: &FileTagCreateButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let onclick = move |_| onclick.emit(());
    html! {
        <tr class="bg-white dark:bg-gray-800 dark:border-gray-700">
            <td colspan="3">
                <button class="p-3 w-full flex items-center justify-center hover:text-blue-600" {onclick}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 mx-2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                    </svg>
                    {"Create tag"}
                </button>
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTagsListProps {
    pub file: RcHash,
    #[prop_or_default]
    pub tags: Vec<Tag>,
}

#[function_component]
pub fn FileTagsList(props: &FileTagsListProps) -> Html {
    let names = use_cached(TagNames.request());
    let inputs = use_list(Vec::<Uuid>::new());
    html! {
        <div class="relative overflow-x-auto py-3">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <TagsListHeader actions=true />
                <tbody>
                {
                    props.tags.iter().cloned().map(|tag| {
                        let name = names
                            .data()
                            .and_then(|names| names.get(tag.name()))
                            .map(|info| &info.display as &str)
                            .unwrap_or(tag.name())
                            .to_string();
                        html! {
                        <FileTagsRow {tag} {name} file={props.file.clone()} />
                    }}).collect::<Html>()
                }
                <FileTagsCreateRow file={props.file.clone()} />
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CommonTagsListProps {
    #[prop_or_default]
    pub tags: Vec<Tag>,
}

#[function_component]
pub fn CommonTagsList(props: &CommonTagsListProps) -> Html {
    let tags = use_cached(QueryTagsIntersection {
        name: None,
        value: None,
        query: vec![],
    }.request());
    html! {
        <div class="relative overflow-x-auto py-3">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <TagsListHeader actions=false />
                <tbody>
                {
                    tags.data().iter().flat_map(|tags| tags.tags.iter()).cloned().map(|tag| html! {
                        <TagsListRow {tag} />
                    }).collect::<Html>()
                }
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ToggleEntryProps {
    text: String,
}

#[function_component]
pub fn ToggleEntry(props: &ToggleEntryProps) -> Html {
    html! {
        <div class="py-1 flex items-center justify-between">
            <span class="text-sm font-medium text-gray-900 dark:text-gray-300">{&props.text}</span>
            <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" value="" class="sr-only peer" checked=true />
                <div class="w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
            </label>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarHeadingProps {
    pub children: Children,
}

#[function_component]
pub fn SidebarHeading(props: &SidebarHeadingProps) -> Html {
    html! {
        <h1 class="text-lg font-bold">
            { for props.children.iter() }
        </h1>
    }
}

#[derive(Properties, PartialEq)]
pub struct QuerySidebarProps {}

#[function_component]
pub fn QuerySidebar(_props: &QuerySidebarProps) -> Html {
    html! {
        <Sidebar>
            <SidebarHeading>{"Common tags"}</SidebarHeading>
            <CommonTagsList />
            <SidebarHeading>{"Settings"}</SidebarHeading>
            <div class="py-2">
                <ToggleEntry text="Show tags" />
                <ToggleEntry text="Hive system tags" />
            </div>
        </Sidebar>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileSidebarProps {
    pub file: RcHash,
}

#[function_component]
pub fn FileSidebar(props: &FileSidebarProps) -> Html {
    let tags = use_cached(
        FileTags {
            hash: props.file.clone(),
            name: None::<String>,
            value: None::<String>,
        }
        .request(),
    );
    log::info!("RENDERING FILE SIDEBAR");
    html! {
        <Sidebar>
            <SidebarHeading>{"Tags"}</SidebarHeading>
            if let Some(tags) = tags.data() {
                <FileTagsList file={props.file.clone()} tags={(**tags).clone()} />
            } else {
                <FileTagsList file={props.file.clone()} tags={vec![]} />
            }

            <SidebarHeading>{"Settings"}</SidebarHeading>
            <div class="py-2">
                <ToggleEntry text="Show labels" />
                <ToggleEntry text="Edit labels" />
                <ToggleEntry text="Show system tags" />
            </div>

            <SidebarHeading>{"Management"}</SidebarHeading>
            <p>{"Delete file"}</p>
        </Sidebar>
    }
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub children: Children,
}

#[function_component]
pub fn Sidebar(props: &SidebarProps) -> Html {
    html! {
        <div class="bg-white md:w-96 md:h-full p-6">
        { for props.children.iter() }
        </div>
    }
}
