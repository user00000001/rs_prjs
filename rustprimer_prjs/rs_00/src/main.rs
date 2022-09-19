#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() -> Result<(), std::io::Error> {
  // Into/From AsRef/AsMut Borrow/BorrowMut/ToOwned Deref Cow
  {
    let string = "hello".to_string();
    let other_string = String::from("hello"); // let foo: U = U::from(bar: T), U implements From<T>
    assert_eq!(string, other_string);

    fn is_hello<T: Into<Vec<u8>>>(s: T) {
      let bytes = b"hello".to_vec();
      assert_eq!(bytes, s.into());
    }
    let s = "hello".to_string();
    is_hello(s); // String implements Into<Vec<u8>>

    struct Person {
      name: String,
    }
    impl Person {
      fn new(name: String) -> Person {
        Person {name: name}
      }
    }
    let name = "Herman".to_string();
    let person = Person::new(name);
    // let person = Person::new("Herman"); // error: The input type is String, not &str 
    
    struct Person1 {
      name: String,
    }
    impl Person1 {
      fn new<S: Into<String>>(name: S) -> Person1 {
        Person1 { name: name.into() } // &str/String into String
      }
    }
    let person1 = Person1::new("Herman"); 
    let person1 = Person1::new("Herman".to_string());
    // impl<'a> From<&'a str> for String {} // String from &str
    // impl<T> From<T> for T {} // T from T
    // impl<T, U> Into<U> for T where U: From<T> {} // T into U
  }

  {
    // T: AsRef<U> , T: owned/shared reference/mutable reference
    fn is_hello<T: AsRef<str>>(s: T) {
      assert_eq!("hello", s.as_ref()); // String asref str to &str
    }
    let s = "hello";
    is_hello(s); // &str asref str to &str
    let s = "hello".to_string();
    is_hello(s);

    // T: AsMut<U> , T: mutable reference
    let mut i = Box::new(0i32);
    fn add_one<U: AsMut<i32>>(u: &mut U) { // &mut Box<i32> asmut &mut i32
      *u.as_mut() += 1;
    }
    add_one(&mut i);
    println!("0 after add_one: {}", i);
  }

  use std::borrow::{Borrow, BorrowMut, ToOwned};
  {
    // T: Borrow<U>: like AsRef, more strict than AsRef
    // 
    fn check<T: Borrow<str>>(s: T) {
      assert_eq!("Hello", s.borrow()); // String/&str borrow str to &str
    }
    let s = "Hello".to_string();
    check(s);
    let s = "Hello";
    check(s);

    // T: BorrowMut<U>: like AsMut, more strict than AsMut
    // 
    fn add_one<T: BorrowMut<u32>>(t: &mut T) {
      *t.borrow_mut() += 1;
    }
    let mut i = Box::new(0);
    add_one(&mut i);
    println!("0 after add_one: {}", i);

    // ToOwned: like Clone, more widely than Clone
    //
    let s = "String";
    let s1 = s.to_owned();
    println!("{} {}", s, s1);
    let v: &[i32] = &[1,2,3];
    let v1: Vec<i32> = v.to_owned();
    println!("{:?} {:?}", v, v1);
  }

  use std::rc::Rc;
  {
    // Deref
    //
    // v: T, ref_v = &v, deref_ref_v = *v_ref

    // coercion
    //
    // T: Deref<Target=U> , foo: T => &foo: &U // similar to AsRef
    //
    // *v => *&v: Box,Rc,Arc,Cow => &v; ****&&&&v => (***)*&v // reference normallization(compilor works at *&v)
    fn foo(s: &str) {}
    let owned = "hello".to_string();
    foo(&owned); // String implements Deref<Target=str>
    let counted = Rc::new(owned);
    foo(&counted); // Rc<T> implements Deref<Target=T>
    fn foo1(s: &[i32]) {}
    let owned = vec![1,2,3];
    foo1(&owned); // Vec<T> implements Deref<Target=[T]>

    struct Foo;
    impl Foo {
      fn foo(&self) {
        println!("Foo");
      }
    }
    let f = &&Foo;
    f.foo(); 
    // (&&f_obj).foo();
    // (**(&&f_obj)).foo();

    (&f).foo();
    (&&f).foo();
    (&&&&&&f).foo();
  }

  use std::borrow::Cow; // Clone-on-write, enum type: Borrowed/Owned
  {
    // Cow
    //
    // 1, can invoke fn xxx(&self, ...) directly
    // 2, can clone(only effect once) then modify


    // .to_mut() may not clone, but not clone again(effect once)
    // .into_owned(self) takes the ownership

    let mut cow: Cow<[_]> = Cow::Owned(vec![1,2,3]);
    let hello = cow.to_mut();
    assert_eq!(hello, &[1,2,3]);

    let hello = cow.into_owned();
    assert_eq!(hello, &[1,2,3]);

    fn abs_all(input: &mut Cow<[i32]>) {
      for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
          input.to_mut()[i] = -v; // input is clone into a new vector the first time(if not already owned)
          // another to_mut using the same new vector
        }
      }
    }

    fn remove_spaces(input: &str) -> String { // if input define type is String, &str input type will clone data, String input type will take the ownership
      let mut buf = String::with_capacity(input.len());
      for c in input.chars() {
        if c != ' ' {
          buf.push(c);
        }
      }
      buf
    }
    fn remove_spaces1<'a>(input: &'a str) -> Cow<'a, str> {
      if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        for c in input.chars() {
          if c != ' ' {
            buf.push(c);
          }
        }
        return Cow::Owned(buf); // take the new String as return parameter
      }
      return Cow::Borrowed(input); // when input not contains spaces, won't clone data
    } 
  }

  Ok(())
}