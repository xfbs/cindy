use restless::{data::Json, methods::Patch, PatchRequest, RequestMethod};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagNameEditRequest<'a> {
    pub name: Option<Cow<'a, str>>,
    pub display: Option<Cow<'a, str>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TagNameEdit<S: Borrow<str>> {
    pub name: S,
    pub name_new: Option<S>,
    pub display_new: Option<S>,
}

impl<S: Borrow<str>> PatchRequest for TagNameEdit<S> {
    type Request = Json<TagNameEditRequest<'static>>;

    fn path(&self) -> Cow<'_, str> {
        format!("api/v1/tag/{}", self.name.borrow()).into()
    }

    fn body(&self) -> Self::Request {
        Json(TagNameEditRequest {
            name: self
                .name_new
                .as_ref()
                .map(|value| value.borrow().to_string().into()),
            display: self
                .display_new
                .as_ref()
                .map(|value| value.borrow().to_string().into()),
        })
    }
}

impl<S: Borrow<str>> RequestMethod for TagNameEdit<S> {
    type Method = Patch<Self>;
}
