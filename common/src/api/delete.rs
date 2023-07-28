use crate::hash::*;

pub struct TagDelete {
    pub name: String,
    pub value: String,
}

pub struct FileHashDelete {
    pub hash: BoxHash,
    pub name: String,
    pub value: String,
}
