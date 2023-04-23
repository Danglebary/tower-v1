use hyper::{Request, Response};
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use tower::Service;
use tracing::debug;

/// Tower Middleware for logging the execution time of another Tower service.
pub struct TimingMiddleware<S> {
    inner: S,
}

impl<S> TimingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for TimingMiddleware<S>
where
    S: Service<Request<B>, Response = Response<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = TimingMiddlewareFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let start = Instant::now();
        TimingMiddlewareFuture {
            future: self.inner.call(req),
            start,
        }
    }
}

#[pin_project]
pub struct TimingMiddlewareFuture<F> {
    #[pin]
    future: F,
    start: Instant,
}

impl<F, B, E> Future for TimingMiddlewareFuture<F>
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
        let duration = this.start.elapsed();
        debug!("completed in {:?}", duration);
        Poll::Ready(result)
    }
}
