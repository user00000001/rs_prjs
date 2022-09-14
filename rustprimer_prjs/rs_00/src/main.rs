#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() {
  {
    // operator: ()
    // implement by Trait Fn / FnMut / FnOnce 

    let plus_one = |x: i32| x + 1;
    // let plus_one = |x: i32| -> i32 { x + 1 }; // no need to give type of input/return parameter
    assert_eq!(2, plus_one(1));
    let plus_two = |x| {
      let mut result: i32 = x;
      result += 1;
      result += 1;
      result
    };
    assert_eq!(4, plus_two(2));

    // different definations
    fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
    let plus_one_v2 = |x: i32| -> i32 { x + 1 };
    let plus_one_v3 = |x: i32| x + 1 ;

    let num = 5;
    let plus_num = |x: i32| x + num; // immutable borrow here
    assert_eq!(10, plus_num(5));
  }

  {
    let mut num = 5;
    {
      let plus_num = |x: i32| x + num; // immutable borrow here
      // let y = &mut num; // error: can not be mutable borrowed
      assert_eq!(10, plus_num(5));
    }
    // let plus_num = |x: i32| x + num; // immutable borrow here
    let y = &mut num; // immutable borrow out of scope
    // assert_eq!(10, plus_num(5));

    let nums = vec!(1,2,3);
    let takes_nums = || nums; // Vec<i32> not implements Copy Trait, nums is moved(as return parameter)
    // println!("{:?}", nums); // error: can not access a moved variable

    let num = 5;
    let owns_num = move|x: i32| x + num; // i32 implements Copy Trait, num is copied.
    println!("{} {}", num, owns_num(1)); // num still owns value

    let mut num = 5;
    { let mut add_num = |x: i32| num += x; add_num(5); }
    assert_eq!(10, num); // same object

    let mut num = 5;
    { let mut add_num = move |x: i32| num += x; add_num(5); }
    assert_eq!(5, num); // different object
  }

  {
    // foo()
    // mod foo {
    //   pub trait Fn<Args> : FnMut<Args> { // &self
    //       extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    //   }
    //   pub trait FnMut<Args> : FnOnce<Args> { // &mut self
    //       extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    //   }
    //   pub trait FnOnce<Args> { // self
    //       type Output;
    //       extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    //   }
    // }

    fn call_with_one<F>(some_closure: F) -> i32 // static dispatch
    where F: Fn(i32) -> i32 {
      some_closure(1)
    }
    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);

    fn call_with_one1<F>(some_closure: &F) -> i32 // dynamic dispatch
    where F: Fn(i32) -> i32 {
      some_closure(1)
    }   
    let answer = call_with_one1(&|x| x + 2);
    assert_eq!(3, answer);

    fn call_with_one2(some_closure: &dyn Fn(i32)->i32) -> i32 {
      some_closure(1)
    }
    fn add_one(i: i32) -> i32 {
      i+1
    }
    let f = add_one;
    // let answer = call_with_one2(&f);
    let answer = call_with_one2(&add_one);
    assert_eq!(2, answer);

    // fn factory() -> (Fn(i32)->i32) { // error: return parameter size not known
    // fn factory() -> &dyn (Fn(i32)->i32) { // error: return parameter haven't explict lifetime
    // fn factory() -> &'static dyn (Fn(i32)->i32) { // error: an object can be set to a static lifetime directly
    fn factory() -> Box<dyn (Fn(i32)->i32)> { // parameter in heap
      let num = 5;
      // |x| x+num // error: not a reference type
      // Box::new(|x|x+num) // error: num is borrowed
      Box::new(move|x|x+num)
    }
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);
  }
}