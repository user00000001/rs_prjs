use tokio::net::TcpStream;
use tokio::sync::Notify;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::future::Future;
use futures::future::poll_fn;
use std::sync::{Arc};
use std::thread;

// pub trait Future {
//     type Output;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context)
//         -> Poll<Self::Output>;
// }

async fn delay(dur: Duration) {
    let when = Instant::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    thread::spawn(move || {
        let now = Instant::now();
        if now < when {
            thread::sleep(when - now);
        }
        notify2.notify_one();
    });
    notify.notified().await;
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // cx.waker().wake_by_ref();
            // Poll::Pending
            let waker = cx.waker().clone();
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when-now);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

enum MainFuture {
    #[allow(dead_code)]
    State0,
    State1(Delay),
    Terminated,
}

impl Future for MainFuture {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        use MainFuture::*;
        loop {
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_secs(3);
                    let future = Delay { when };
                    *self = State1(future);
                }
                State1(ref mut my_future) => {
                // State1(&mut my_future) => { // not the same
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, "done");
                            *self = Terminated;
                            return Poll::Ready(out);
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    } 
                }
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}

async fn my_async_fn() {
    println!("hello from async");
    let _socket = TcpStream::connect("127.0.0.1:22").await.unwrap();
    println!("async TCP operation complete");
}

#[tokio::main]
async fn main() {
    let what_is_this = my_async_fn(); // Future
    what_is_this.await;
    delay(Duration::from_secs(3)).await;

    let when = Instant::now() + Duration::from_secs(3);
    let future = Delay { when };
    let out = future.await;
    println!("Delay Future: {}", out);

    let when = Instant::now() + Duration::from_secs(3);
    let future = MainFuture::State1(Delay{when});
    let out = future.await;
    println!("Main Future with Delay Future: {}", out);

    let when = Instant::now() + Duration::from_secs(3);
    let mut delay = Some(Delay{when});
    poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        assert!(res.is_pending());
        tokio::spawn(async move {
            delay.await;
        });
        Poll::Ready(())
    }).await;
}
