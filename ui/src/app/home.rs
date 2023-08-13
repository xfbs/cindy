use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawQuery {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
}

impl RawQuery {
    fn decode(self) -> Query {
        Query {
            sort: self.sort,
            group: self.group,
            query: match self.query.as_deref().map(serde_json::from_str) {
                Some(Ok(result)) => result,
                Some(Err(error)) => {
                    log::error!("Failed to parse query: {error}");
                    Default::default()
                }
                None => Default::default(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Query {
    #[serde(default)]
    pub query: Rc<Vec<Rc<TagPredicate<'static>>>>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
}

impl Query {
    fn encode(self) -> RawQuery {
        RawQuery {
            sort: self.sort,
            group: self.group,
            query: match self.query.len() {
                0 => None,
                _ => Some(serde_json::to_string(&self.query).unwrap()),
            },
        }
    }
}

#[function_component]
pub fn HomeView() -> Html {
    let location = use_location().unwrap();
    let navigator = use_navigator().unwrap();
    let query: RawQuery = location.query().unwrap();
    let query = query.decode();

    let onchange = {
        let query = query.clone();
        move |new: Vec<Rc<TagPredicate<'static>>>| {
            let mut query = query.clone();
            query.query = Rc::new(new);
            if let Err(error) = navigator.replace_with_query(&Route::Home, &query.encode()) {
                log::error!("{error:?}");
            }
        }
    };

    html! {
        <div>
            <NavBar>
                <Search {onchange} query={query.query.clone()} />
            </NavBar>
            <SidebarLayout>
                <SidebarLayoutSidebar>
                    <QuerySidebar query={query.query.clone()} />
                </SidebarLayoutSidebar>
                <SidebarLayoutContent>
                    <FilesGrid query={query.query.clone()} />
                </SidebarLayoutContent>
            </SidebarLayout>
        </div>
    }
}

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
fn FileCardPending(props: &FileCardLoaderProps) -> Html {
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
struct FileCardLoaderProps {
    hash: BoxHash,
}

#[function_component]
fn FileCardLoader(props: &FileCardLoaderProps) -> Html {
    let tags = use_cached(FileTags {
        hash: props.hash.clone(),
        name: None::<String>,
        value: None::<String>,
    });
    let tags = tags
        .data()
        .map(|tags| tags[..].to_vec())
        .unwrap_or_default();
    html! {
        <FileCard hash={props.hash.clone()} {tags} />
    }
}

#[derive(Properties, PartialEq)]
struct FileTagProps {
    tag: Tag,
}

#[function_component]
fn FileTag(props: &FileTagProps) -> Html {
    html! {
        <button class="bg-blue-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1 pointer-events-auto">
            {props.tag.name()}{":"}{props.tag.value()}
        </button>
    }
}

#[derive(Properties, PartialEq)]
struct FileCardProps {
    hash: BoxHash,
    tags: Vec<Tag>,
}

#[function_component]
fn FileCard(props: &FileCardProps) -> Html {
    let content = FileContent {
        hash: props.hash.clone(),
    };
    html! {
        <>
        <Link<Route> to={Route::file(props.hash.clone().into())}>
            <img class="rounded-lg" src={content.uri()} alt="" />
        </Link<Route>>
        <div class="absolute bottom-0 left-0 p-2 min-w-full pointer-events-none">
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
struct FilesGridProps {
    #[prop_or_default]
    query: Rc<Vec<Rc<TagPredicate<'static>>>>,
}

#[function_component]
fn FilesGrid(props: &FilesGridProps) -> Html {
    let files = use_cached(QueryFiles {
        query: props
            .query
            .iter()
            .map(|pred| (**pred).clone())
            .collect::<Vec<TagPredicate<'static>>>()
            .into(),
    });
    html! {
        if let Some(files) = files.data() {
            <Grid>
            {
                files.iter().cloned().map(|hash| {
                    html! {
                        <FileCardPending {hash} />
                    }
                }).collect::<Html>()
            }
            </Grid>
        } else {
            <div>{"Loading"}</div>
        }
    }
}
