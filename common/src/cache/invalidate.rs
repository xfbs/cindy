use crate::{BoxHash, Mutation};
use std::time::Duration;

/// Something which can be invalidated.
pub trait Invalidatable<T = Mutation> {
    fn invalidated_by(&self, mutation: &T) -> bool {
        true
    }

    fn duration(&self) -> Option<Duration> {
        None
    }
}

impl<T> Invalidatable<()> for T {}

/// Something which produces invalidations.
pub trait Invalidator<T = Mutation> {
    fn mutations(&self) -> Vec<T>;
}
