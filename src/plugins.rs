use crate::{hash::Hash, tag::Tag};
use anyhow::Result;
use std::{
    collections::BTreeSet,
    fs::{File, Metadata},
};

// TODO: move media_info to plugin system
// TODO: move paths into to plugin system
pub trait Plugin {
    fn inspect_data(&self, hash: &Hash, file: &mut File) -> Result<BTreeSet<Tag>>;
    fn inspect_metadata(&self, hash: &Hash, metadata: &Metadata) -> Result<BTreeSet<Tag>>;
}
