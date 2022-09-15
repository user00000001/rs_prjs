#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case)]

mod aaa {
  const X: i32 = 0;
  pub fn print_aaa() {
    println!("{}", 42);
  }

  mod bbb {
    fn print_bbb() {
      println!("{}", 37);
    }
  }
}

mod ccc {
  pub fn print_ccc() {
    println!("{}", 25);
  }
}

mod ddd; // equal mostly: mod ddd { /* file content */ }

mod a;
use a::b::c::d;

fn main() {
  {
    // cargo new --lib foo
    //
    // foo
    // ├── Cargo.toml
    // └── src
    //     └── lib.rs // libary crate

    // cargo new --bin foo
    //
    // foo
    // ├── Cargo.toml
    // └── src
    //     └── main.rs // execute crate

    aaa::print_aaa();
    use ccc::print_ccc;
    print_ccc();
    use ddd::print_ddd;
    print_ddd();
  }

  {
    // 1. 优先查找xxx.rs 文件
    //  i. main.rs、lib.rs、mod.rs中的mod xxx; 默认优先查找同级目录下的 xxx.rs 文件；
    //  ii. 其他文件yyy.rs中的mod xxx;默认优先查找同级目录的yyy目录下的 xxx.rs 文件；
    // 2. 如果 xxx.rs 不存在，则查找 xxx/mod.rs 文件，即 xxx 目录下的 mod.rs 文件。
    d::print_ddd();

    // self
    // 
    // self 在路径中，有两种意思：
    // 1. use self::xxx 表示，加载当前模块中的 xxx。此时 self 可省略；
    // 2. use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy；

    // super
    // 
    // super 表示，当前模块路径的上一级路径，可以理解成父模块。

    // ::xxx::yyy
    // 
    // 引用根路径下的 xxx::yyy，这个根路径，指的是当前 crate 的根路径。

    // use xxx::*;
    // 
    // 表示导入 xxx 模块下的所有可见 item（加了 pub 标识的 item）

    // pub use
    // 
    // re-exporting deeply item to current path level
    a::d::print_ddd();

    // extern crate xxx;
    // 
    // 相当于引入了一个符号 xxx，后面可以直接以这个 xxx 为根引用这个 crate 中的 item
    // use xxx::yyy::zzz;
    // extern crate xxx as foo; // as: rename item
    // use foo::yyy::zzz;
  }

  {
    // Prelude: default import
    // 
    // std::marker::{Copy, Send, Sized, Sync}
    // std::ops::{Drop, Fn, FnMut, FnOnce}
    // std::mem::drop
    // std::boxed::Box
    // std::borrow::ToOwned
    // std::clone::Clone
    // std::cmp::{PartialEq, PartialOrd, Eq, Ord}
    // std::convert::{AsRef, AsMut, Into, From}
    // std::default::Default
    // std::iter::{Iterator, Extend, IntoIterator, DoubleEndedIterator, ExactSizeIterator}
    // std::option::Option::{self, Some, None}
    // std::result::Result::{self, Ok, Err}
    // std::slice::SliceConcatExt
    // std::string::{String, ToString}
    // std::vec::Vec
    use std::prelude::*;
    let v = v1::Box::new(10);
  }

  {
    // pub(crate) item;
    // 
    // 来限制 item 只在当前 crate 中可用，在当前 crate 的其他子树中，可以通过 use + path 的语法来引用 item

    // pub restricted 的使用
    // 
    // old: VISIBILITY ::= <empty> | `pub`
    // new: VISIBILITY ::= <empty> | `pub` | `pub` `(` USE_PATH `)` | `pub` `(` `crate` `)`
    // 1, pub 无明确指定意味着无限制；
    // 2, pub(crate) 当前 crate 有效；
    // 3, pub(in <path>) 在 <path> 表示的模块中有效。
  }
}

// // Intent: `a1` exports `I`, `bar`, and `foo`, but nothing else.
// pub mod a1 {
//   pub const I: i32 = 3;
//   // `semisecret` will be used "many" places within `a1`, but
//   // is not meant to be exposed outside of `a1`.
//   fn semisecret(x: i32) -> i32  { use self::b::c::J; x + J }
//   pub fn bar(z: i32) -> i32 { semisecret(I) * z }
//   pub fn foo(y: i32) -> i32 { semisecret(I) + y }
//   mod b {
//       mod c {
//           const J: i32 = 4; // J is meant to be hidden from the outside world.
//       }
//   }
// }

// // Intent: `a1` exports `I`, `bar`, and `foo`, but nothing else.
// pub mod a1 {
//   pub const I: i32 = 3;
//   // `semisecret` will be used "many" places within `a1`, but
//   // is not meant to be exposed outside of `a1`.
//   // (If we put `pub use` here, then *anyone* could access it.)
//   use self::b::semisecret;
//   pub fn bar(z: i32) -> i32 { semisecret(I) * z }
//   pub fn foo(y: i32) -> i32 { semisecret(I) + y }
//   mod b {
//       pub use self::c::semisecret;
//       mod c {
//           const J: i32 = 4; // J is meant to be hidden from the outside world.
//           pub fn semisecret(x: i32) -> i32  { x + J }
//       }
//   }
// }

// Intent: `a1` exports `I`, `bar`, and `foo`, but nothing else.
pub mod a1 {
  pub const I: i32 = 3;
  // `semisecret` will be used "many" places within `a1`, but
  // is not meant to be exposed outside of `a1`.
  // (`pub use` would be *rejected*; see Note 1 below)
  use self::b::semisecret;
  pub fn bar(z: i32) -> i32 { semisecret(I) * z }
  pub fn foo(y: i32) -> i32 { semisecret(I) + y }
  mod b {
      pub(in crate::a1) use self::c::semisecret;
      mod c {
          const J: i32 = 4; // J is meant to be hidden from the outside world.
          // `pub(in a1)` means "usable within hierarchy of `mod a1`, but not
          // elsewhere."
          pub(in crate::a1) fn semisecret(x: i32) -> i32  { x + J }
      }
  }
}

mod a2 {
  #[derive(Default)]
  struct Priv(i32);
  pub mod b {
      use crate::a2::Priv as Priv_a;
      #[derive(Default)]
      pub struct F {
          pub    x: i32,
                 y: Priv_a,
          pub(in crate::a2) z: Priv_a,
      }
      #[derive(Default)]
      pub struct G(pub i32, Priv_a, pub(in crate::a2) Priv_a);
      // ... accesses to F.{x,y,z} ...
      // ... accesses to G.{0,1,2} ...
  }
  // ... accesses to F.{x,z} ...
  // ... accesses to G.{0,2} ...
}
mod k {
  use crate::a2::b::{F, G};
  // ... accesses to F and F.x ...
  // ... accesses to G and G.0 ...
}

// crate: c1
pub mod a3 {
  struct Priv(i32);
  pub(crate) struct R { pub y: i32, z: Priv } // ok: field allowed to be more public
  pub        struct S { pub y: i32, z: Priv }
  // pub fn to_r_bad(s: S) -> R { R {y:1, z:Priv(1)} } //~ ERROR: `R` restricted solely to this crate
  pub(crate) fn to_r(s: S) -> R { R { y: s.y, z: s.z } } // ok: restricted to crate
}
use crate::a3::{R, S}; // ok: `a::R` and `a::S` are both visible
// pub use crate::a3::R as ReexportAttempt; //~ ERROR: `a::R` restricted solely to this crate

// crate: c2
// extern crate c1;
// use c1::a::S; // ok: `S` is unrestricted
// use c1::a::R; //~ ERROR: `c1::a::R` not visible outside of its crate