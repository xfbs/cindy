use super::*;

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

#[derive(Properties, PartialEq)]
pub struct FileCardLoaderProps {
    pub hash: BoxHash,
}

#[function_component]
pub fn FileCardLoader(props: &FileCardLoaderProps) -> Html {
    let tags = use_get(FileTags {
        hash: Cow::Owned(props.hash.clone()),
        name: None,
        value: None,
    });
    html! {
        <FileCard hash={props.hash.clone()} tags={tags.data.clone().unwrap_or_default()} />
    }
}

#[derive(Properties, PartialEq)]
pub struct FileCardProps {
    pub hash: BoxHash,
    pub tags: Vec<Tag>,
}

#[function_component]
pub fn FileCard(props: &FileCardProps) -> Html {
    let content = FileContent {
        hash: Cow::Borrowed(&props.hash),
    };
    html! {
        <div class="bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 hover:border-black shadow hover:shadow-lg relative">
            <a href="#">
                <img class="rounded-lg" src={content.uri()} alt="" />
            </a>
            <div class="absolute bottom-0 left-0 p-2 min-w-full">
                <div class="flex flex-wrap">
                    {
                        props.tags.iter().map(|tag| html!{
                            <span class="bg-blue-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1">{tag.name()}{":"}{tag.value()}</span>
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FilesGridLoaderProps {}

#[function_component]
pub fn FilesGridLoader(props: &FilesGridLoaderProps) -> Html {
    let data = use_get(FileQuery {
        query: vec![].into(),
    });

    html! {
        if let Some(data) = &data.data {
            <FilesGrid files={data.clone()} />
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FilesGridProps {
    #[prop_or_default]
    pub files: Vec<BoxHash>,
}

#[function_component]
pub fn FilesGrid(props: &FilesGridProps) -> Html {
    html! {
        <div>
            <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 3xl:grid-cols-5 gap-3 p-3">
            {
                props.files.iter().cloned().map(|hash| {
                    html! {
                        <FileCardLoader {hash} />
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
