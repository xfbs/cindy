use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInspectProps {
    pub file: RcHash,
    #[prop_or_default]
    pub overlays: bool,
}

#[function_component]
pub fn FileInspect(props: &FileInspectProps) -> Html {
    let content = FileContent {
        hash: props.file.clone(),
    };
    let size = (500, 500);
    let labels: Vec<(Tag, Label)> = vec![(
        Tag::new("name".into(), "value".into()),
        Rectangle {
            start: Point::new(55, 90),
            end: Point::new(150, 150),
        }
        .into(),
    )];
    html! {
        <div class="p-3 relative">
            <img class="shadow rounded-lg w-screen" src={format!("/{}", content.uri())} alt="" />
            if props.overlays {
            {
                labels.iter().cloned().map(|(tag, label)| {
                    match label {
                        Label::Rectangle(rectangle) => html! { <RectangleOverlay {tag} {rectangle} /> },
                        _ => html!{}
                    }
                }).collect::<Html>()
            }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct RectangleOverlayProps {
    pub tag: Tag,
    pub rectangle: Rectangle,
}

#[function_component]
pub fn RectangleOverlay(props: &RectangleOverlayProps) -> Html {
    html! {
        <div class="absolute rounded bg-red-400 opacity-50 hover:opacity-80 text-center" style="bottom: 50%; left: 50%; width: 15%; height: 15%;">
            {props.tag.to_string()}
        </div>
    }
}