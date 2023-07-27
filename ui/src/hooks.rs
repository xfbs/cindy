use crate::prelude::*;
use js_sys::Function;
use wasm_bindgen::{
    closure::{Closure, IntoWasmClosure},
    JsCast,
};
use web_sys::{IntersectionObserver, IntersectionObserverEntry};
use yew::functional::*;

#[hook]
pub fn use_visible(node: NodeRef) -> bool {
    // https://stackoverflow.com/questions/1462138/event-listener-for-when-element-becomes-visible
    let visible = use_state_eq(|| false);
    let visible_clone = visible.clone();
    use_effect_once(move || {
        let closure =
            Closure::<dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver)>::wrap(
                Box::new(
                    move |entries: Vec<IntersectionObserverEntry>,
                          observer: IntersectionObserver| {
                        visible_clone.set(
                            entries.iter().any(|entry| entry.intersection_ratio() > 0.0)
                                || *visible_clone,
                        );
                    },
                )
                .unsize(),
            )
            .into_js_value();
        let observer = IntersectionObserver::new(&closure.dyn_ref().unwrap()).unwrap();
        if let Some(node) = node.get() {
            observer.observe(&node.dyn_ref().unwrap());
        }
        move || observer.disconnect()
    });
    *visible
}
