use crate::prelude::*;
use web_sys::HtmlElement;

#[derive(Properties, PartialEq)]
struct FileInspectProps {
    pub file: RcHash,
    #[prop_or_default]
    pub overlays: bool,
}

#[function_component]
fn FileInspect(props: &FileInspectProps) -> Html {
    let content = FileContent {
        hash: props.file.clone(),
    };
    //let size = (500, 500);
    let labels: Vec<(Tag, Label)> = vec![(
        Tag::new("name".into(), "value".into()),
        Rectangle {
            start: Point::new(55, 90),
            end: Point::new(150, 150),
        }
        .into(),
    )];

    // current selection
    let selection = use_state(|| None as Option<Rectangle<f64>>);

    let onmousedown = {
        let selection = selection.clone();
        move |event: MouseEvent| {
            let target: HtmlElement = event.target_dyn_into().unwrap();
            let width_rel = event.offset_x() as f64 / target.offset_width() as f64;
            let height_rel = event.offset_y() as f64 / target.offset_height() as f64;
            log::info!("Postition {width_rel} {height_rel}");
            let point = Point::new(width_rel, height_rel);
            selection.set(Some(Rectangle {
                start: point,
                end: point,
            }));
        }
    };

    let onmousemove = {
        let selection = selection.clone();
        move |event: MouseEvent| {
            if let Some(mut rectangle) = *selection {
                let target: HtmlElement = event.target_dyn_into().unwrap();
                let width_rel = event.offset_x() as f64 / target.offset_width() as f64;
                let height_rel = event.offset_y() as f64 / target.offset_height() as f64;
                log::info!("Postition {width_rel} {height_rel}");
                let point = Point::new(width_rel, height_rel);
                rectangle.end = point;
                selection.set(Some(rectangle));
                event.prevent_default();
            }
        }
    };

    let onmouseup = {
        let selection = selection.clone();
        move |_event: MouseEvent| {
            selection.set(None);
        }
    };

    html! {
        <div class="p-3 relative" {onmousedown} {onmousemove} {onmouseup}>
            <img class="shadow rounded-lg w-screen" src={format!("/{}", content.uri())} alt="" draggable="false" />
            if props.overlays {
            {
                labels.iter().cloned().map(|(_tag, label)| {
                    match label {
                        _ => html!{}
                    }
                }).collect::<Html>()
            }
            }
            if let Some(rectangle) = *selection {
                <RectangleOverlay tag={Tag::new("test".into(), "test".into())} {rectangle} />
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct InspectNavigateProps {
    file: RcHash,
}

#[function_component]
fn InspectNavigate(props: &InspectNavigateProps) -> Html {
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

    let (previous, next) = match files.data() {
        Some(files) => (
            files
                .iter()
                .rev()
                .skip_while(|f| *f != &props.file)
                .nth(1)
                .cloned(),
            files
                .iter()
                .skip_while(|f| *f != &props.file)
                .nth(1)
                .cloned(),
        ),
        None => (None, None),
    };

    let hash = match use_route::<Route>() {
        Some(Route::File { hash, .. }) => Some(hash),
        _ => None,
    };
    html! {
        <div class="flex items-center gap-x-4">
            <Link to={Route::home()} classes="hover:text-blue-600" query={query.raw.clone()}>{"Query"}</Link>
            <div class="grow text-center font-bold">
            if let Some(hash) = &hash {
                {hash.to_string()}
            }
            </div>
            if let Some(previous) = previous {
                <Link to={Route::file(previous.into())} query={query.raw.clone()} classes="hover:text-blue-600" disabled=true>{"Previous"}</Link>
            } else {
                <button class="hover:text-blue-600" disabled=true>{"Previous"}</button>
            }
            if let Some(next) = next {
                <Link to={Route::file(next.into())} query={query.raw.clone()} classes="hover:text-blue-600" disabled=true>{"Next"}</Link>
            } else {
                <button class="hover:text-blue-600" disabled=true>{"Next"}</button>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RectangleOverlayProps {
    pub tag: Tag,
    pub rectangle: Rectangle<f64>,
}

#[function_component]
fn RectangleOverlay(props: &RectangleOverlayProps) -> Html {
    let style = format!(
        "bottom: {}%; left: {}%; width: {}%; height: {}%;",
        100.0 - 100.0 * props.rectangle.start.y.max(props.rectangle.end.y),
        100.0 * props.rectangle.start.x.min(props.rectangle.end.x),
        100.0 * props.rectangle.width().abs(),
        100.0 * props.rectangle.height().abs(),
    );
    html! {
        <div class="absolute rounded bg-red-400 opacity-50 hover:opacity-80 text-center pointer-events-none" {style}>
            {props.tag.to_string()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct FileViewProps {
    pub hash: RcHash,
}

#[function_component]
pub fn FileView(props: &FileViewProps) -> Html {
    html! {
        <div>
            <QueryStateProvider>
                <NavBar>
                    <InspectNavigate file={props.hash.clone()} />
                </NavBar>
                <SidebarLayout>
                    <SidebarLayoutSidebar>
                        <FileSidebar file={props.hash.clone()} />
                    </SidebarLayoutSidebar>
                    <SidebarLayoutContent>
                        <FileInspect file={props.hash.clone()} overlays=true />
                    </SidebarLayoutContent>
                </SidebarLayout>
            </QueryStateProvider>
        </div>
    }
}
