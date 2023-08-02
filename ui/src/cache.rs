use crate::request::HttpRequest;
use cindy_common::cache::{CacheKey, RcValue};
use std::{any::Any, collections::BTreeMap, fmt::Debug, rc::Rc, sync::Mutex};
use yew::{
    functional::{UseStateHandle, UseStateSetter},
    prelude::*,
};

#[derive(Clone)]
pub struct Entry {
    pub progress: bool,
    /// Current cached value.
    pub value: RcValue,
    /// List of subscribers to this value.
    pub subscriptions: Vec<UseStateSetter<RcValue>>,
}

impl Entry {
    /// Broadcast the current value of the cache entry to all subscribers.
    pub fn broadcast(&self) {
        for subscriber in &self.subscriptions {
            subscriber.set(self.value.clone());
        }
    }

    /// Subscribe for updates
    pub fn subscribe(&mut self, setter: &UseStateSetter<RcValue>) {
        if !self.subscriptions.iter().any(|i| i == setter) {
            self.subscriptions.push(setter.clone());
        }
    }

    /// Unsubscribe for updates
    pub fn unsubscribe(&mut self, setter: &UseStateSetter<RcValue>) {
        self.subscriptions.retain(|s| s != setter);
    }
}

pub trait CacheItem: CacheKey + Clone + Ord {
    //type Target: Clone + Debug + PartialEq + 'static;
}

impl<T: Debug + Clone + Ord + 'static> CacheItem for T {}

#[derive(Clone, Default)]
pub struct BTreeCache {
    pub entries: BTreeMap<Box<dyn CacheKey>, Entry>,
}

#[derive(Clone, Default)]
pub struct Cache {
    pub cache: Rc<Mutex<BTreeCache>>,
}

impl PartialEq for Cache {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.cache, &other.cache)
    }
}

impl BTreeCache {
    /// Unsubscribe to the value of this data.
    pub fn mutate<T: CacheKey, R, F: FnOnce(&mut Entry) -> R>(
        &mut self,
        data: &T,
        mutate: F,
    ) -> Option<R> {
        if let Some(entry) = self.entries.get_mut(data as &dyn CacheKey) {
            Some(mutate(entry))
        } else {
            None
        }
    }

    /// Unsubscribe to the value of this data.
    pub fn mutate_all<F: Fn(&Box<dyn CacheKey>, &mut Entry)>(&mut self, mutate: F) {
        for (key, entry) in &mut self.entries {
            mutate(key, entry);
        }
    }

    /// Unsubscribe to the value of this data.
    pub fn insert<T: CacheKey>(&mut self, data: T, entry: Entry) {
        let key = Box::new(data);
        self.entries.insert(key, entry);
    }
}

impl Cache {
    fn subscribe<R: HttpRequest + CacheItem>(&self, request: &R, handle: UseStateHandle<RcValue>)
    where
        R::Response: PartialEq,
    {
        let setter = handle.setter();
        let mut cache = self.cache.lock().expect("Failure to lock cache");

        // add self as subscriber to cache value, if exists.
        let mutated = cache.mutate(request, |entry| {
            entry.subscribe(&setter);

            // only set it if it is different
            let value = entry.value.clone().downcast::<R::Response>().unwrap();
            let current = (*handle).clone().downcast::<R::Response>().unwrap();
            if value != current {
                setter.set(entry.value.clone());
            }

            entry.clone()
        });

        match mutated {
            None => {
                cache.insert(
                    request.clone(),
                    Entry {
                        progress: true,
                        value: RcValue::default(),
                        subscriptions: vec![setter.clone()],
                    },
                );
                drop(cache);
                self.fetch(request);
            }
            Some(entry) if !entry.value.valid() && !entry.progress => {
                drop(cache);
                self.fetch(request);
            }
            _ => {}
        }
    }

    /// Trigger a fetch of this data.
    fn fetch<T: HttpRequest + CacheItem>(&self, data: &T) {
        let data = data.clone();
        let cache = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match data.send().await {
                Ok(result) => cache.cache(&data, Rc::new(result)),
                Err(error) => cache.failure(&data, error),
            }
        });
    }

    /// Cache this data.
    pub fn failure<T: HttpRequest + CacheItem>(&self, data: &T, error: crate::request::Error) {
        self.cache
            .lock()
            .expect("Failure to lock cache")
            .mutate(data, move |entry| {
                entry.progress = false;
                entry.broadcast();
            });
    }

    /// Cache this data.
    pub fn cache<T: HttpRequest + CacheItem>(&self, data: &T, value: Rc<T::Response>) {
        self.cache
            .lock()
            .expect("Failure to lock cache")
            .mutate(data, move |entry| {
                entry.value = RcValue::new(value as Rc<dyn Any>);
                entry.broadcast();
            });
    }

    /// Unsubscribe to the value of this data.
    pub fn unsubscribe<T: HttpRequest + CacheItem>(
        &self,
        data: &T,
        setter: &UseStateSetter<RcValue>,
    ) {
        self.cache
            .lock()
            .expect("Failure to lock cache")
            .mutate(data, |entry| {
                entry.unsubscribe(setter);
            });
    }

    /// Invalidate this data.
    pub fn invalidate<T: HttpRequest + CacheItem>(&self, data: &T) {
        self.cache
            .lock()
            .expect("Failure to lock cache")
            .mutate(data, |entry| {
                entry.value.invalidate();
                entry.broadcast();
            });
    }

    /// FIXME: invalidates entire cache.
    pub fn invalidate_all(&self) {
        let mut cache = self.cache.lock().expect("Failure to lock cache");
        cache.mutate_all(|_key, entry| {
            entry.value.invalidate();
            entry.broadcast();
        });
    }
}

#[derive(Properties, PartialEq)]
pub struct CacheProviderProps {
    pub children: Children,
}

#[function_component]
pub fn CacheProvider(props: &CacheProviderProps) -> Html {
    log::debug!("Creating new cache");
    let state = use_state(Cache::default);
    let context: Cache = (*state).clone();
    html! {
        <ContextProvider<Cache> {context}>
        { for props.children.iter() }
        </ContextProvider<Cache>>
    }
}

#[hook]
pub fn use_cached<R: HttpRequest + CacheItem>(data: R) -> RcValue<R::Response>
where
    R::Response: PartialEq,
{
    log::debug!("use_data({data:?})");
    let cache = use_context::<Cache>().expect("Cache not present");
    let state = use_state(|| RcValue::default());
    let state_clone = state.clone();
    use_effect(move || {
        cache.subscribe(&data, state_clone.clone());
        move || {
            cache.unsubscribe(&data, &state_clone.setter());
        }
    });
    let value = (*state).clone();
    value.downcast().expect("Value is of wrong type")
}
