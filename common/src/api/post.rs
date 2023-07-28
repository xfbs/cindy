use crate::hash::*;

pub struct TagCreate {
    pub name: String,
    pub value: String,
}

pub struct FileTagCreate {
    pub hash: BoxHash,
    pub name: String,
    pub value: String,
}
