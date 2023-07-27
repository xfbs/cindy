use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInspectProps {
    pub file: RcHash,
}

#[function_component]
pub fn FileInspect(props: &FileInspectProps) -> Html {
    let content = FileContent {
        hash: Cow::Borrowed(&props.file),
    };
    html! {
        <div class="rounded-lg shadow min-h-screen min-w-screen p-3">
            <img class="rounded-lg w-screen" src={format!("/{}", content.uri())} alt="" />
        </div>
    }
}
