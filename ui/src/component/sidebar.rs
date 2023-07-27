use crate::prelude::*;

#[function_component]
pub fn TagsList() -> Html {
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
pub fn QuerySidebar() -> Html {
    html! {
        <div class="bg-white w-full md:w-96 md:min-h-screen p-6">
            <h1 class="text-lg font-bold" >{"Common Tags"}</h1>
            <TagsList />
        </div>
    }
}

#[function_component]
pub fn FileSidebar() -> Html {
    html! {
        <div class="bg-white md:w-96 md:min-h-screen p-6">
            <h1 class="text-lg font-bold" >{"Tags"}</h1>
            <TagsList />

            <h1 class="text-lg font-bold" >{"File Labels"}</h1>
            <TagsList />
        </div>
    }
}
