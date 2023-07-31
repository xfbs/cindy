use crate::Mutation;

/// Something which can be invalidated.
pub trait Invalidatable<M = Mutation> {
    fn invalidated_by(&self, _mutation: &M) -> bool {
        true
    }
}

impl<T> Invalidatable<()> for T {}

/// Something which produces invalidations.
pub trait Invalidator<M = Mutation> {
    fn mutations(&self) -> Vec<M>;
}
