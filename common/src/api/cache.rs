use crate::{BoxHash, Hash, Tag};
use std::{
    any::Any,
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

mod value;
pub use value::*;

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

pub trait CacheableRequest: Cacheable {
    type Target: 'static;
}

pub trait CacheKey: Ord {
    fn new<R: CacheableRequest>(request: R) -> Self;
}

pub trait Mutator {
    fn mutations(&self) -> Vec<Mutation>;
}

pub trait CacheValue {
    fn new<D: Any>(data: D) -> Self;
}

pub struct BTreeCache<K: CacheKey, E: CacheValue> {
    pub entries: BTreeMap<K, E>,
}

impl<K: CacheKey, V: CacheValue> BTreeCache<K, V> {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    fn insert<R: CacheableRequest>(&mut self, request: R, data: R::Target) {
        self.entries.insert(K::new(request), V::new(data));
    }
}

