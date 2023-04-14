mod future;
mod layer;

pub use self::layer::TimeoutLayer;

use self::future::ResponseFuture;
use std::{
    task::{Context, Poll},
    time::Duration,
};
use tower::Service;

pub struct Timeout<S> {
    inner: S,
    timeout: Duration,
}

impl<S> Timeout<S> {
    pub fn new(inner: S, timeout: Duration) -> Self {
        Timeout { inner, timeout }
    }
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
    S::Error: Into<tower::BoxError>,
{
    type Response = S::Response;
    type Error = tower::BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let inner_response = self.inner.call(req);
        let sleep = tokio::time::sleep(self.timeout);

        ResponseFuture::new(inner_response, sleep)
    }
}
