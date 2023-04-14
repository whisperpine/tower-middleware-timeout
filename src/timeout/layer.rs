use super::Timeout;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TimeoutLayer {
    timeout: Duration,
}

impl TimeoutLayer {
    pub fn new(timeout: Duration) -> Self {
        TimeoutLayer { timeout }
    }
}

impl<S> tower::Layer<S> for TimeoutLayer {
    type Service = Timeout<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Timeout::new(inner, self.timeout)
    }
}
