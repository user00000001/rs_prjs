#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {
  // concurrent / parallel / multiple threads
  use std::thread;
  use rand::Rng;
  use std::collections::HashMap;
  {
    let new_thread = thread::spawn(move||{ // FnOnce
      println!("I am a new thread");
    });
    new_thread.join().unwrap();

    let new_thread_result = thread::Builder::new()
      .name("thread1".to_string()) // set name
      .stack_size(4*1024*1024).spawn(move||{ // set thread stack size
        println!("I am thread1.");
      });
      new_thread_result.unwrap() // build may failed
        .join().unwrap();
    
    let new_thread = thread::spawn(move||{
      loop {
        print!("\rI am a new thread.");
      }
    });
    // new_thread.join().unwrap(); // main thread exits, children threads exit too

    let new_thread = thread::spawn(move||{
      thread::spawn(move||{
        loop {
          print!("\rI am a new thread.");
        }
      });
    });
    new_thread.join().unwrap();
    println!("Child thread is finish!"); // parent thread exits, children threads may not exits
    thread::sleep_ms(100);

    let mut goals: HashMap<usize, (i32, String)> = HashMap::new();
    for i in (1..=100usize) {
      let mut rng = rand::thread_rng();
      let x = rng.gen_range(0..=100);
      let v = thread::spawn(move||{
        match x {
          x if x >= 90 => String::from("A"),
          x if x >= 80 => String::from("B"),
          x if x >= 70 => String::from("C"),
          x if x >= 60 => String::from("D"),
          _ => String::from("E"),
        }
      }).join().unwrap();
      goals.insert(i, (x, v));
    }
    for i in 1..=100usize {
      println!("{:>3}'s goal is {:<3}, rank: {}", i, goals[&i].0, goals[&i].1);
    }
  }

  use std::sync::mpsc; // Multiple Producers Single Consumer
  use std::rc::Rc;
  {
    struct Student {
      id: u32
    }
    // !Send contains:
    //
    // 1, *mut T / *const T
    // 2, Rc / Weak

    // let (tx, rx): (mpsc::Sender<Rc<Student>>, mpsc::Receiver<Rc<Student>>) = mpsc::channel(); // error: Rc<Student> not imples Send
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    thread::spawn(move||{
      tx.send(1).unwrap(); // move tx to this thread, send i32 back to rx
    });
    println!("receive {}", rx.recv().unwrap());

    // self-define struct can send cross threads
    #[derive(Debug)]
    struct Student1<F> 
      where F: Send + 'static 
    {
      id: Rc<F>,
    }
    // self guarantee safety
    unsafe impl<F> Send for Student1<F> where F: Send + 'static {}
    let (tx, rx): (mpsc::Sender<Student1<i32>>, mpsc::Receiver<Student1<i32>>) = mpsc::channel(); 
    thread::spawn(move||{
      let id_rc = Rc::new(0);
      tx.send(Student1{id: id_rc}).unwrap(); 
    });
    println!("receive {:?}", rx.recv().unwrap());

    const THREAD_COUNT: i32 = 2;
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    for id in 0..THREAD_COUNT {
      let thread_tx = tx.clone(); // like Rc
      thread::spawn(move||{
        thread_tx.send(id+1).unwrap(); // no blocking, limitless capacity size
        println!("send {}", id + 1);
      });
    }
    thread::sleep_ms(2000);
    println!("wake up");
    for _ in 0..THREAD_COUNT {
      println!("receive {}", rx.recv().unwrap()); // blocking, FIFO
    }

    let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(0); // specified capacity size, 0 represents no cache
    let new_thread = thread::spawn(move||{
      println!("before send");
      tx.send(1).unwrap(); // capacity size 0, blocking
      println!("after send");
    });
    println!("before sleep");
    thread::sleep_ms(5000);
    println!("after sleep");
    println!("receive {}", rx.recv().unwrap());
    new_thread.join().unwrap();
  }

  use std::sync::Arc;
  {
    // shared memory: more data race
    //
    static mut VAR: i32 = 5; // const will inline to code
    let new_thread = thread::spawn(move||{
      unsafe { // self guarantee safety
        println!("static value in new thread: {}", unsafe { VAR } );
        VAR = VAR + 1 
      };
    });
    new_thread.join().unwrap();
    println!("static value in main thread: {}", unsafe { VAR } );

    let var: Arc<i32> = Arc::new(5); // includes Box implement
    let share_var = var.clone();
    let new_thread = thread::spawn(move||{
      println!("share value in new thread: {}, address: {:p}", share_var, &*share_var);
    });
    new_thread.join().unwrap();
    println!("share value in main thread: {}, address: {:p}", var, &*var); // same object pointer address
  }

  use std::sync::{
    Mutex, // methods: lock 
    Condvar, // methods: wait, notify_one, notify_all
  };
  use std::sync::atomic::{
    AtomicUsize, // AtomicBool, AtomicIsize, AtomicPtr
    Ordering,
  };
  {
    // condition variable
    //
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move||{
      let &(ref lock, ref cvar) = &*pair2;
      let mut started = lock
        .lock() // require mutex guard lock, blocking
        .unwrap(); // got mutex guard lock
      *started = true;
      cvar.notify_one();
      println!("notify main thread");
      drop(started); // release mutex guard lock
    });
    // Wait for the thread to start up?
    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap(); // need to get mutex guard lock first
    while !*started {
      println!("before wait");
      started = cvar.wait(started).unwrap(); // wait(started) consume/release mutex guard lock, return the mutex guard to get own mutex lock again
      println!("after wait");
    }

    // atomic type: more effect
    //
    let var: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(5));
    let share_var = var.clone();
    let new_thread = thread::spawn(move||{
      println!("share value in new thread: {}", share_var.load(Ordering::SeqCst));
      share_var.store(9, Ordering::SeqCst);
    });
    new_thread.join().unwrap();
    println!("share value in main thread: {}", var.load(Ordering::SeqCst));

    // lock
    // 
    let var: Arc<Mutex<u32>> = Arc::new(Mutex::new(5));
    let share_var = var.clone();
    let new_thread = thread::spawn(move||{
      let mut val = share_var.lock().unwrap();
      println!("share value in new thread: {}", *val);
      *val = 9;
    });
    new_thread.join().unwrap();
    println!("share value in main thread: {}", *(var.lock().unwrap()));
  }

  use rayon::prelude::*;
  {
    // parallel pattern :-(
    let mut colors = [-20.0f32, 0.0, 20.0, 40.0,
      80.0, 100.0, 150.0, 180.0, 200.0, 250.0, 300.0];
    println!("original: {:?}", &colors);
    colors.par_iter_mut().for_each(|color|{
      let c: f32 = if *color < 0.0 {
        0.0
      } else if *color > 255.0 {
        255.0
      } else {
        *color
      };
      *color = c / 255.0;
    });
    println!("transformed: {:?}", &colors);
  }

  Ok(())
}