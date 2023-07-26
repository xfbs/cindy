use super::*;

#[function_component]
fn Logo() -> Html {
    html! {
        <a href="/" class="flex items-center group">
            <img src="/cindy.svg" class="h-8 mr-3 group-hover:scale-125 transition duration-300" alt="Cindy Logo" />
            <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"Cindy"}</span>
        </a>
    }
}

#[function_component]
fn Settings() -> Html {
    html! {
        <div class="w-auto" id="navbar-default">
            <ul class="font-medium flex flex-col p-0 flex-row space-x-8 mt-0">
                <li>
                    <a href="#" class="group flex items-center text-gray-900 md:border-0 md:hover:text-blue-700 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">
                        <img src="/gear.svg" class="h-8 mr-3 group-hover:rotate-180 transition duration-500 group-hover:scale-125" alt="Settings icon" />
                        <span class="hidden md:block p-0">
                            {"Settings"}
                        </span>
                    </a>
                </li>
            </ul>
        </div>
    }
}

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav class="bg-gray-200 border-gray-200 dark:bg-gray-900 sticky top-0 z-10 border">
            <div class="max-w-screen-4xl flex flex-wrap items-center justify-between mx-auto p-4">
                <Logo />
                <Settings />
            </div>
        </nav>
    }
}
