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
                        {"Display"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                        {"Name"}
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
    let name = use_state(|| None::<String>);
    let display = use_state(|| None::<String>);

    let save = use_request(
        TagNameEdit {
            name: props.name.clone(),
            name_new: (*name).clone(),
            display_new: (*display).clone(),
        }
        .request(),
    );

    let onclick_name = {
        let name = name.clone();
        let prop_name = props.name.clone();
        let system = props.info.system;
        move |_| {
            if name.is_none() && !system {
                name.set(Some(prop_name.clone()));
            }
        }
    };

    let onclick_display = {
        let display = display.clone();
        let prop_name = props.info.display.clone();
        move |_| {
            if display.is_none() {
                display.set(Some(prop_name.clone()));
            }
        }
    };

    let oninput_name = {
        let name = name.clone();
        move |event: InputEvent| {
            let target: HtmlInputElement = event.target_dyn_into().unwrap();
            name.set(Some(target.value()));
        }
    };

    let oninput_display = {
        let display = display.clone();
        move |event: InputEvent| {
            let target: HtmlInputElement = event.target_dyn_into().unwrap();
            display.set(Some(target.value()));
        }
    };

    let onkeydown = {
        let save = save.clone();
        let name = name.clone();
        let display = display.clone();
        move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                save.run();
            }
            if event.key() == "Escape" {
                name.set(None);
                display.set(None);
            }
        }
    };

    html! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white" onclick={onclick_display}>
                if let Some(display) = &*display {
                    <input type="text" oninput={oninput_display} class="group-hover:bg-gray-50 w-full" placeholder="Name" value={display.clone()} onkeydown={onkeydown.clone()} />
                } else {
                    {&props.info.display}
                }
            </th>
            <td class="px-6 py-4" onclick={onclick_name}>
                if let Some(name) = &*name {
                    <input type="text" oninput={oninput_name} class="group-hover:bg-gray-50 w-full" placeholder="Name" value={name.clone()} onkeydown={onkeydown.clone()} />
                } else {
                    {&props.name}
                }
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
                if name.is_some() || display.is_some() {
                    <button onclick={move |_| save.run()} class="font-medium text-blue-600 dark:text-blue-500 hover:underline">
                        {"Save"}
                    </button>
                } else {
                    if !props.info.system {
                        <Link<SettingsRoute> to={SettingsRoute::tag(&props.name)} classes="font-medium text-blue-600 dark:text-blue-500 hover:underline">
                            {"Edit"}
                        </Link<SettingsRoute>>
                    }
                }
            </td>
        </tr>
    }
}

#[function_component]
pub fn SettingsTags() -> Html {
    let tags = use_cached(TagNames.request());
    html! {
        <>
            <SidebarHeading>{"Tags"}</SidebarHeading>
            <TagNameTable>
                {
                    for tags
                        .data()
                        .iter()
                        .flat_map(|tags| (***tags).clone().into_iter())
                        .map(|(name, info)|
                            html!{
                                <TagNameRow {name} {info} />
                            })
                }
            <TagNameCreateRow />
            </TagNameTable>
        </>
    }
}

#[function_component]
fn TagNameCreateRow() -> Html {
    let name = use_state(String::new);
    let slug = use_state(String::new);

    let request = use_request(
        TagNameCreate {
            name: (*slug).clone(),
            display: Some((*name).clone()),
        }
        .request(),
    );

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
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                <input type="text" onkeydown={onkeydown.clone()} oninput={name_oninput} class="group-hover:bg-gray-50 w-full" placeholder="Display" />
            </th>
            <td class="px-6 py-4">
                <input type="text" onkeydown={onkeydown.clone()} oninput={slug_oninput} class="group-hover:bg-gray-50 w-full" placeholder="Name" />
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
