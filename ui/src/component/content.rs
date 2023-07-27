use crate::prelude::*;

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
    let node = use_node_ref();
    let visible = use_visible(node.clone());
    html! {
        <div ref={node} class="bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 hover:border-black shadow hover:shadow-lg relative min-h-[16rem]">
            if visible {
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
            }
        </div>
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
                        <FileCardLoader {hash} />
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
