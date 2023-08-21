use crate::prelude::*;

#[function_component]
pub fn HomeView() -> Html {
    html! {
        <div>
            <QueryStateProvider>
                <NavBar>
                    <Search />
                </NavBar>
                <SidebarLayout>
                    <SidebarLayoutSidebar>
                        <QuerySidebar />
                    </SidebarLayoutSidebar>
                    <SidebarLayoutContent>
                        <FilesGrid />
                    </SidebarLayoutContent>
                </SidebarLayout>
            </QueryStateProvider>
        </div>
    }
}

#[function_component]
fn FileCardLoading() -> Html {
    html! {
        <div class="min-h-[16rem] m-auto object-center">
            <Spinner />
            <span class="sr-only">{"Loading..."}</span>
        </div>
    }
}

#[function_component]
fn FileCardPending(props: &FileCardLoaderProps) -> Html {
    html! {
        <div class="bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 hover:border-black shadow hover:shadow-lg relative">
            <FileCardLoader hash={props.hash.clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct FileCardVisibilityProps {
    hash: BoxHash,
    #[prop_or_default]
    callback: Callback<()>,
}

#[function_component]
fn FileCardVisibility(props: &FileCardVisibilityProps) -> Html {
    let node = use_node_ref();
    let visible_state = use_state_eq(|| false);
    let visible = use_visible(node.clone(), false);

    if visible && !*visible_state {
        props.callback.emit(());
    }

    if *visible_state != visible {
        visible_state.set(visible);
    }

    html! {
        <div ref={node} class="bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 hover:border-black shadow hover:shadow-lg relative">
            <FileCardLoader hash={props.hash.clone()} />
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
    let query_state = use_query_state().unwrap();
    let tag = props.tag.clone();
    let onclick = move |_| {
        query_state.predicate_append(tag.clone().into_filter().exists());
    };
    html! {
        <button class="bg-blue-200 rounded opacity-50 hover:opacity-80 cursor-default transition duration-100 m-1 p-1 pointer-events-auto" {onclick} >
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
    let query = use_query_state().unwrap();
    let content = FileContent {
        hash: props.hash.clone(),
    };
    html! {
        <>
        <Link to={Route::file(props.hash.clone().into())} query={Some(query.raw.clone())}>
            <img class="rounded-lg aspect-square w-full" src={content.uri()} alt="" />
        </Link>
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

#[function_component]
fn FilesGrid() -> Html {
    let query = use_query_state().unwrap();

    let files = use_cached(QueryFiles {
        query: query
            .query
            .query
            .iter()
            .map(|pred| (**pred).clone())
            .collect::<Vec<TagPredicate<'static>>>()
            .into(),
    });

    // count, set to initial
    let trailing = 32usize;
    let initial = 64usize;
    let count = use_state(|| initial);

    log::info!("Count is {}", *count);

    let callback = {
        let files = files.data().map(|f| f.len()).unwrap_or_default();
        let count = count.clone();
        move |()| {
            let new = (*count + initial).min(files).max(initial);
            log::info!("Setting count to {new}");
            count.set(new);
        }
    };

    html! {
        if let Some(files) = files.data() {
            <Grid>
            {
                files.iter().take(*count).map(|hash| {
                    html! {
                        <FileCardPending key={hash.to_string()} hash={hash.clone()} />
                    }
                }).collect::<Html>()
            }
            {
                files.iter().skip(*count + 1).take(trailing).map(|hash| {
                    let callback = callback.clone();
                    html! {
                        <FileCardVisibility key={hash.to_string()} hash={hash.clone()} {callback} />
                    }
                }).collect::<Html>()
            }
            </Grid>
        } else {
            <div>{"Loading"}</div>
        }
    }
}
