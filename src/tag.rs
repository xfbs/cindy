use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
}

#[test]
fn tag_methods() {
    let tag = Tag::new("name".into(), "value".into());
    assert_eq!(tag.name(), "name");
    assert_eq!(tag.value(), "value");
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TagFilter<'a>(Option<Cow<'a, str>>, Option<Cow<'a, str>>);

impl<'a> TagFilter<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(name: Option<S>, value: Option<S>) -> Self {
        TagFilter(name.map(Into::into), value.map(Into::into))
    }

    pub fn name(&self) -> Option<&str> {
        self.0.as_ref().map(|v| v.borrow())
    }

    pub fn value(&self) -> Option<&str> {
        self.1.as_ref().map(|v| v.borrow())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TagPredicate<'a> {
    Exists(TagFilter<'a>),
    Missing(TagFilter<'a>),
}

impl<'a> TagPredicate<'a> {
    pub fn exists(&self) -> bool {
        matches!(self, TagPredicate::Exists(_))
    }

    pub fn filter(&self) -> &TagFilter<'a> {
        match self {
            Self::Exists(filter) => filter,
            Self::Missing(filter) => filter,
        }
    }
}

impl<'a> From<TagFilter<'a>> for TagPredicate<'a> {
    fn from(filter: TagFilter<'a>) -> Self {
        TagPredicate::Exists(filter)
    }
}

impl FromStr for TagPredicate<'static> {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.starts_with('!') {
            false => input.parse().map(TagPredicate::Exists),
            true => input
                .trim_start_matches('!')
                .parse()
                .map(TagPredicate::Missing),
        }
    }
}

#[test]
fn path_predicate_from_str() {
    assert_eq!(
        TagPredicate::from_str("tag:value").unwrap(),
        TagPredicate::Exists(TagFilter::new(Some("tag"), Some("value")))
    );
    assert_eq!(
        TagPredicate::from_str("*:value").unwrap(),
        TagPredicate::Exists(TagFilter::new(None, Some("value")))
    );
    assert_eq!(
        TagPredicate::from_str("name:*").unwrap(),
        TagPredicate::Exists(TagFilter::new(Some("name"), None))
    );

    assert_eq!(
        TagPredicate::from_str("!tag:value").unwrap(),
        TagPredicate::Missing(TagFilter::new(Some("tag"), Some("value")))
    );
    assert_eq!(
        TagPredicate::from_str("!*:value").unwrap(),
        TagPredicate::Missing(TagFilter::new(None, Some("value")))
    );
    assert_eq!(
        TagPredicate::from_str("!name:*").unwrap(),
        TagPredicate::Missing(TagFilter::new(Some("name"), None))
    );
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ParseError {
    #[error("missing colon")]
    MissingColon,
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

fn parse_glob(input: &str) -> Option<&str> {
    match input {
        "*" => None,
        other => Some(other),
    }
}

impl FromStr for TagFilter<'static> {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some((name, value)) = input.split_once(':') else {
            return Err(ParseError::MissingColon);
        };
        Ok(TagFilter(
            parse_glob(name).map(|v| v.to_string().into()),
            parse_glob(value).map(|v| v.to_string().into()),
        ))
    }
}

#[test]
fn tag_from_str() {
    let tag: Tag = "name:value".parse().unwrap();
    assert_eq!(tag.name(), "name");
    assert_eq!(tag.value(), "value");
}
