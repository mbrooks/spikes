struct AppMiddleware;

impl AppMiddleware {
    fn new() -> AppMiddleware {
        AppMiddleware {}
    }

    fn call(&self, app: App) -> App {
        app
    }
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
}

#[allow(dead_code)]
impl App {
    fn new(req: ServiceRequest, res: ServiceResponse) -> App {
        App {
            req,
            res
        }
    }

    fn add(self, middleware: AppMiddleware) -> App {
        middleware.call(self)
    }
    
    fn get_request(&self) -> &ServiceRequest {
        &self.req
    }
    
    fn done(&mut self) {
        self.res.send();
    }
}

trait Middleware {
    fn new() -> AppMiddleware;
    fn call(&self, app: App) -> App;
}

impl Middleware {
    fn new() -> AppMiddleware {
        AppMiddleware {}
    }

    fn call(&self, mut app: App) -> App {
        let request: &ServiceRequest = app.get_request();
        println!("Middleware {}", request.get_body());
        app.res.set_body("Response Body here".to_string());
        app
    }
}

fn main() {
    let request: ServiceRequest = ServiceRequest::new(&"Hello World!".to_string());
    let response: ServiceResponse = ServiceResponse::new();
    let app: App = App::new(request, response);
    app
        .add(Middleware::new())
        .add(Middleware::new())
        .done();
}
