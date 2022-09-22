#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case, unnameable_test_items)]


fn main() -> Result<(), std::io::Error> {
  use std::any::Any;
  // pub trait Any: 'static + Reflect { // 看，Any在定义的时候就规定了其生命周期，而Reflect是一个Marker，默认所有的Rust类型都会实现他！注意，这里不是所有原生类型，而是所有类型。
  //   fn get_type_id(&self) -> TypeId;
  // }
  use std::fmt::Debug;
  {
    // 熟悉Java的同学肯定对Java的反射能力记忆犹新，同样的，Rust也提供了运行时反射的能力。但是，这里有点小小的不同，因为 Rust 不带 VM 不带 Runtime ,因此，其提供的反射更像是一种编译时反射。
    // 因为，Rust只能对 'static 生命周期的变量（常量）进行反射！
    // 
    // 我们会有这样的需求，去某些路径里加载配置文件。我们可能提供一个配置文件路径，好吧，这是个字符串(String)。但是，当我想要传入多个配置文件的路径的时候怎们办？理所应当的，我们传入了一个数组。
    // 这下可坏了……Rust不支持重载啊！于是有人就很单纯的写了两个函数～～！
    // 其实不用……我们只需要这么写……
    fn load_config<T:Any+Debug>(value: &T) -> Vec<String> {
      let mut cfgs: Vec<String> = vec![];
      let value = value as &dyn Any; // turn type object to trait Object
      match value.downcast_ref::<String>() { // type inference
        Some(cfp) => cfgs.push(cfp.clone()),
        None => (),
      };
      match value.downcast_ref::<Vec<String>>() { // 转换成 Any 是为了有机会获取到他的类型信息，转换回来，则是为了去使用这个值本身。
        Some(v) => cfgs.extend_from_slice(&v),
        None => (),
      };
      if cfgs.len() == 0 {
        panic!("No Config File");
      }
      cfgs
    }
    let cfp = "/etc/waylog.conf".to_string();
    assert_eq!(load_config(&cfp), vec!["/etc/waylog.conf".to_string()]);
    let cfps = vec![
      "/etc/waylog.conf".to_string(),
      "/etc/waylog_sec.conf".to_string(),
    ];
    assert_eq!(load_config(&cfps), vec![
      "/etc/waylog.conf".to_string(),
      "/etc/waylog_sec.conf".to_string(),
    ]);
  }
  
  Ok(())
}