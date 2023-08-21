use crate::tag::{ParseError, TagFilter};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(String, String);

impl Tag {
    pub fn new(tag: String, value: String) -> Self {
        Tag(tag, value)
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &str {
        &self.1
    }

    pub fn filter(&self) -> TagFilter<'_> {
        TagFilter::new(Some(&self.0), Some(&self.1))
    }

    pub fn into_filter(self) -> TagFilter<'static> {
        TagFilter::new(Some(self.0), Some(self.1))
    }
}

impl FromStr for Tag {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.split_once(':') {
            Some((tag, value)) => Ok(Tag::new(tag.into(), value.into())),
            None => Err(ParseError::MissingColon),
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_methods() {
        let tag = Tag::new("name".into(), "value".into());
        assert_eq!(tag.name(), "name");
        assert_eq!(tag.value(), "value");
    }

    #[test]
    fn tag_from_str() {
        let tag: Tag = "name:value".parse().unwrap();
        assert_eq!(tag.name(), "name");
        assert_eq!(tag.value(), "value");
    }
}
