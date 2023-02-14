use http::{Request, Response, StatusCode};
use std::future::Future;
use std::future::poll_fn;
use std::pin::Pin;
use std::task::*;
use tokio;
use tower::Service;

struct HelloWorld;

impl Service<Request<&str>> for HelloWorld {
    type Response = Response<Vec<u8>>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        println!("poll_ready({:?})", cx);
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<&str>) -> Self::Future {
        println!("{:?}", req);

        // create the body
        let body: Vec<u8> = "hello, world!\n"
            .as_bytes()
            .to_owned();
        // Create the HTTP response
        let resp = Response::builder()
            .status(StatusCode::OK)
            .body(body)
            .expect("Unable to create `http::Response`");

        // create a response in a future.
        let fut = async {
            Ok(resp)
        };

        // Return the response as an immediate future
        Box::pin(fut)
    }
}

#[tokio::main]
async fn main() {
    let mut hello = HelloWorld{};
    let req = Request::new("hey!");

    let poll_result = poll_fn(|cx| hello.poll_ready(cx)).await;
    match poll_result {
        Ok(_) => {
            let res = hello.call(req).await;
            println!("{:?}", res);        
        },
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

}