use super::*;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TagPredicate<'a> {
    Exists(TagFilter<'a>),
    Missing(TagFilter<'a>),
}

impl<'a> TagPredicate<'a> {
    pub fn exists(&self) -> bool {
        matches!(self, TagPredicate::Exists(_))
    }

    pub fn matches(&mut self, tags: &[Tag]) -> bool {
        match self {
            Self::Exists(filter) => tags.iter().any(|tag| filter.matches(tag)),
            Self::Missing(filter) => tags.iter().all(|tag| !filter.matches(tag)),
        }
    }

    pub fn filter(&self) -> &TagFilter<'a> {
        match self {
            Self::Exists(filter) => filter,
            Self::Missing(filter) => filter,
        }
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

impl<'a> Display for TagPredicate<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let prefix = match self {
            Self::Exists(_) => "",
            Self::Missing(_) => "!",
        };
        let filter = self.filter();
        write!(f, "{prefix}{filter}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_predicate_filter() {
        let filter = TagFilter::new(Some("object"), None);

        let predicate = TagPredicate::Missing(filter.clone());
        assert_eq!(predicate.filter(), &filter);
        assert_eq!(predicate.exists(), false);

        let predicate = TagPredicate::Exists(filter.clone());
        assert_eq!(predicate.filter(), &filter);
        assert_eq!(predicate.exists(), true);
    }
}
