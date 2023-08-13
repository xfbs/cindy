mod get;
pub use get::*;

mod post;
pub use post::*;

mod delete;
pub use delete::*;

mod encoding;
pub use encoding::*;

mod patch;
pub use patch::*;

mod query;
pub use crate::cache::*;
pub use query::*;

#[cfg(test)]
mod tests;
