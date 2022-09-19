#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {
  // Rc, Arc, Mutex, RwLock, Cell, RefCell
  use std::rc::{Rc, Weak};
  {
    // Rc: immutable / one thread / reference count = 0, then recycle
    //
    let five = Rc::new(5);
    let five2 = five.clone(); // sharing ownership with five2, reference count add 1
    let five3 = five.clone(); // sharing ownership with five3
    let five4 = five2.clone(); // sharing ownership with five4
    println!(
      "five: {:?} \nfive2: {:?} \nfive3: {:?} \nfive4: {:?}", 
      &*five as *const i32,  
      &*five2 as *const i32,
      &*five3 as *const i32,
      &*five4 as *const i32,
    ); // reference the same object
    println!("{:?}", (five, five2, five3, five4));

    // Rc Weak: not add reference count(not sharing ownership)
    //
    let five = Rc::new(5);
    let mut weak_five = Rc::downgrade(&five);
    println!("{:#?} sc: {}, wc: {}", weak_five, weak_five.strong_count(), weak_five.weak_count());
    let weak_five1 = weak_five.clone();
    if weak_five.upgrade().is_some() {
      println!("{} from weak", unsafe{ &*weak_five.as_ptr() });
      let raw = weak_five.into_raw();
      println!("{} from weak", unsafe{ &*raw });
      weak_five = unsafe { Weak::from_raw(raw) };
    }
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("{:?} has sc: {}, wc: {}", strong_five.unwrap(), weak_five.strong_count(), weak_five.weak_count());
  }

  use std::sync::{Arc, Weak as AWeak};
  use std::thread::{self, JoinHandle};
  {
    let mut join_handles = Vec::<JoinHandle<()>>::new();
    // Arc - atomic reference count: cross threads sharing / (im)mutable object
    //
    let numbers: Vec<_> = (0..10u32).collect();
    let shared_numbers = Arc::new(numbers);
    for i in 0..5 {
      let child_numbers = shared_numbers.clone();
      let task = thread::spawn(move||{
        let local_numbers = &child_numbers[..];
        println!("{:?}", local_numbers);
      });
      join_handles.push(task);
    }
    for task in join_handles {
      task.join();
    }

    let mut join_handles = Vec::<JoinHandle<()>>::new();
    // Arc Weak: 
    // 
    let five = Arc::new(5);
    let five_weak = Arc::downgrade(&five);

    for i in 0..5 {
      let weak_five = five_weak.clone();
      let task = thread::spawn(move||{
        println!("{} has sc: {}, wc: {}", weak_five.upgrade().unwrap(), weak_five.strong_count(), weak_five.weak_count());
      });
      join_handles.push(task);
    }
    for task in join_handles {
      task.join();
    }
  }

  {
    struct Owner {
      name: String,
    }
    struct Gadget {
      id: i32,
      owner: Rc<Owner>,
    }
    let gadget_owner: Rc<Owner> = Rc::new(
      Owner { name: String::from("Gadget Man") }
    );
    let gadget1 = Gadget { id: 1, owner: gadget_owner.clone() }; // clone rc
    let gadget2 = Gadget { id: 2, owner: gadget_owner.clone() };
    drop(gadget_owner); // drop one rc
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name); // shared rc still own object
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);
  }

  use std::sync::{Mutex, RwLock};
  use std::sync::mpsc::channel;
  {
    // Mutex - mutual exclusion:
    //
    const N: usize = 10;
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();
    for _ in 0..10 {
      let (data, tx) = (data.clone(), tx.clone());
      thread::spawn(move||{
        let mut data = data
          .lock() // exclusion
          .unwrap(); // try_lock: no blocking, need to handle return error.
        *data += 1; // dereference mutex guard
        if*data == N {
          tx.send(()).unwrap();
          println!("data is {} now.", *data);
        }
      });
    }
    rx.recv().unwrap();
    println!("rx received.");

    let lock = RwLock::new(5);
    {
      // multiple reads
      let r1 = lock.read().unwrap(); // try_read: try_xxx like
      let r2 = lock.read().unwrap(); // not block here
      assert_eq!(*r1, 5);
      assert_eq!(*r2, 5);
    } // all in this scope are dropped
    {
      // single write
      let mut w = lock.write().unwrap(); // try_write: try_xxx like
      drop(w);
      let mut w = lock.write().unwrap(); // if previous w is not dropped, then execution will block here
      *w += 1;
      assert_eq!(*w, 6);
    } // all are dropped also
  }

  use std::cell::{Cell, RefCell};
  use std::collections::HashMap;
  {
    // Cell: Cell<T>, T need to implement Copy Trait
    //
    let c = Cell::new(5);
    let five = c.get();
    println!("{:?}", five);
    c.set(five + 1);
    println!("{:?}", c.get());

    // RefCell: runtime borrow / not cross threads
    // 
    let hm = HashMap::new();// hashmap no need to be mutable
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(hm)); // in single thread
    shared_map.borrow_mut().insert("africa", 92388); // RefMut
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
    println!("{:#?}", shared_map.borrow()); // Ref
    let hm_m1 = shared_map.borrow_mut();
    // let hm1 = shared_map.borrow(); // error: not allowed multiple (im)mutable borrow at the same time
    // let hm2 = shared_map.borrow(); // multiple borrows
    
    let result = thread::spawn(move||{
      let c = RefCell::new(5);
      let m = c.borrow();
      let b = c.borrow_mut(); // error: not allowed multiple (im)mutable borrow at the same time
    }).join();
    assert!(result.is_err()); // pass when result is error

    let c = RefCell::new(5);
    let five = c.into_inner(); // take ownership
    println!("{}", five);
    // println!("{:?}", c); // error: c is moved
  }

  {
    struct Owner {
      name: String,
      gadgets: RefCell<Vec<Weak<Gadget>>>,
    }
    struct Gadget {
      id: i32,
      owner: Rc<Owner>,
    }
    let gadget_owner: Rc<Owner> = Rc::new(Owner{
      name: "Gadget Man".to_string(),
      gadgets: RefCell::new(Vec::new()),
    });
    let gadget1 = Rc::new(Gadget{id: 1, owner: gadget_owner.clone()});
    let gadget2 = Rc::new(Gadget{id: 2, owner: gadget_owner.clone()}); // use reference to the same owner

    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1)); // use weak reference to a gadget 
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
      let gadget = gadget_opt.upgrade().unwrap();
      println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
    // gadget1/gadget2 are dropped firstly, then gadget_owner's rc is zero and dropped.
  }

  Ok(())
}