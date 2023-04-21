use futures::future::{ready, Ready};
use hyper::{Body, Request, Response};
use std::{
    convert::Infallible,
    task::{Context, Poll},
    time::Duration,
};
use tower::{Service, ServiceBuilder};

use crate::{
    layer::{logging_layer::LoggingLayer, timeout_layer::TimeoutLayer, timing_layer::TimingLayer},
    middleware::{
        logging_middleware::LoggingMiddleware, timeout_middleware::TimeoutMiddleware,
        timing_middleware::TimingMiddleware,
    },
};

pub struct HelloWorldService;

pub type ServiceType = LoggingMiddleware<TimingMiddleware<TimeoutMiddleware<HelloWorld>>>;

impl HelloWorldService {
    pub fn new() -> ServiceType {
        ServiceBuilder::new()
            .layer(LoggingLayer::new())
            .layer(TimingLayer::new())
            .layer(TimeoutLayer::new(Duration::from_secs(5)))
            .service(HelloWorld)
    }
}

pub struct HelloWorld;

impl Service<Request<Body>> for HelloWorld {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        ready(Ok(Response::new(Body::from("Hello, world!"))))
    }
}
