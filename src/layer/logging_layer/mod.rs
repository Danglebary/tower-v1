use tower::Layer;

use crate::middleware::logging_middleware::LoggingMiddleware;

pub struct LoggingLayer;

impl LoggingLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware::new(inner)
    }
}
