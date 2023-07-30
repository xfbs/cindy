use crate::BoxHash;

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
