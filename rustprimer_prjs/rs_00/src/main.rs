#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

fn main() {
  {
    let mut v1: Vec<i32> = Vec::new();
    // let mut v2 = Vec::new::<i32>(); // error: new function has not implemented with generic
    let mut v2 = (0i32..5).collect::<Vec<i32>>();

    let v: Vec<i32> = vec![];
    let v = vec!(1,2,3);
    let v = vec![1,2,3];
    // let mut temp = Vec::new();
    // temp.push(1);
    // temp.push(2);
    // temp.push(3);
    // let v = temp;

    let v = vec![0; 10];

    let v: Vec<_> = (1..5).collect(); // Vec implements FromIterator trait

    let a = vec!(1,2,3);
    assert_eq!(a[1usize], 2); // Vec implements Index/IndexMut trait

    let mut v = vec!(1,2,3);
    assert_eq!(v.get(1), Some(&2)); // get or get_mut return reference
    assert_eq!(v.get_mut(3), None);

    let mut v = vec![1,2,3];
    for i in &v {} // immutable reference
    for i in &mut v {} // mutable reference
    for i in v {} // i take the element ownnership from v
  }

  {
    use std::time;
    
    fn push_1m(v: &mut Vec<usize>, total: usize) {
      let e = time::SystemTime::now();
      for i in 1..total {
        v.push(i);
      }
      let ed = time::SystemTime::now();
      println!("time spend: {:?}", ed.duration_since(e).unwrap());
    }
    let mut v: Vec<usize> = vec![];
    push_1m(&mut v, 5_000_000);
    let mut v: Vec<usize> = vec![];
    v.reserve(5_000_000); // pre-allocate space
    push_1m(&mut v, 5_000_000);
  }

  {
    // Key type: derive(PartialEq, Eq, Hash)
    // 1, Key1 == Key2, Hash(Key1) == Hash(Key2)
    // 2, Hash(Key) do not change Key value, (take cares of Cell/RefCell)
    // 3, avoid: Key1 != Key2, Hash(Key1) == Hash(Key2)
    use std::collections::HashMap;

    let mut come_from: HashMap<&str, &str> = HashMap::new();
    come_from.insert("WaySLOG", "HeBei");
    come_from.insert("Marisa", "U.S.");
    come_from.insert("Mike", "HuoGuo");

    if !come_from.contains_key("elton") {
      println!("Oh, 我们查到了{}个人，但是可怜的Elton猫还是无家可归", come_from.len());
    }

    come_from.remove("Mike");
    println!("Mike猫的家乡不是火锅！不是火锅！不是火锅！虽然好吃！");

    let who = ["MoGu", "Marisa"];
    for person in &who {
      match come_from.get(person) {
        Some(location) => println!("{} 来自: {}", person, location),
        None => println!("{} 也无家可归啊.", person),
      }
    }

    println!("那么，所有人呢？");
    for (name, location) in &come_from {
      println!("{}来自: {}", name, location);
    }

    // python2: char counter
    // 
    // val = {}
    // for c in "abcdefasdasdawe":
    //     val[c] = 1 + val.setdefault(c, 0)
    // print val

    let mut letters: HashMap<char, i32> = HashMap::new();
    for ch in "a short treatise on fungi".chars() {
      let counter = letters.entry(ch).or_insert(0);
      *counter += 1;
    }
    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);
    assert_ne!(letters.get(&'s'), None);
  }
}