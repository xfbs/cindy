mod value;
pub use value::*;

mod filter;
pub use filter::*;

mod predicate;
pub use predicate::*;

mod info;
pub use info::*;

mod serde;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("missing colon")]
    MissingColon,
}
