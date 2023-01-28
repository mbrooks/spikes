trait AppMiddleware {
    fn new() -> Self;
    fn call(&self, app: App) -> App;
}

struct ServiceRequest {
    body: String,
}

impl ServiceRequest {
    fn new(body: &String) -> ServiceRequest {
        ServiceRequest {
            body: body.to_string()
        }
    }

    fn get_body(&self) -> String {
        self.body.clone()
    }
}

impl std::fmt::Display for ServiceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

struct ServiceResponse {
    body: String,
}

impl ServiceResponse {
    fn new() -> ServiceResponse {
        ServiceResponse {
            body: "".to_string()
        }
    }

    fn set_body(&mut self, body: String) {
        self.body = body;
    }

    fn get_body(&self) -> String {
        self.body.clone()
    }

    fn send(&self) {
        println!("This is the response: {}", self.get_body());
    }
}

struct App {
    req: ServiceRequest,
    res: ServiceResponse,
    done: bool,
}

#[allow(dead_code)]
impl App {
    fn new(req: ServiceRequest, res: ServiceResponse) -> App {
        App {
            req,
            res,
            done: false,
        }
    }

    fn add<S: AppMiddleware>(self, middleware: S) -> App {
        // if done, do nothing
        if self.done {
            return self
        }

        middleware.call(self)
    }
    
    fn get_request(&self) -> &ServiceRequest {
        &self.req
    }
    
    fn done(&mut self) {
        self.res.send();
    }
}

struct Middleware1;

impl AppMiddleware for Middleware1 {
    fn new() -> Middleware1 {
        Middleware1 {}
    }

    fn call(&self, mut app: App) -> App {
        let request: &ServiceRequest = app.get_request();
        println!("Middleware1 {}", request.get_body());
        app.res.set_body("First Response Body".to_string());
        app.done = true;
        app
    }
}

struct Middleware2;

impl AppMiddleware for Middleware2 {
    fn new() -> Middleware2 {
        Middleware2 {}
    }

    fn call(&self, mut app: App) -> App {
        let request: &ServiceRequest = app.get_request();
        println!("Middleware2 {}", request.get_body());
        let second_response: &str = "Second Response Body";
        let response_body: String = format!("{} {}", app.res.get_body(), second_response);
        app.res.set_body(response_body);
        app
    }
}

fn main() {
    let request: ServiceRequest = ServiceRequest::new(&"Hello World!".to_string());
    let response: ServiceResponse = ServiceResponse::new();
    let app: App = App::new(request, response);
    app
        .add(Middleware1::new())
        .add(Middleware2::new())
        .done();
}
