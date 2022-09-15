#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() {
  {
    for i in 1..10 { // 1..10: iterator into_iter iterator, then invoke iterator next function
      println!("{}", i);
    }

    let values = vec!(1,3,5); // Vec does not implement Iterator, but IntoIterator
    for x in values {
      println!("{}", x);
    }
    // Iterator: IntoIterator
    // {
    //   let result = match IntoIterator::into_iter(values) { // check IntoIterator first: into_iter
    //     mut iter => loop {
    //       match iter.next() { // check Iterator last: next
    //         Some(x) => println!("{}", x),
    //         None => break,
    //       }
    //     },
    //   };
    //   result
    // }
    let inf_seq = (1..).into_iter(); // iterator is lazy, do not apply collect or fold methods
    for i in inf_seq.take(10) {
      println!("{}", i);
    }
  }

  {
    // let v = (1..20).collect(); // error: need type for FromIterator
    let v: Vec<_> = (1..20).collect(); // explict type of value
    let v = (1..20).collect::<Vec<_>>(); // explict type of method

    let r = (1..20).fold(1u64, |mul, x| mul*x); // MapReduce: reduce function
    println!("{}", r);
    let m: Vec<_> = (1..20).map(|x|x+1)
    .collect(); // map function is lazy
    println!("{:?}", m);
    let f: Vec<_> = (1..20).filter(|x| x%2 == 0)
    .collect(); // filter function is lazy too
    println!("{:?}", f);

    let v = vec!(1,2,3,4,5,6);
    let v_take = v.iter()
      .cloned()
      .take(2)
      .collect::<Vec<_>>();
    assert_eq!(v_take, vec![1,2]);

    let v_skip: Vec<_> = v.iter()
      .cloned()
      .skip(2)
      .collect();
    assert_eq!(v_skip, vec!(3,4,5,6));

    use std::collections::HashMap;
    let names = vec!["WaySLOG", "Mike", "Elton"];
    let scores = vec![60,80,100];
    let score_map: HashMap<_, _> = names.iter()
      .zip(scores.iter())
      .collect();
    println!("{:?}", score_map);

    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
      .enumerate()
      .filter(|&(idx,_)|idx%2 == 0)
      .map(|(idx,val)|val)
      .fold(0u64, |sum, acm| sum + acm);
    println!("{}", val);
    let val = v.iter()
      .enumerate()
      .filter(|&(idx,_)|idx%2 == 0)
      .unzip::<usize,&u64,Vec<usize>, Vec<&u64>>().1; // get only values
    println!("{:?}", val);
    let val: (Vec<usize>, Vec<&u64>) = v.iter()
      .enumerate()
      .filter(|&(idx,_)|idx%2 == 0)
      .unzip(); // get only values
    println!("{:?}", val.1);

    // find()/position() -> Option<Item>/Option<usize>
    // all()/any() -> bool
    // max()/min() -> bool
  }

  {
    let v = (0u32..).into_iter();
    let v1 = v.take(1000).filter(|i| (i/100).pow(3)+ ((i%100)/10).pow(3) + (i%10).pow(3) == *i).filter(|i|i%10 == 0).collect::<Vec<u32>>();
    for i in v1 {
      println!("{}^3 + {}^3 + {}^3 = {}", i/100, i%100/10, i%10, i);
    }
  }
}