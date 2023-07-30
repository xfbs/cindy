use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagQuery<S: Borrow<str>> {
    pub name: Option<S>,
    pub value: Option<S>,
}
