use rust_express::{App, Request, Response};

fn main() {
    let mut app = App::new();

    app.get("/", |req| {
        Response::new(200, format!("Hello from GET {}", req.path))
    });

    app.post("/", |req| {
        Response::new(200, format!("Hello from POST {}", req.path))
    });

    let mut request = Request {
        method: "GET".to_string(),
        path: "/".to_string(),
    };

    let response = app.handle_request(&mut request);
    println!("Response: {} {}", response.status_code, response.body);
}
