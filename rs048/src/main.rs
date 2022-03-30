use tokio::net::TcpStream;
use tokio::sync::Notify;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::future::Future;
use std::collections::VecDeque;
use futures::task::{self, ArcWake};
use futures::future::poll_fn;
use crossbeam::channel;
use std::sync::{Arc, Mutex};
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

struct Delay1 {
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay1 {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
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
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        use MainFuture::*;
        loop {
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_millis(10);
                    let future = Delay { when };
                    *self = State1(future);
                }
                State1(ref mut my_future) => {
                // State1(&mut my_future) => { // not the same
                    match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, "done");
                            *self = Terminated;
                            return Poll::Ready(());
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
    let _socket = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    println!("async TCP operation complete");
}

#[tokio::main]
async fn main() {
    let what_is_this = my_async_fn(); // Future
    what_is_this.await;
    delay(Duration::from_secs(3)).await;

    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay { when };
    let out = future.await;
    println!("{}", out);

    let when = Instant::now() + Duration::from_millis(10);
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

    let mut mini_tokio = MiniTokio::new();
    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };
        let out = future.await;
        assert_eq!(out, "done");
    });
    mini_tokio.run();

    let mini_tokio1 = MiniTokio1::new();
    mini_tokio1.spawn(async {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };
        let out = future.await;
        assert_eq!(out, "done");
    });
    mini_tokio1.run();
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

struct MiniTokio {
    tasks: VecDeque<Task>,
}

struct MiniTokio1 {
    scheduled: channel::Receiver<Arc<Task1>>,
    sender: channel::Sender<Arc<Task1>>,
}

struct Task1 {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task1>>,
}

impl Task1 {
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }
    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();
        let _ = future.as_mut().poll(&mut cx);
    }
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task1>>)
    where
        F: Future<Output=()> + Send + 'static,
    {
        let task = Arc::new(Task1 {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });
        let _ = sender.send(task);
    }
}

impl ArcWake for Task1 {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl MiniTokio1 {
    fn run(&self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
    fn new() -> MiniTokio1 {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio1 { scheduled, sender }
    }
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output=()> + Send + 'static,
    {
        Task1::spawn(future, &self.sender);
    }
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