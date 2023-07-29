use crate::{BoxHash, Hash, Tag};
use std::{borrow::Borrow, collections::BTreeSet, time::Duration};

/// Enumeration of possible mutations.
///
/// These cases should roughly mirror the database tables that Cindy uses. Every mutation might
/// modify, insert or delete something in a table.
pub enum Mutation {
    /// Add or delete file
    Files,
    /// Change tag name
    TagNames { name: Option<String> },
    /// Change tag value
    TagValues {
        name: Option<String>,
        value: Option<String>,
    },
    /// Change file tags
    FileTags {
        file: Option<BoxHash>,
        name: Option<String>,
        value: Option<String>,
    },
    /// Change label
    Labels {
        file: Option<BoxHash>,
        name: Option<String>,
        value: Option<String>,
    },
}

pub trait Cacheable {
    fn invalidated_by(&self, mutation: &Mutation) -> bool {
        true
    }

    fn duration(&self) -> Option<Duration> {
        None
    }
}

pub trait Mutator {
    fn mutations(&self) -> Vec<Mutation>;
}
