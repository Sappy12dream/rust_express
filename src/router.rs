use crate::{Request, Response};
use std::collections::HashMap;

pub struct Router {
    routes: HashMap<String, fn(&mut Request) -> Response>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn get(&mut self, route: &str, handler: fn(&mut Request) -> Response) {
        self.routes.insert(route.to_string(), handler);
    }

    pub fn post(&mut self, route: &str, handler: fn(&mut Request) -> Response) {
        self.routes.insert(route.to_string(), handler);
    }

    pub fn handle_request(&self, request: &mut Request) -> Response {
        match self.routes.get(&request.path) {
            Some(handler) => handler(request),
            None => Response::new(404, "Not Found".to_string()),
        }
    }
}
