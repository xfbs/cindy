use crate::prelude::*;
use uuid::Uuid;

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
                <th scope="col" class="px-6 py-3 pl-1">
                    {"Name"}
                </th>
                if props.actions {
                    <th scope="col" class="px-6 py-3">
                        {"Value"}
                    </th>
                    <th scope="col" class="px-6 py-3 pr-1">
                    </th>
                } else {
                    <th scope="col" class="px-6 py-3 pr-1">
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
pub struct TagsListRowProps {
    pub tag: Tag,
}

#[function_component]
pub fn TagsListRow(props: &TagsListRowProps) -> Html {
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                {props.tag.name()}
            </th>
            <td class="px-6 py-4 pr-1">
                {props.tag.value()}
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTagsRowProps {
    pub file: RcHash,
    pub tag: Tag,
}

#[function_component]
pub fn FileTagsRow(props: &FileTagsRowProps) -> Html {
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                {props.tag.name()}
            </th>
            <td class="px-6 py-4">
                {props.tag.value()}
            </td>
            <td class="px-6 py-4 pr-1">
                <RowDeleteButton />
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTagsCreateRowProps {
    pub file: RcHash,
    pub id: Uuid,
    #[prop_or_default]
    pub ondelete: Callback<()>,
}

#[function_component]
pub fn FileTagsCreateRow(props: &FileTagsCreateRowProps) -> Html {
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
            <th scope="row" class="px-6 py-3 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                <select id="countries" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-1 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                    <option value="media">{"media"}</option>
                    <option value="format">{"format"}</option>
                    <option value="resolution">{"resolution"}</option>
                    <option value="depth">{"depth"}</option>
                </select>
            </th>
            <td class="px-6 py-4">
                <input onsubmit={move |_| {}} />
            </td>
            <td class="px-6 py-4 pr-1">
                <RowDeleteButton onclick={props.ondelete.clone()} />
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
                <button class="bg-blue-200 p-3 w-full flex items-center justify-center" {onclick}>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
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
    let inputs = use_list(Vec::<Uuid>::new());
    html! {
        <div class="relative overflow-x-auto py-3">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <TagsListHeader actions=true />
                <tbody>
                {
                    props.tags.iter().cloned().map(|tag| html! {
                        <FileTagsRow {tag} file={props.file.clone()} />
                    }).collect::<Html>()
                }
                {
                    inputs.current().iter().copied().map(|id| {
                        let inputs = inputs.clone();
                        let ondelete = move |_| inputs.retain(|i| i != &id);
                        html! {
                        <FileTagsCreateRow {id} file={props.file.clone()} {ondelete}/>
                    }}).collect::<Html>()
                }
                <FileTagCreateButton onclick={move |_| inputs.push(Uuid::new_v4())} />
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
    html! {
        <div class="relative overflow-x-auto py-3">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <TagsListHeader actions=false />
                <tbody>
                {
                    props.tags.iter().cloned().map(|tag| html! {
                        <TagsListRow {tag} />
                    }).collect::<Html>()
                }
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct QuerySidebarProps {}

#[function_component]
pub fn QuerySidebar(props: &QuerySidebarProps) -> Html {
    html! {
        <div class="bg-white w-full md:w-96 md:min-h-screen p-6">
            <h1 class="text-lg font-bold" >{"Common Tags"}</h1>
            <CommonTagsList />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileSidebarProps {
    pub file: RcHash,
}

#[function_component]
pub fn FileSidebar(props: &FileSidebarProps) -> Html {
    let tags = use_get(FileTags {
        hash: Cow::Owned((&props.file).into()),
        name: None,
        value: None,
    });
    html! {
        <div class="bg-white md:w-96 md:min-h-screen p-6">
            <h1 class="text-lg font-bold" >{"Tags"}</h1>
            <FileTagsList file={props.file.clone()} tags={tags.data.clone().unwrap_or_default()} />

            <h1 class="text-lg font-bold" >{"File Labels"}</h1>
            <FileTagsList file={props.file.clone()} />
        </div>
    }
}
