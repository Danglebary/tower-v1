use tower::Layer;

use super::middleware::TimingMiddleware;

pub struct TimingLayer;

impl TimingLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for TimingLayer {
    type Service = TimingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimingMiddleware::new(inner)
    }
}

impl Default for TimingLayer {
    fn default() -> Self {
        Self::new()
    }
}
