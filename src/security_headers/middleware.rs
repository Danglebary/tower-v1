use hyper::{http::HeaderValue, Request, Response};
use phf::phf_map;
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::Service;

pub static SECURITY_HEADERS: phf::Map<&'static str, &'static str> = phf_map! {
    "Strict-Transport-Security" => "max-age=63072000; includeSubDomains; preload",
    "Content-Security-Policy" => "deault-src 'self'",
    "X-Frame-Options" => "SAMEORIGIN",
};

/// Tower Middleware for adding web security headers to responses.
pub struct SecurityHeadersMiddleware<S> {
    inner: S,
}

impl<S> SecurityHeadersMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for SecurityHeadersMiddleware<S>
where
    S: Service<Request<B>, Response = Response<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = SecurityHeadersMiddlewareFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        SecurityHeadersMiddlewareFuture {
            future: self.inner.call(req),
        }
    }
}

#[pin_project]
pub struct SecurityHeadersMiddlewareFuture<F> {
    #[pin]
    future: F,
}

impl<F, B, E> Future for SecurityHeadersMiddlewareFuture<F>
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

        let response = match result {
            Ok(mut res) => {
                let headers = res.headers_mut();
                for (k, v) in SECURITY_HEADERS.entries() {
                    headers.append(*k, HeaderValue::from_static(*v));
                }

                Ok(res)
            }
            Err(err) => Err(err),
        };

        Poll::Ready(response)
    }
}
