use std::{
    fmt::Display,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::Future;
use pin_project::pin_project;
use tokio::time::{sleep, Sleep};
use tower::{BoxError, Service};

/// Tower Middleware for adding timeout functionality to another ower service.
pub struct TimeoutMiddleware<S> {
    inner: S,
    duration: Duration,
}

impl<S> TimeoutMiddleware<S> {
    pub fn new(inner: S, duration: Duration) -> Self {
        Self { inner, duration }
    }
}

impl<S, R> Service<R> for TimeoutMiddleware<S>
where
    S: Service<R>,
    S::Error: Into<BoxError> + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = TimeoutMiddlewareFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: R) -> Self::Future {
        TimeoutMiddlewareFuture {
            future: self.inner.call(req),
            sleep: sleep(self.duration),
        }
    }
}

#[pin_project]
pub struct TimeoutMiddlewareFuture<F> {
    #[pin]
    future: F,
    #[pin]
    sleep: Sleep,
}

impl<F, T, E> Future for TimeoutMiddlewareFuture<F>
where
    F: Future<Output = Result<T, E>>,
    E: Into<BoxError> + Send + Sync + 'static,
{
    type Output = Result<T, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.future.poll(cx) {
            Poll::Pending => {}
            Poll::Ready(result) => return Poll::Ready(result.map_err(Into::into)),
        }

        match this.sleep.poll(cx) {
            Poll::Pending => {}
            Poll::Ready(_) => return Poll::Ready(Err(Box::new(TimeoutElapsedError))),
        }

        Poll::Pending
    }
}

#[derive(Debug)]
pub struct TimeoutElapsedError;

impl Display for TimeoutElapsedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "timeout elapsed")
    }
}

impl std::error::Error for TimeoutElapsedError {}
