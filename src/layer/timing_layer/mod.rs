use tower::Layer;

use crate::middleware::timing_middleware::TimingMiddleware;

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
