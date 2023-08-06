use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TagNameInfo {
    /// Count of possible values
    pub values: u64,

    /// Whether this is a system tag that should not be modified.
    pub system: bool,

    /// What to display this tag name as.
    pub display: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TagValueInfo {
    /// Count of files tagged with this
    pub files: u64,

    /// Whether this is a system tag that should not be modified.
    pub system: bool,

    /// What to display this tag value as.
    pub display: String,
}
