use std::future::Future;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use pin_project_lite::pin_project;
use tokio::time::Instant;

pin_project! {
    pub struct LogElapsed<F> {
        #[pin]
        future: F,
    }
}

pub trait FutureLogExt: Future {
    fn log_elapsed(self) -> LogElapsed<Self>
    where
        Self: Sized,
    {
        LogElapsed { future: self }
    }
}

impl<F> Future for LogElapsed<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let time = Instant::now();
        let output = ready!(this.future.poll(cx));
        let elapsed = time.elapsed();
        tracing::Span::current().record("elapsed", format!("{:.3?}", elapsed));
        Poll::Ready(output)
    }
}

impl<T: ?Sized> FutureLogExt for T where T: Future {}
