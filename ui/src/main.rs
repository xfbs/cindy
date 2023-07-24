use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/query")]
    Query,
    #[at("/file")]
    File,
}

#[function_component]
fn NavBar() -> Html {
    html! {
        <nav class="bg-white border-gray-200 bg-white dark:bg-gray-900 sticky top-0 z-10 border">
            <div class="max-w-screen-4xl flex flex-wrap items-center justify-between mx-auto p-4">
                <a href="/" class="flex items-center">
                    <img src="/cindy.svg" class="h-8 mr-3" alt="Flowbite Logo" />
                    <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"Cindy"}</span>
                </a>
                <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                    <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                        <li>
                            <a href="#" class="block py-2 pl-3 pr-4 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">{"Settings"}</a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}

#[function_component]
fn TagsList() -> Html {
    html! {
        <div class="relative overflow-x-auto py-3">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="px-6 py-3">
                        {"Name"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                        {"Value"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                        {"Action"}
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                        {"media"}
                        </th>
                        <td class="px-6 py-4">
                        {"image"}
                        </td>
                        <td class="px-6 py-4">
                        {"delete"}
                        </td>
                    </tr>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                        {"resolution"}
                        </th>
                        <td class="px-6 py-4">
                        {"hd"}
                        </td>
                        <td class="px-6 py-4">
                        {"delete"}
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[function_component]
fn FileCard() -> Html {
    html! {
        <div class="max-w-sm bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 hover:border-black shadow hover:shadow-lg relative">
            <a href="#">
                <img class="rounded-lg" src="https://images.unsplash.com/photo-1488372759477-a7f4aa078cb6?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1740&q=80" alt="" />
            </a>
            <div class="absolute bottom-0 left-0 p-2 w-max">
                <span class="block bg-white rounded p-1">{"media:image"}</span>
                <span class="block bg-white rounded p-1">{"media:image"}</span>
            </div>
        </div>
    }
}

#[function_component]
fn Files() -> Html {
    html! {
        <div>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3 p-3">
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
                <FileCard />
            </div>
        </div>
    }
}

#[function_component]
fn QuerySidebar() -> Html {
    html! {
        <div class="bg-white w-96 min-h-screen p-6">
            <h1 class="text-lg font-bold" >{"Filters"}</h1>
            <TagsList />
            <h1 class="text-lg font-bold" >{"Common Tags"}</h1>
            <TagsList />
        </div>
    }
}

#[function_component]
fn FileSidebar() -> Html {
    html! {
        <div class="bg-white w-96 min-h-screen p-6">
            <h1 class="text-lg font-bold" >{"Filters"}</h1>
            <TagsList />
            <h1 class="text-lg font-bold" >{"Tags"}</h1>
            <TagsList />
            <h1 class="text-lg font-bold" >{"File Labels"}</h1>
            <TagsList />
        </div>
    }
}

#[function_component]
fn Content() -> Html {
    html! {
        <div class="flex min-h-screen">
            <div class="flex-grow">
            <Files />
            </div>
            <div class="min-w-200">
            <QuerySidebar />
            </div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
        <NavBar />
        <Content />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
