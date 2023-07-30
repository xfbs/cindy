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

    fn decode(data: Bytes) -> Result<Self::Target, Self::Error> {
        Ok(())
    }
}

pub trait InputFormat {
    type Target;
    fn encode(target: &Self::Target) -> Bytes;
}

impl InputFormat for () {
    type Target = ();

    fn encode(target: &Self::Target) -> Bytes {
        Vec::new().into()
    }
}

impl InputFormat for Bytes {
    type Target = Bytes;

    fn encode(target: &Self::Target) -> Bytes {
        target.clone()
    }
}

impl<T: Serialize> InputFormat for Json<T> {
    type Target = T;

    fn encode(data: &Self::Target) -> Bytes {
        serde_json::to_vec(data).unwrap().into()
    }
}
