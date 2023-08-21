use crate::prelude::*;
use uuid::Uuid;
use web_sys::HtmlSelectElement;

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
            <Cross />
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
            <Cross />
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TagsListRowProps {
    pub tag: Tag,
    #[prop_or_default]
    pub query: Rc<Vec<Rc<TagPredicate<'static>>>>,
}

#[function_component]
pub fn TagsListRow(props: &TagsListRowProps) -> Html {
    let tag_value = use_cached(TagList {
        name: props.tag.name().to_string().into(),
        value: props.tag.value().to_string().into(),
    });
    let tag_names = use_cached(TagNames);
    let tag_name = tag_names
        .data()
        .and_then(|names| names.get(props.tag.name()));
    let delete = use_request(QueryTagRemove {
        query: props.query.iter().map(|pred| (**pred).clone()).collect(),
        name: Some(props.tag.name().to_string()),
        value: Some(props.tag.value().to_string()),
    });

    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
            <th scope="row" class="px-3 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                if let Some(info) = &tag_name {
                    {&info.display}
                } else {
                    {props.tag.name()}
                }
            </th>
            <td class="px-3 py-4">
            if let Some((_, info)) = tag_value.data().iter().flat_map(|d| d.iter()).next() {
                {&info.display}
            } else {
                {props.tag.value()}
            }
            </td>
            <td class="px-3 py-4 pr-1">
                if tag_name.map(|info| !info.system).unwrap_or(false) {
                    <RowDeleteButton onclick={move |_| delete.run()} />
                }
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTagsRowProps {
    pub file: RcHash,
    pub info: Option<TagNameInfo>,
    pub tag: Tag,
}

#[function_component]
pub fn FileTagsRow(props: &FileTagsRowProps) -> Html {
    let tag_value = use_cached(TagList {
        name: props.tag.name().to_string().into(),
        value: props.tag.value().to_string().into(),
    });
    let delete = use_request(FileTagDelete {
        hash: props.file.clone(),
        name: Some(props.tag.name().to_string()),
        value: Some(props.tag.value().to_string()),
    });
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
            <th scope="row" class="px-3 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                if let Some(info) = &props.info {
                    {&info.display}
                } else {
                    {props.tag.name()}
                }
            </th>
            <td class="px-3 py-4">
                if let Some((_, info)) = tag_value.data().iter().flat_map(|d| d.iter()).next() {
                    {&info.display}
                } else {
                    {props.tag.value()}
                }
            </td>
            <td class="px-3 py-4 pr-1">
                if props.info.as_ref().map(|info| !info.system).unwrap_or(false) {
                    <RowDeleteButton onclick={move |_| delete.run()} />
                }
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

    let tag_names = use_cached(TagNames);

    if let Some(tag_names) = tag_names.data() {
        if !tag_names.contains_key(&*name) {
            if let Some((first_name, _)) = tag_names.iter().find(|(_, info)| !info.system) {
                name.set(first_name.clone());
            }
        }
    }

    let tag_values = use_cached(TagList {
        name: Some((**name).to_string()),
        value: None::<String>,
    });

    let create = use_request(FileTagCreate {
        hash: props.file.clone(),
        name: (**name).to_string(),
        value: (**value).to_string(),
    });

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
pub struct FileTagsListProps {
    pub file: RcHash,
    #[prop_or_default]
    pub tags: Vec<Tag>,
}

#[function_component]
pub fn FileTagsList(props: &FileTagsListProps) -> Html {
    let names = use_cached(TagNames);
    html! {
        <div class="relative overflow-x-auto py-3">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <TagsListHeader actions=true />
                <tbody>
                {
                    props.tags.iter().cloned().map(|tag| {
                        let info = names
                            .data()
                            .and_then(|names| names.get(tag.name()))
                            .cloned();
                        html! {
                        <FileTagsRow {tag} {info} file={props.file.clone()} />
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
    pub query: Rc<Vec<Rc<TagPredicate<'static>>>>,
}

#[function_component]
pub fn CommonTagsList(props: &CommonTagsListProps) -> Html {
    let tags = use_cached(QueryTags {
        name: None,
        value: None,
        query: props.query.iter().map(|pred| (**pred).clone()).collect(),
        mode: QueryTagsMode::Intersection,
    });
    html! {
        <div class="relative overflow-x-auto py-3">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <TagsListHeader actions=true />
                <tbody>
                {
                    tags.data().iter().flat_map(|tags| tags.iter()).cloned().map(|tag| html! {
                        <TagsListRow {tag} query={props.query.clone()} />
                    }).collect::<Html>()
                }
                <CommonTagsCreateRow query={props.query.clone()} />
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CommonTagsCreateRowProps {
    #[prop_or_default]
    pub query: Rc<Vec<Rc<TagPredicate<'static>>>>,
}

#[function_component]
pub fn CommonTagsCreateRow(props: &CommonTagsCreateRowProps) -> Html {
    let name = use_state(String::new);
    let value = use_state(String::new);

    let tag_names = use_cached(TagNames);

    if let Some(tag_names) = tag_names.data() {
        if !tag_names.contains_key(&*name) {
            if let Some((first_name, _)) = tag_names.iter().find(|(_, info)| !info.system) {
                name.set(first_name.clone());
            }
        }
    }

    let tag_values = use_cached(TagList {
        name: Some((**name).to_string()),
        value: None::<String>,
    });

    let create = use_request(QueryTagCreate {
        query: props.query.iter().map(|pred| (**pred).clone()).collect(),
        name: (**name).to_string(),
        value: (**value).to_string(),
    });

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
pub struct ToggleEntryProps {
    text: String,
}

#[function_component]
pub fn ToggleEntry(props: &ToggleEntryProps) -> Html {
    html! {
        <div class="py-1 flex items-center justify-between">
            <span class="text-sm font-medium text-gray-900 dark:text-gray-300">{&props.text}</span>
            <Toggle checked=true />
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

#[function_component]
pub fn QuerySidebar() -> Html {
    let query = use_query_state().unwrap();
    html! {
        <Sidebar>
            <SidebarHeading>{"Common tags"}</SidebarHeading>
            <CommonTagsList query={query.query.query.clone()} />
            <SidebarHeading>{"Settings"}</SidebarHeading>
            <div class="py-2">
                <ToggleEntry text="Show tags" />
                <ToggleEntry text="Show system tags" />
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
    let tags = use_cached(FileTags {
        hash: props.file.clone(),
        name: None::<String>,
        value: None::<String>,
    });
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
