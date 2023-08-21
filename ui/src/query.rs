use crate::prelude::Route;
use cindy_common::TagPredicate;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RawQuery {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
}

impl RawQuery {
    fn decode(self) -> Query {
        Query {
            sort: self.sort,
            group: self.group,
            query: match self.query.as_deref().map(serde_json::from_str) {
                Some(Ok(result)) => result,
                Some(Err(error)) => {
                    log::error!("Failed to parse query: {error}");
                    Default::default()
                }
                None => Default::default(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Query {
    #[serde(default)]
    pub query: Rc<Vec<Rc<TagPredicate<'static>>>>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
}

impl Query {
    fn encode(self) -> RawQuery {
        RawQuery {
            sort: self.sort,
            group: self.group,
            query: match self.query.len() {
                0 => None,
                _ => Some(serde_json::to_string(&self.query).unwrap()),
            },
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct QueryState {
    pub raw: Rc<RawQuery>,
    pub query: Rc<Query>,
    navigator: Navigator,
    route: Route,
}

impl QueryState {
    pub fn predicate_append(&self, predicate: TagPredicate<'static>) {
        let mut predicates: Vec<_> = (*self.query.query).clone();
        predicates.push(predicate.into());
        let query = Query {
            query: Rc::new(predicates),
            ..(*self.query).clone()
        };
        if let Err(error) = self
            .navigator
            .replace_with_query(&Route::Home, &query.encode())
        {
            log::error!("{error:?}");
        }
    }

    pub fn predicate_remove(&self, index: usize) -> Option<Rc<TagPredicate<'static>>> {
        let mut predicates: Vec<_> = (*self.query.query).clone();
        if index >= predicates.len() {
            return None;
        }

        let predicate = predicates.remove(index);
        let query = Query {
            query: Rc::new(predicates),
            ..(*self.query).clone()
        };

        if let Err(error) = self
            .navigator
            .replace_with_query(&Route::Home, &query.encode())
        {
            log::error!("{error:?}");
        }

        Some(predicate)
    }
}

#[derive(Properties, PartialEq)]
pub struct QueryStateProviderProps {
    pub children: Children,
}

#[function_component]
pub fn QueryStateProvider(props: &QueryStateProviderProps) -> Html {
    let location = use_location().unwrap();
    let route = use_route().unwrap();
    let navigator = use_navigator().unwrap();
    let raw: RawQuery = location.query().unwrap();

    let state = QueryState {
        query: raw.clone().decode().into(),
        raw: raw.into(),
        navigator,
        route,
    };

    html! {
        <ContextProvider<QueryState> context={state} >
        { for props.children.iter() }
        </ContextProvider<QueryState>>
    }
}

#[hook]
pub fn use_query_state() -> Option<QueryState> {
    use_context()
}
