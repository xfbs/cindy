use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::{convert::Infallible, error::Error};

pub trait OutputFormat: Sized {
    type Target;
    type Error: Error + Sync + Send + 'static;

    fn decode(data: Bytes) -> Result<Self::Target, Self::Error>;
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

impl OutputFormat for () {
    type Target = ();
    type Error = Infallible;

    fn decode(_data: Bytes) -> Result<Self::Target, Self::Error> {
        // TODO: raise error on non empty body?
        Ok(())
    }
}

pub trait InputFormat {
    fn encode(&self) -> Bytes;
}

impl InputFormat for () {
    fn encode(&self) -> Bytes {
        Vec::new().into()
    }
}

impl InputFormat for Bytes {
    fn encode(&self) -> Bytes {
        self.clone()
    }
}

impl<T: Serialize> InputFormat for Json<T> {
    fn encode(&self) -> Bytes {
        serde_json::to_vec(&self.0).unwrap().into()
    }
}
