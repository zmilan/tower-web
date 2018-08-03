use super::{Route, Params};

use http::Request;

/// A set of routes
#[derive(Debug)]
pub struct RouteSet<T> {
    routes: Vec<Route<T>>,
}

// ===== impl RouteSet =====

impl<T> RouteSet<T> {
    pub fn new() -> RouteSet<T> {
        RouteSet { routes: vec![] }
    }

    pub(crate) fn map<F, U>(self, f: F) -> RouteSet<U>
    where F: Fn(T) -> U,
    {
        let mut routes = vec![];

        for route in self.routes.into_iter() {
            routes.push(route.map(&f));
        }

        RouteSet { routes }
    }

    pub(crate) fn insert(&mut self, route: Route<T>) {
        self.routes.push(route);
    }

    pub(crate) fn insert_all(&mut self, set: RouteSet<T>) {
        self.routes.extend(set.routes);
    }
}

impl<T> RouteSet<T>
where
    T: Clone,
{
    /// Match a request against a route set
    pub(crate) fn test(&self, request: &Request<()>) -> Option<(T, Params)> {
        self.routes
            .iter()
            .flat_map(|route| route.test(request))
            .next()
    }
}
