use std::time::Duration;
use tower::Layer;

use super::middleware::TimeoutMiddleware;

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

impl Default for TimeoutLayer {
    fn default() -> Self {
        Self::new(Duration::from_millis(5 * 1000)) // TODO: does adding a default here make sense?
    }
}
