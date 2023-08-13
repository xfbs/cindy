use super::*;
use crate::prelude::*;
use uuid::Uuid;

#[derive(Properties, PartialEq)]
struct TagValueTableProps {
    #[prop_or_default]
    children: Children,
}

#[function_component]
fn TagValueTable(props: &TagValueTableProps) -> Html {
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
                        {"Files"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                        {"Labels"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                            <span class="sr-only">{"Action"}</span>
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
struct TagValueRowProps {
    tag: Tag,
    info: TagValueInfo,
}

#[function_component]
fn TagValueRow(props: &TagValueRowProps) -> Html {
    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                {&props.info.display}
            </th>
            <td class="px-6 py-4">
                {&props.tag}
            </td>
            <td class="px-6 py-4">
                {&props.info.files}
            </td>
            <td class="px-6 py-4">
            </td>
            <td class="px-6 py-4 text-right">
                if !props.info.system {
                    <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">
                        {"Delete"}
                    </a>
                }
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
struct TagValueCreateRowProps {
    name: String,
}

#[function_component]
fn TagValueCreateRow(props: &TagValueCreateRowProps) -> Html {
    let name = use_state(String::new);
    let slug = use_state(String::new);

    let request = use_request(TagValueCreate {
        name: props.name.clone(),
        value: (*slug).clone(),
        display: Some((*name).clone()),
    });

    let name_oninput = {
        let name = name.clone();
        move |event: InputEvent| {
            let target: HtmlInputElement = event.target_dyn_into().unwrap();
            name.set(target.value());
        }
    };

    let slug_oninput = {
        let slug = slug.clone();
        move |event: InputEvent| {
            let target: HtmlInputElement = event.target_dyn_into().unwrap();
            slug.set(target.value());
        }
    };

    let onkeydown = {
        let request = request.clone();
        move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                request.run();
            }
        }
    };

    html! {
        <tr class="group bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                <input type="text" onkeydown={onkeydown.clone()} oninput={name_oninput} class="group-hover:bg-gray-50 w-full" placeholder="Name" />
            </th>
            <td class="px-6 py-4">
                <input type="text" onkeydown={onkeydown.clone()} oninput={slug_oninput} class="group-hover:bg-gray-50 w-full" placeholder="Slug" />
            </td>
            <td class="px-6 py-4">
            </td>
            <td class="px-6 py-4">
            </td>
            <td class="px-6 py-4 text-right">
                <button onclick={move |_| request.run()} class="font-medium text-blue-600 dark:text-blue-500 hover:underline">
                    {"Create"}
                </button>
            </td>
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct SettingsTagProps {
    pub name: String,
}

#[function_component]
pub fn SettingsTag(props: &SettingsTagProps) -> Html {
    let tags = use_cached(TagList {
        name: Some(props.name.clone()),
        value: None::<String>,
    });
    html! {
        <>
            <p>{format!("Tag for {}", &props.name)}</p>
            <TagValueTable>
            if let Some(tags) = tags.data() {
                {
                    for (**tags)
                        .clone()
                        .into_iter()
                        .map(|(tag, info)|
                            html!{
                                <TagValueRow {tag} {info} />
                            })
                }
            }
            <TagValueCreateRow name={props.name.clone()} />
            </TagValueTable>
        </>
    }
}
