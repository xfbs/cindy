use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FilterProps {
    pub filter: TagPredicate<'static>,
    #[prop_or_default]
    pub ondelete: Callback<()>,
}

#[function_component]
fn Filter(props: &FilterProps) -> Html {
    let ondelete = props.ondelete.clone();
    let onclick = move |_| {
        ondelete.emit(());
    };
    html! {
        <span class="ml-2 px-1 bg-red-400 rounded block flex items-center whitespace-nowrap">{props.filter.to_string()}
            <svg class="w-4 h-4 bg-red-500 ml-1 rounded hover:border hover:border-red-600" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg" {onclick}>
                <path fill-rule="evenodd" clip-rule="evenodd" d="M6.79289 7.49998L4.14645 4.85353L4.85355 4.14642L7.5 6.79287L10.1464 4.14642L10.8536 4.85353L8.20711 7.49998L10.8536 10.1464L10.1464 10.8535L7.5 8.20708L4.85355 10.8535L4.14645 10.1464L6.79289 7.49998Z" fill="#000000"/>
            </svg>
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct SearchProps {
    #[prop_or_default]
    pub onchange: Callback<Vec<TagPredicate<'static>>>,
}

#[function_component]
pub fn Search(props: &SearchProps) -> Html {
    // current active tag filters
    let filters = use_state_eq(|| Vec::<TagPredicate<'static>>::new());

    // current search bar input
    let input = use_state_eq(|| String::new());

    // run onsubmit callback, this will try to parse the filter from the search
    // bar and add it to the list of filters
    let onsubmit = {
        let filters = filters.clone();
        let onchange = props.onchange.clone();
        let input = input.clone();
        move |_| {
            if let Ok(filter) = input.parse() {
                let mut current = (*filters).clone();
                current.push(filter);
                filters.set(current.clone());
                input.set(String::new());
                onchange.emit(current);
            }
        }
    };

    // capture changes in search bar, this is synced two-ways.
    let oninput = {
        let input = input.clone();
        move |event: InputEvent| {
            let target: HtmlInputElement = event.target_dyn_into().unwrap();
            input.set(target.value());
        }
    };

    html! {
        <form {onsubmit} action="#abc">
            <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">{"Search"}</label>
            <div class="flex items-center border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                <div class="pl-3 pointer-events-none">
                    <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
                    </svg>
                </div>
                {
                    filters
                        .iter()
                        .enumerate()
                        .map(|(index, predicate)| {
                            // callback to delete this predicate
                            let filters = filters.clone();
                            let onchange = props.onchange.clone();
                            let ondelete = move |()| {
                                let mut current = (*filters).clone();
                                current.remove(index);
                                filters.set(current.clone());
                                onchange.emit(current);
                            };

                            html!{<Filter filter={predicate.clone()} {ondelete} />}
                        })
                        .collect::<Html>()
                }
                <input type="search" id="default-search" class="block w-full p-3 text-sm bg-gray-50 text-gray-900 dark:bg-gray-700" placeholder="Add tags to filter by..." required=true {oninput} value={input.to_string()} />
                <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 mx-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{"Search"}</button>
            </div>
        </form>
    }
}
