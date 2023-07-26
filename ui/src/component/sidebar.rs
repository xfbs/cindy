use super::*;

#[function_component]
pub fn QuerySidebar() -> Html {
    html! {
        <div class="bg-white w-full md:w-96 md:min-h-screen p-6">
            <h1 class="text-lg font-bold" >{"Common Tags"}</h1>
            <TagsList />
            <TagsList />
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

#[function_component]
pub fn Content() -> Html {
    html! {
        <div class="min-h-screen md:relative my-auto">
            <div class="md:min-w-200 md:fixed top-16 right-0 h-full md:max-h-screen md:overflow-scroll md:pb-16">
                <QuerySidebar />
            </div>
            <div class="md:mr-96">
                <FilesGridLoader />
            </div>
        </div>
    }
}
