use crate::prelude::*;

#[function_component]
fn FileCardLoading() -> Html {
    html! {
        <div class="min-h-[16rem] m-auto object-center">
            <svg aria-hidden="true" class="w-8 h-8 mr-2 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
            </svg>
            <span class="sr-only">{"Loading..."}</span>
        </div>
    }
}

#[function_component]
pub fn FileCardPending(props: &FileCardLoaderProps) -> Html {
    let node = use_node_ref();
    let visible = use_visible(node.clone(), true);
    html! {
        <div ref={node} class="bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 hover:border-black shadow hover:shadow-lg relative">
            if visible {
                <FileCardLoader hash={props.hash.clone()} />
            } else {
                <FileCardLoading />
            }
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
        hash: props.hash.clone(),
        name: None::<String>,
        value: None::<String>,
    });
    html! {
        <FileCard hash={props.hash.clone()} tags={tags.data.clone().unwrap_or_default()} />
    }
}

#[derive(Properties, PartialEq)]
pub struct FileTagProps {
    pub tag: Tag,
}

#[function_component]
pub fn FileTag(props: &FileTagProps) -> Html {
    html! {
        <span class="bg-blue-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1">
            {props.tag.name()}{":"}{props.tag.value()}
        </span>
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
        hash: props.hash.clone(),
    };
    html! {
        <>
        <Link<Route> to={Route::file(props.hash.clone().into())}>
            <img class="rounded-lg" src={content.uri()} alt="" />
        </Link<Route>>
        <div class="absolute bottom-0 left-0 p-2 min-w-full">
            <div class="flex flex-wrap">
                {
                    props.tags.iter().cloned().map(|tag| html!{
                        <FileTag {tag} />
                    }).collect::<Html>()
                }
            </div>
        </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct FilesGridLoaderProps {
    #[prop_or_default]
    pub query: Vec<TagPredicate<'static>>,
}

#[function_component]
pub fn FilesGridLoader(props: &FilesGridLoaderProps) -> Html {
    let data = use_get(FileQuery {
        query: props.query.clone().into(),
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
                        <FileCardPending {hash} />
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
