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
fn Filter() -> Html {
    html! {
        <span class="ml-2 px-1 bg-red-400 rounded block flex items-center">{"name:abc"}
            <svg class="w-4 h-4 bg-red-500 ml-1 rounded hover:border hover:border-red-600" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M6.79289 7.49998L4.14645 4.85353L4.85355 4.14642L7.5 6.79287L10.1464 4.14642L10.8536 4.85353L8.20711 7.49998L10.8536 10.1464L10.1464 10.8535L7.5 8.20708L4.85355 10.8535L4.14645 10.1464L6.79289 7.49998Z" fill="#000000"/>
            </svg>
        </span>
    }
}

#[function_component]
fn Search() -> Html {
    html! {
        <form>
            <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">{"Search"}</label>
            <div class="flex items-center border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                <div class="pl-3 pointer-events-none">
                    <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
                    </svg>
                </div>
                <Filter />
                <Filter />
                <Filter />
                <input type="search" id="default-search" class="block w-full p-3 text-sm bg-gray-50 text-gray-900 dark:bg-gray-700" placeholder="Add tags to filter by..." required=true />
                <button type="submit" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 mx-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">{"Search"}</button>
            </div>
        </form>
    }
}

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav class="bg-gray-200 border-gray-200 dark:bg-gray-900 sticky top-0 z-10 border">
            <div class="max-w-screen-4xl flex flex-wrap items-center justify-between mx-auto p-4 gap-y-4 gap-x-3 md:flex-nowrap">
                <Logo />
                <div class="md:order-last">
                    <Settings />
                </div>
                <div class="grow">
                    <Search />
                </div>
            </div>
        </nav>
    }
}
