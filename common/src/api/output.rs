use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::{convert::Infallible, error::Error};

pub trait OutputFormat: Sized {
    type Target;
    type Error: Error + Sync + Send + 'static;

    fn decode(data: Bytes) -> Result<Self::Target, Self::Error>;

    fn decode_boxed(data: Bytes) -> Result<Self::Target, Box<dyn Error + Sync + Send + 'static>> {
        Ok(Self::decode(data)?)
    }
}

#[derive(Clone, Debug)]
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> OutputFormat for Json<T> {
    type Target = T;
    type Error = serde_json::Error;

    fn decode(data: Bytes) -> Result<Self::Target, Self::Error> {
        serde_json::from_slice(&data[..])
    }
}

impl OutputFormat for Bytes {
    type Target = Bytes;
    type Error = Infallible;

    fn decode(data: Bytes) -> Result<Self::Target, Self::Error> {
        Ok(data)
    }
}
