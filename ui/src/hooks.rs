use crate::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{IntersectionObserver, IntersectionObserverEntry};
use yew::functional::*;
use yew_router::prelude::use_location;

#[hook]
/// Check if an element is visible.
///
/// If the sticky bit is set, then the element remains stuck in the visible position even when not
/// visible anymore.
pub fn use_visible(node: NodeRef, sticky: bool) -> bool {
    // https://stackoverflow.com/questions/1462138/event-listener-for-when-element-becomes-visible
    let visible = use_state_eq(|| false);
    let visible_clone = visible.clone();
    use_effect(move || {
        let visible = visible_clone.clone();
        let closure = Closure::<dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver)>::new(
            move |entries: Vec<IntersectionObserverEntry>, observer: IntersectionObserver| {
                // determine if any part of this node is visible.
                let visible = entries.iter().any(|entry| entry.intersection_ratio() > 0.0);

                // if the visibility changed, update the state.
                if (visible != *visible_clone) && (!sticky || !*visible_clone) {
                    visible_clone.set(visible);
                }

                // if this is sticky and it is currently visible, disconnect the observer.
                if visible && sticky {
                    observer.disconnect();
                }
            },
        )
        .into_js_value();
        let observer = IntersectionObserver::new(&closure.dyn_ref().unwrap()).unwrap();
        if let Some(node) = node.get() {
            observer.observe(&node.dyn_ref().unwrap());
        }
        move || {
            visible.set(false);
            observer.disconnect()
        }
    });
    *visible
}

#[hook]
pub fn use_query_state() -> QueryState {
    let location = use_location().unwrap();
    let query: QueryState = location.query().unwrap();
    query
}
