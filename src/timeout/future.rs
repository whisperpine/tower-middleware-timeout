use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Sleep;

#[pin_project::pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    inner_response: F,
    #[pin]
    sleep: Sleep,
}

impl<F> ResponseFuture<F> {
    pub fn new(inner_response: F, sleep: Sleep) -> Self {
        ResponseFuture {
            inner_response,
            sleep,
        }
    }
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<tower::BoxError>,
{
    type Output = Result<Response, tower::BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();

        match this.inner_response.poll(cx) {
            Poll::Ready(result) => {
                return Poll::Ready(result).map_err(Into::into);
            }
            Poll::Pending => {}
        }

        match this.sleep.poll(cx) {
            Poll::Ready(()) => {
                let error = anyhow::anyhow!("timeout error");
                return Poll::Ready(Err(error.into()));
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}
