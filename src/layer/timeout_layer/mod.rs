use std::time::Duration;
use tower::Layer;

use crate::middleware::timeout_middleware::TimeoutMiddleware;

pub struct TimeoutLayer {
    duration: Duration,
}

impl TimeoutLayer {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

impl<S> Layer<S> for TimeoutLayer {
    type Service = TimeoutMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimeoutMiddleware::new(inner, self.duration)
    }
}
