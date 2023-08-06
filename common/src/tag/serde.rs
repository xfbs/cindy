use super::{Tag, TagPredicate};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

// custom serde implementation that serializes and deserializes tags as strings (using their
// as string representation).
impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // serialize as hex string or as byte array, depending on format.
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: &'de str = <&'de str>::deserialize(deserializer)?;
        Self::from_str(data).map_err(Error::custom)
    }
}

// custom serde implementation that serializes and deserializes tags as strings (using their
// as string representation).
impl<'a> Serialize for TagPredicate<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // serialize as hex string or as byte array, depending on format.
        self.to_string().serialize(serializer)
    }
}

impl<'de, 'a> Deserialize<'de> for TagPredicate<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: String = String::deserialize(deserializer)?;
        data.parse().map_err(Error::custom)
    }
}
