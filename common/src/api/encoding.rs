use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::{convert::Infallible, error::Error};

pub trait ResponseEncoding: Sized {
    type Target;
    type Error: Error + Sync + Send + 'static;

    fn decode(data: Bytes) -> Result<Self::Target, Self::Error>;
}

#[derive(Clone, Debug)]
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> ResponseEncoding for Json<T> {
    type Target = T;
    type Error = serde_json::Error;

    fn decode(data: Bytes) -> Result<Self::Target, Self::Error> {
        serde_json::from_slice(&data[..])
    }
}

impl ResponseEncoding for Bytes {
    type Target = Bytes;
    type Error = Infallible;

    fn decode(data: Bytes) -> Result<Self::Target, Self::Error> {
        Ok(data)
    }
}

impl ResponseEncoding for () {
    type Target = ();
    type Error = Infallible;

    fn decode(_data: Bytes) -> Result<Self::Target, Self::Error> {
        // TODO: raise error on non empty body?
        Ok(())
    }
}

pub trait RequestEncoding {
    fn encode(&self) -> Bytes;
}

impl RequestEncoding for () {
    fn encode(&self) -> Bytes {
        Vec::new().into()
    }
}

impl RequestEncoding for Bytes {
    fn encode(&self) -> Bytes {
        self.clone()
    }
}

impl<T: Serialize> RequestEncoding for Json<T> {
    fn encode(&self) -> Bytes {
        serde_json::to_vec(&self.0).unwrap().into()
    }
}
