use tokio_stream::Stream;
use async_stream::stream;
// use futures_util::stream::StreamExt;
use futures_util::pin_mut;
use std::time::{Duration, Instant};
use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};
use tokio_stream::StreamExt;

// pub trait Stream {
//     type Item;

//     fn poll_next(
//         self: Pin<&mut Self>, 
//         cx: &mut Context<'_>
//     ) -> Poll<Option<Self::Item>>;

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (0, None)
//     }
// }

struct Delay {
  when: Instant
}

impl Future for Delay {
  type Output = &'static str;
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
    -> Poll<Self::Output>
  {
    if Instant::now() >= self.when {
      println!("Hello world!");
      Poll::Ready("done")
    } else {
      cx.waker().wake_by_ref();

      Poll::Pending
    }
    // Poll::Pending
  }
}

struct Interval {
  rem: usize,
  delay: Delay,
}

impl Stream for Interval {
  type Item = &'static str;
  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>)
    -> Poll<Option<Self::Item>>
  {
    if self.rem == 0 {
      return Poll::Ready(None);
    }
    match Pin::new(&mut self.delay).poll(cx) {
      Poll::Ready(s) => {
        let when = self.delay.when + Duration::from_millis(10);
        self.delay = Delay {when};
        self.rem -= 1;
        Poll::Ready(Some(s))
      }
      Poll::Pending => Poll::Pending,
    }
  }
}

#[tokio::main]
async fn main() {
  let mut interval = Interval { rem: 5, delay: Delay { when: Instant::now() }};
  while let Some(v) = interval.next().await {
    println!("{}", v);
  }

  let s = stream! {
    let mut when = Instant::now();
    for i in 0..3 {
      let delay = Delay{when};
      delay.await;
      yield i;
      when += Duration::from_secs(1);
    }
  };
  pin_mut!(s);
  while let Some(v) = s.next().await {
    println!("Got = {:?}", v);
  }
}