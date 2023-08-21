use crate::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct ToggleProps {
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

#[function_component]
pub fn Toggle(props: &ToggleProps) -> Html {
    let onchange = props.onchange.clone();
    let oninput = move |event: InputEvent| {
        let element: HtmlInputElement = event.target_dyn_into().unwrap();
        onchange.emit(element.checked());
    };
    html! {
        <label class="relative inline-flex items-center cursor-pointer">
            <input type="checkbox" value="" class="sr-only peer" checked={props.checked} {oninput} disabled={props.disabled} />
            <div class="w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
        </label>
    }
}
