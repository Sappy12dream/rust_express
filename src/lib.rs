pub mod router;

pub struct App {
    router: router::Router,
}

impl App {
    pub fn new() -> Self {
        App {
            router: router::Router::new(),
        }
    }

    pub fn get(&mut self, route: &str, handler: fn(&mut Request) -> Response) {
        self.router.get(route, handler);
    }

    pub fn post(&mut self, route: &str, handler: fn(&mut Request) -> Response) {
        self.router.post(route, handler);
    }

    pub fn handle_request(&self, request: &mut Request) -> Response {
        self.router.handle_request(request)
    }
}

pub struct Request {
    pub method: String,
    pub path: String,
}

pub struct Response {
    pub status_code: u16,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16, body: String) -> Self {
        Response { status_code, body }
    }
}
