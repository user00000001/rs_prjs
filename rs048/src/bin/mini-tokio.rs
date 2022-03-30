use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::future::Future;
use std::collections::VecDeque;
use futures::task;

// pub trait Future {
//     type Output;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context)
//         -> Poll<Self::Output>;
// }

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
            cx.waker().wake_by_ref();

            // let waker = cx.waker().clone();
            // let when = self.when;
            // std::thread::spawn(move || {
            //     let now = Instant::now();
            //     if now < when {
            //         std::thread::sleep(when - now);
            //     }
            //     waker.wake();
            // });

            Poll::Pending
        }
    }
}

fn main() {
    let mut mini_tokio = MiniTokio::new();
    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_secs(3);

        // let when = Instant::now() + Duration::from_millis(10);

        let future = Delay { when };
        let out = future.await;
        assert_eq!(out, "done");
        println!("out is: {}", out);
    });
    println!("main thread working.");
    mini_tokio.run();
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

struct MiniTokio {
    tasks: VecDeque<Task>,
}

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }
    fn spawn<F>(&mut self, future: F) 
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }
    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}