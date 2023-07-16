use crate::tag::Tag;
use anyhow::Result;
use std::{collections::BTreeSet, fs::File};

pub trait Plugin {
    fn inspect(&self, file: &mut File) -> Result<BTreeSet<Tag>>;
}
