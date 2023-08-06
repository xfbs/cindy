use crate::tag::{ParseError, Tag, TagPredicate};
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, Cow},
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
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

    pub fn matches(&self, tag: &Tag) -> bool {
        let name_matches = self.name().map(|name| name == tag.name()).unwrap_or(true);
        let value_matches = self
            .value()
            .map(|value| value == tag.value())
            .unwrap_or(true);
        name_matches && value_matches
    }

    pub fn exists(self) -> TagPredicate<'a> {
        TagPredicate::Exists(self)
    }

    pub fn missing(self) -> TagPredicate<'a> {
        TagPredicate::Missing(self)
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

impl<'a> Display for TagFilter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}:{}",
            self.0.as_ref().map(Cow::as_ref).unwrap_or("*"),
            self.1.as_ref().map(Cow::as_ref).unwrap_or("*")
        )
    }
}

fn parse_glob(input: &str) -> Option<&str> {
    match input {
        "*" => None,
        other => Some(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_filter() {
        let tag_filter = TagFilter::new(Some("object"), None);
        assert_eq!(tag_filter.name(), Some("object"));
        assert_eq!(tag_filter.value(), None);
    }

    #[test]
    fn test_predicate() {
        let tag_filter = TagFilter::new(Some("object"), None);

        assert_eq!(
            tag_filter.clone().exists(),
            TagPredicate::Exists(tag_filter.clone())
        );

        assert_eq!(
            tag_filter.clone().missing(),
            TagPredicate::Missing(tag_filter.clone())
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            "name:value".parse(),
            Ok(TagFilter::new(Some("name"), Some("value")))
        );
        assert_eq!("name:*".parse(), Ok(TagFilter::new(Some("name"), None)));
        assert_eq!("*:value".parse(), Ok(TagFilter::new(None, Some("value"))));
        assert_eq!("*:*".parse(), Ok(TagFilter::new::<&str>(None, None)));
        assert_eq!("abc".parse::<TagFilter>(), Err(ParseError::MissingColon));
    }

    #[test]
    fn test_display() {
        assert_eq!(
            TagFilter::new(Some("name"), Some("value")).to_string(),
            "name:value"
        );
        assert_eq!(TagFilter::new(None, Some("value")).to_string(), "*:value");
        assert_eq!(TagFilter::new(Some("name"), None).to_string(), "name:*");
        assert_eq!(TagFilter::new::<&str>(None, None).to_string(), "*:*");
    }

    #[test]
    fn tag_filter_matches() {
        let tag = Tag::new("name".into(), "value".into());

        // all matching cases:
        assert!(TagFilter::new::<&str>(None, None).matches(&tag));
        assert!(TagFilter::new::<&str>(Some("name"), None).matches(&tag));
        assert!(TagFilter::new::<&str>(None, Some("value")).matches(&tag));
        assert!(TagFilter::new::<&str>(Some("name"), Some("value")).matches(&tag));

        // non-matching examples:
        assert!(!TagFilter::new::<&str>(Some("other"), None).matches(&tag));
        assert!(!TagFilter::new::<&str>(Some("other"), Some("value")).matches(&tag));
        assert!(!TagFilter::new::<&str>(None, Some("other")).matches(&tag));
        assert!(!TagFilter::new::<&str>(Some("name"), Some("other")).matches(&tag));
        assert!(!TagFilter::new::<&str>(Some("other"), Some("other")).matches(&tag));
    }
}
