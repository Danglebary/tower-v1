use hyper::{Method, Request, Response};
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::Service;

/// Tower Middleware for logging the initial Request and final Response of another Tower service.
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> LoggingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for LoggingMiddleware<S>
where
    S: Service<Request<B>, Response = Response<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LoggingMiddlewareFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        log::debug!("request {} {}", method, path);

        LoggingMiddlewareFuture {
            future: self.inner.call(req),
            method,
            path,
        }
    }
}

#[pin_project]
pub struct LoggingMiddlewareFuture<F> {
    #[pin]
    future: F,
    method: Method,
    path: String,
}

impl<F, B, E> Future for LoggingMiddlewareFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let result = match this.future.poll(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(res) => res,
        };
        let status = if let Ok(result) = &result {
            result.status().as_u16()
        } else {
            500
        };
        log::debug!("response {} {} status={}", this.method, this.path, status,);
        Poll::Ready(result)
    }
}
