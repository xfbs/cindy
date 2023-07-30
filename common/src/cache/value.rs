use std::{
    any::Any,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value<T> {
    /// Valid means the data is usable.
    valid: bool,
    data: Option<T>,
}

pub type RcValue<V = dyn Any> = Value<Rc<V>>;
pub type ArcValue<V = dyn Any + Send + Sync> = Value<Arc<V>>;

impl Value<Rc<dyn Any>> {
    pub fn downcast<T: 'static>(self) -> Option<Value<Rc<T>>> {
        let value = Value {
            valid: self.valid,
            data: match self.data {
                None => None,
                Some(data) => Some(data.downcast::<T>().ok()?),
            },
        };
        Some(value)
    }
}

impl Value<Arc<dyn Any + Send + Sync>> {
    pub fn downcast<T: Any + Send + Sync>(self) -> Option<Value<Arc<T>>> {
        let value = Value {
            valid: self.valid,
            data: match self.data {
                None => None,
                Some(data) => Some(data.downcast::<T>().ok()?),
            },
        };
        Some(value)
    }
}

impl<T> Default for Value<T> {
    fn default() -> Self {
        Self {
            valid: false,
            data: None,
        }
    }
}

impl<T> Value<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Some(data),
            valid: true,
        }
    }

    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn valid(&self) -> bool {
        self.valid
    }

    pub fn invalidate(&mut self) {
        self.valid = false;
    }
}
