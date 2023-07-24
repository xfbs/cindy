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
        <nav class="bg-gray-200 border-gray-200 dark:bg-gray-900 sticky top-0 z-10 border">
            <div class="max-w-screen-4xl flex flex-wrap items-center justify-between mx-auto p-4">
                <a href="/" class="flex items-center group">
                    <img src="/cindy.svg" class="h-8 mr-3 group-hover:scale-110 transition duration-100" alt="Cindy Logo" />
                    <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">{"Cindy"}</span>
                </a>
                <div class="w-auto" id="navbar-default">
                    <ul class="font-medium flex flex-col p-0 flex-row space-x-8 mt-0">
                        <li>
                            <a href="#" class="group flex items-center text-gray-900 md:border-0 md:hover:text-blue-700 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">
                                <img src="/gear.svg" class="h-8 mr-3 group-hover:rotate-12 transition duration-100" alt="Settings icon" />
                                <span class="hidden md:block p-0">
                                    {"Settings"}
                                </span>
                            </a>
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
                <thead class="text-xs text-gray-700 uppercase bg-gray-200 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="px-6 py-3 pl-1">
                        {"Name"}
                        </th>
                        <th scope="col" class="px-6 py-3">
                        {"Value"}
                        </th>
                        <th scope="col" class="px-6 py-3 pr-1">
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                        {"media"}
                        </th>
                        <td class="px-6 py-4">
                        {"image"}
                        </td>
                        <td class="px-6 py-4 pr-1">
                        {"X"}
                        </td>
                    </tr>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white pl-1">
                        {"resolution"}
                        </th>
                        <td class="px-6 py-4">
                        {"hd"}
                        </td>
                        <td class="px-6 py-4 pr-1">
                        {"X"}
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
        <div class="bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 hover:border-black shadow hover:shadow-lg relative">
            <a href="#">
                <img class="rounded-lg" src="https://images.unsplash.com/photo-1488372759477-a7f4aa078cb6?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1740&q=80" alt="" />
            </a>
            <div class="absolute bottom-0 left-0 p-2 min-w-full">
                <div class="flex flex-wrap">
                    <span class="bg-blue-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1">{"video"}</span>
                    <span class="bg-red-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1">{"image"}</span>
                    <span class="bg-blue-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1">{"hd"}</span>
                    <span class="bg-red-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1">{"test.jpg"}</span>
                    <span class="bg-blue-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1">{"jpg"}</span>
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn Files() -> Html {
    html! {
        <div>
            <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 gap-3 p-3">
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
        <div class="bg-white w-full md:w-96 md:min-h-screen p-6">
            <h1 class="text-lg font-bold" >{"Filters"}</h1>
            <TagsList />
            <h1 class="text-lg font-bold" >{"Common Tags"}</h1>
            <TagsList />
            <TagsList />
            <TagsList />
            <TagsList />
            <TagsList />
            <TagsList />
            <TagsList />
            <TagsList />
            <TagsList />
            <TagsList />
        </div>
    }
}

#[function_component]
fn FileSidebar() -> Html {
    html! {
        <div class="bg-white md:w-96 md:min-h-screen p-6">
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
        <div class="min-h-screen md:relative my-auto">
            <div class="md:min-w-200 md:fixed top-16 right-0 h-full md:max-h-screen md:overflow-scroll md:pb-16">
                <QuerySidebar />
            </div>
            <div class="md:mr-96">
                <Files />
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
