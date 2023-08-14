use restless::clients::gloo::GlooRequest;
use cindy_common::cache::{CacheKey, RcValue};
use prokio::time::sleep;
use std::{any::Any, collections::BTreeMap, fmt::Debug, rc::Rc, sync::Mutex, time::Duration};
use yew::{
    functional::{UseStateHandle, UseStateSetter},
    prelude::*,
};

const DELAY_INITIAL: Duration = Duration::from_millis(100);
const DELAY_MULTIPLIER: f64 = 1.5;

#[derive(Clone, Default)]
pub struct Entry {
    /// Delay to use for next request
    pub delay: Option<Duration>,
    /// Fetch in-progress
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

    /// Get current delay and update.
    pub fn delay_update(&mut self) {
        self.delay = match self.delay {
            Some(current) => Some(Duration::from_secs_f64(
                current.as_secs_f64() * DELAY_MULTIPLIER,
            )),
            None => Some(DELAY_INITIAL),
        };
    }

    pub fn delay_reset(&mut self) {
        self.delay = None;
    }

    pub fn needs_fetch(&self) -> bool {
        !self.value.valid() && !self.progress
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

    pub fn get<T: CacheKey>(&self, data: &T) -> Option<&Entry> {
        self.entries.get(data as &dyn CacheKey)
    }
}

impl Cache {
    fn subscribe<R: GlooRequest + CacheItem>(&self, request: &R, handle: UseStateHandle<RcValue>)
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
                        subscriptions: vec![setter.clone()],
                        ..Default::default()
                    },
                );
                drop(cache);
                self.fetch(request, None);
            }
            Some(entry) if entry.needs_fetch() => {
                let delay = entry.delay;
                drop(cache);
                self.fetch(request, delay);
            }
            _ => {}
        }
    }

    /// Trigger a fetch of this data.
    fn fetch<T: GlooRequest + CacheItem>(&self, data: &T, delay: Option<Duration>) {
        let data = data.clone();
        let cache = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(delay) = delay {
                sleep(delay).await;
            }
            match data.send().await {
                Ok(result) => cache.cache(&data, Rc::new(result)),
                Err(error) => cache.failure(&data, &error),
            }
        });
    }

    /// Cache this data.
    pub fn failure<T: GlooRequest + CacheItem>(&self, data: &T, error: &dyn std::error::Error) {
        self.cache
            .lock()
            .expect("Failure to lock cache")
            .mutate(data, move |entry| {
                entry.delay_update();
                entry.progress = false;
                entry.broadcast();
            });
    }

    /// Cache this data.
    pub fn cache<T: GlooRequest + CacheItem>(&self, data: &T, value: Rc<T::Response>) {
        self.cache
            .lock()
            .expect("Failure to lock cache")
            .mutate(data, move |entry| {
                entry.delay_reset();
                entry.value = RcValue::new(value as Rc<dyn Any>);
                entry.progress = false;
                entry.broadcast();
            });
    }

    /// Unsubscribe to the value of this data.
    pub fn unsubscribe<T: GlooRequest + CacheItem>(
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
    pub fn invalidate<T: GlooRequest + CacheItem>(&self, data: &T) {
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
pub fn use_cached<R: GlooRequest + CacheItem>(data: R) -> RcValue<R::Response>
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
