use crate::hash::Hash;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct Config {
    pub index: IndexConfig,
    pub thumbs: ThumbsConfig,
    pub data: DataConfig,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DataConfig {
    pub path: PathBuf,
    pub hash: HashAlgorithm,
    pub prefix: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct IndexConfig {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ThumbsConfig {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum HashAlgorithm {
    #[default]
    Blake2b512,
    Blake2s256,
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            path: "index.db".into(),
        }
    }
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            path: "data".into(),
            hash: Default::default(),
            prefix: vec![2, 2],
        }
    }
}

impl DataConfig {
    pub fn data_path(&self, hash: &Hash) -> PathBuf {
        let string = hash.to_string();
        let mut slice = &string[..];
        let mut path = self.path.clone();
        for length in &self.prefix {
            let (current, rest) = slice.split_at(*length as usize);
            path.push(current);
            slice = rest;
        }
        path.push(slice);
        path
    }
}

impl Default for ThumbsConfig {
    fn default() -> Self {
        Self {
            path: "thumbs".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_default() {
        let _default = Config::default();
    }

    #[test]
    fn test_clone() {
        let default = Config::default();
        let _cloned = default.clone();
    }

    #[test]
    fn test_debug() {
        let default = Config::default();
        format!("{default:?}");
    }

    #[test]
    fn test_serialize() {
        let config = Config::default();
        let config_string = toml::to_string(&config).unwrap();
        let config_parsed = toml::from_str(&config_string).unwrap();
        assert_eq!(config, config_parsed);
    }

    #[test]
    fn test_parse() {
        let config_str = r#"
[data]
path = "data"
hash = "blake2b512"
prefix = [2, 2]

[index]
path = "index.db"

[thumbs]
path = "thumbs"
        "#;
        let _config: Config = toml::from_str(config_str).unwrap();
    }

    #[test]
    fn test_data_path() {
        let data = DataConfig::default();
        let hash = [0x9a, 0xbc, 0xde, 0xf0];
        let path = data.data_path(&hash);
        assert_eq!(path, Path::new("data/9a/bc/def0"));
    }
}
