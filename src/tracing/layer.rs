use tower::Layer;

use super::middleware::TracingMiddleware;

pub struct TracingLayer;

impl TracingLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for TracingLayer {
    type Service = TracingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TracingMiddleware::new(inner)
    }
}

impl Default for TracingLayer {
    fn default() -> Self {
        Self::new()
    }
}
