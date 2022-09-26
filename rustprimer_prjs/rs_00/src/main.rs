#![allow(unused, deprecated, ellipsis_inclusive_range_patterns,
non_upper_case_globals, non_snake_case, unnameable_test_items)]


fn main() -> Result<(), std::io::Error> {
  {
    // Stack
    //
    #[derive(Debug)]
    struct Stack<T> {
      top: Option<Box<StackNode<T>>> // 如果不用 Box 封装，rustc 编译器会报错，在 Rust 里面，rustc 默认使用栈空间，但是这里的StackNode定义的时候使用了递归的数据结构，next 属性的类型是 StackNode<T>，而这个类型是无法确定大小的，所有这种无法确定大小的类型，都不能保存在栈空间。所以需要使用Box来封装。这样的话next的类型就是一个指向某一块堆空间的指针，而指针是可以确定大小的，因此能够保存在栈空间。
    }
    #[derive(Debug, Clone)]
    struct StackNode<T> {
      val: T,
      next: Option<Box<StackNode<T>>>, // Option 里面包括元素，None 和 Some(T) ，这样就很轻松的描述了 next 指向栈尾的元素的时候，都是在 Option 类型下，方便了功能实现，也方便了错误处理。
    }
    impl<T> StackNode<T> {
      fn new(val: T) -> StackNode<T> {
        StackNode { val, next: None }
      }
    }
    impl<T> Stack<T> {
      fn new() -> Stack<T> {
        Stack { top: None }
      }
      fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        // 使用了 Option 类型的 take 方法：
        //  fn take(&mut self) -> Option<T>
        // 它会把 Option 类型的值取走，并把它的元素改为 None
        node.next = next;
        self.top = Some(Box::new(node));
      }
      fn pop(&mut self) -> Option<T> {
        let val = self.top.take();
        match val {
          None => None,
          Some(mut x) => {
            self.top = x.next.take();
            Some(x.val)
          }
        }
      }
    }
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct {
      a: i32
    }
    let a = TestStruct{a: 5};
    let b = TestStruct{a: 9};
    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);
    s.push(&a);
    s.push(&b);
    println!("{:?}", s);
    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
  }
  
  {
    // Queue
    //
    #[derive(Debug)]
    struct Queue<T> {
      qdata: Vec<T>,
    }
    impl<T> Queue<T> {
      fn new() -> Self {
        Queue { qdata: Vec::new() }
      }
      fn push(&mut self, item: T) {
        self.qdata.push(item);
      }
      fn pop(&mut self) -> Option<T> {
        if self.qdata.len() > 0 {
          return Some(self.qdata.remove(0))
        }
        None
      }
    }
    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);
    q.pop();
    println!("{:?}", q);
  }
    
  // 在计算机科学中，二叉树是每个节点最多有两个子树的树结构。通常子树被称作“左子树”（left subtree）和“右子树”（right subtree）。二叉树常被用于实现二叉查找树和二叉堆。
  // 二叉查找树的子节点与父节点的键一般满足一定的顺序关系，习惯上，左节点的键小于父亲节点的键，右节点的键大于父亲节点的键。
  // 二叉堆是一种特殊的堆，二叉堆是完全二元树（二叉树）或者是近似完全二元树（二叉树）。二叉堆有两种：最大堆和最小堆。最大堆：父结点的键总是大于或等于任何一个子节点的键；最小堆：父结点的键总是小于或等于任何一个子节点的键。
  // 二叉树的每个结点至多只有二棵子树(不存在度大于2的结点)，二叉树的子树有左右之分，次序不能颠倒。二叉树的第i层至多有2^{i-1}个结点；深度为k的二叉树至多有2^k-1个结点；对任何一棵二叉树T，如果其终端结点数为n_0，度为2的结点数为n_2，则n_0=n_2+1。
  // 一棵深度为k，且有2^k-1个节点称之为满二叉树；深度为k，有n个节点的二叉树，当且仅当其每一个节点都与深度为k的满二叉树中，序号为1至n的节点对应时，称之为完全二叉树。
  // 二叉树不是树的一种特殊情形，尽管其与树有许多相似之处，但树和二叉树有两个主要差别：
  //   树中结点的最大度数没有限制，而二叉树结点的最大度数为2。
  //   树的结点无左、右之分，而二叉树的结点有左、右之分。  
  {
    // BinaryTree
    // 
    type TreeNode<K, V> = Option<Box<Node<K, V>>>;
    #[derive(Debug)]
    struct Node<K, V: std::fmt::Display> {
      left: TreeNode<K, V>,
      right: TreeNode<K, V>,
      key: K,
      value: V,
    }
    // 二叉树的遍历
    //   先序遍历：首先访问根，再先序遍历左（右）子树，最后先序遍历右（左）子树。
    //   中序遍历：首先中序遍历左（右）子树，再访问根，最后中序遍历右（左）子树。
    //   后序遍历：首先后序遍历左（右）子树，再后序遍历右（左）子树，最后访问根。
    trait BinaryTree<K, V> {
      fn pre_order(&self);
      fn in_order(&self);
      fn pos_order(&self);
    }
    // 由于二叉查找树要求键可排序，我们要求K实现PartialOrd
    trait BinarySearchTree<K: PartialOrd, V>: BinaryTree<K, V> {
      fn insert(&mut self, key: K, value: V);
    }
    impl<K, V: std::fmt::Display> Node<K, V> {
      fn new(key: K, value: V) -> Self {
        Node { left: None, right: None, value, key }
      }
    }
    impl<K: PartialOrd, V: std::fmt::Display> BinarySearchTree<K, V> for Node<K, V> {
      fn insert(&mut self, key: K, value: V) {
        if self.key < key {
          if let Some(ref mut right) = self.right {
            right.insert(key, value);
          } else {
            self.right = Some(Box::new(Node::new(key, value)));
          }
        } else {
          if let Some(ref mut left) = self.left {
            left.insert(key, value);
          } else {
            self.left = Some(Box::new(Node::new(key, value)));
          }
        }
      }
    }
    impl<K, V: std::fmt::Display> BinaryTree<K, V> for Node<K, V> {
      fn pre_order(&self) {
          println!("{}", self.value);
          if let Some(ref left) = self.left { left.pre_order(); }
          if let Some(ref right) = self.right { right.pre_order(); }
      }
      fn in_order(&self) {
        if let Some(ref left) = self.left { left.in_order(); }
        println!("{}", self.value);
        if let Some(ref right) = self.right { right.in_order(); }
      }
      fn pos_order(&self) {
          if let Some(ref left) = self.left { left.pos_order() }
          if let Some(ref right) = self.right { right.pos_order() }
          println!("{}", self.value);
      }
    }
    type BST<K, V> = Node<K, V>;
    let mut root = BST::<i32, i32>::new(3, 4);
    root.insert(2, 3);
    root.insert(4, 6);
    root.insert(5, 5);
    root.insert(6, 6);
    root.insert(1, 8);
    if let Some(ref left) = root.left {
      assert_eq!(left.value, 3);
    }
    if let Some(ref right) = root.right {
      assert_eq!(right.value, 6);
      if let Some(ref right) = right.right {
        assert_eq!(right.value, 5);
      }
    }
    println!("Pre Order traversal");
    root.pre_order();
    println!("In Order traversal");
    root.in_order();
    println!("Pos Order traversal");
    root.pos_order();
  }

  {
    // BinaryHeap
    //
    trait Heap<T> {
      fn push(&mut self, value: T);
      fn peek(&self) -> Option<&T>;
      fn pop(&mut self) -> Option<T>;
    }
    #[derive(Debug, Default)]
    pub struct MyHeap<T> {
      vec: Vec<T>,
    }
    impl<T: Ord> Heap<T> for MyHeap<T> {
      fn push(&mut self, value: T) { // Max heap
        self.vec.push(value);
        let mut idx = self.vec.len() - 1;
        while idx > 0 {
          let pdx = (idx - 1) / 2;
          match self.vec.get(idx) > self.vec.get(pdx) { // 尾部值比中部值大
            true => self.vec.swap(idx, pdx), // 交换位置，使中部值大
            false => return,
          }
          idx = pdx; // 中部值为新尾部值，继续循环比较，交换
        }
      }
      fn peek(&self) -> Option<&T> {
          self.vec.first()
      }
      fn pop(&mut self) -> Option<T> {
          Some(self.vec.swap_remove(0))
      }
    }
    let mut heap = MyHeap::default();
    (0..10).for_each(|e|heap.push(e));
    println!("{heap:?}");
    println!("{:?}", heap.peek().map(|x|x));
    println!("{}", heap.pop().unwrap());
  }

  {
    // 优先队列是0个或多个元素的集合，每个元素都有一个优先权或值，对优先队列执行的操作有：查找; 插入一个新元素; 删除。
    // 在最小优先队列(min priority queue)中，查找操作用来搜索优先权最小的元素，删除操作用来删除该元素；对于最大优先队列(max priority queue)，查找操作用来搜索优先权最大的元素，删除操作用来删除该元素。优先权队列中的元素可以有相同的优先权，查找与删除操作可根据任意优先权进行。

    #[derive(Debug)]
    struct PriorityQueue<T> where T: PartialOrd + Clone { pq: Vec<T> }
    impl<T> PriorityQueue<T> where T: PartialOrd + Clone {
      fn new() -> PriorityQueue<T> { PriorityQueue { pq: Vec::new() } }
      fn len(&self) -> usize { self.pq.len() }
      fn is_empty(&self) -> bool { self.pq.len() == 0 }
      fn insert(&mut self, value: T) { self.pq.push(value) }
      fn max(&self) -> Option<T> {
        if self.is_empty() { return None; }
        let max = self.max_index();
        Some(self.pq[max].clone())
      }
      fn min(&self) -> Option<T> {
        if self.is_empty() { return None; }
        let min = self.min_index();
        Some(self.pq[min].clone())
      }
      fn delete_max(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let max = self.max_index();
        Some(self.pq.remove(max).clone())
      }
      fn delete_min(&mut self) -> Option<T> {
        if self.is_empty() { return None; }
        let min = self.min_index();
        Some(self.pq.remove(min).clone())
      }
      fn max_index(&self) -> usize {
        let mut max = 0;
        for i in 1..=self.pq.len() - 1 {
          if self.pq[max] < self.pq[i] {
            max = i;
          }
        }
        max
      }
      fn min_index(&self) -> usize {
        let mut min = 0;
        for i in 0..self.pq.len() - 1 {
          if self.pq[i] < self.pq[i + 1] {
            min = i;
          }
        }
        min
      }
    }
    fn test_keep_min() {
      let mut pq = PriorityQueue::new();
      pq.insert(3);
      pq.insert(2);
      pq.insert(1);
      pq.insert(4);
      assert!(pq.min().unwrap() == 1);
    }
    fn test_keep_max() {
      let mut pq = PriorityQueue::new();
      pq.insert(2);
      pq.insert(4);
      pq.insert(1);
      pq.insert(3);
      assert!(pq.max().unwrap() == 4);
    }
    fn test_is_empty() {
      let mut pq = PriorityQueue::new();
      assert!(pq.is_empty());
      pq.insert(1);
      assert!(!pq.is_empty());
    }
    fn test_len() {
      let mut pq = PriorityQueue::new();
      assert!(pq.len() == 0);
      pq.insert(2);
      pq.insert(4);
      pq.insert(1);
      assert!(pq.len() == 3);
    }
    fn test_delete_min() {
      let mut pq = PriorityQueue::new();
      pq.insert(3);
      pq.insert(2);
      pq.insert(1);
      pq.insert(4);
      assert!(pq.len() == 4);
      assert!(pq.delete_min().unwrap() == 1);
      assert!(pq.len() == 3);
    }
    fn test_delete_max() {
      let mut pq = PriorityQueue::new();
      pq.insert(2);
      pq.insert(10);
      pq.insert(1);
      pq.insert(6);
      pq.insert(3);
      assert!(pq.len() == 5);
      assert!(pq.delete_max().unwrap() == 10);
      assert!(pq.len() == 4);
    }
    test_len();
    test_delete_max();
    test_delete_min();
    test_is_empty();
    test_keep_max();
    test_keep_min();
  }

  {
    // 链表是一种物理存储单元上非连续、非顺序的存储结构，数据元素的逻辑顺序是通过链表中的指针链接次序实现的。链表由一系列结点（链表中每一个元素称为结点）组成，结点可以在运行时动态生成。每个结点包括两个部分：一个是存储数据元素的数据域，另一个是存储下一个结点地址的指针域。 由于不必须按顺序存储，链表在给定位置插入的时候可以达到O(1)的复杂度，比另一种线性表顺序表快得多，但是在有序数据中查找一个节点或者访问特定下标的节点则需要O(n)的时间，而线性表相应的时间复杂度分别是O(logn)和O(1)。
    // 使用链表结构可以克服数组需要预先知道数据大小的缺点，链表结构可以充分利用计算机内存空间，实现灵活的内存动态管理。但是链表失去了数组随机读取的优点，同时链表由于增加了结点的指针域，空间开销比较大。链表最明显的好处就是，常规数组排列关联项目的方式可能不同于这些数据项目在内存或磁盘上的顺序，数据的存取往往要在不同的排列顺序中转换。链表允许插入和移除表上任意位置上的节点，但是不允许随机存取。链表有很多种不同的类型：单向链表，双向链表以及循环链表。

    use List::*;
    enum List { Cons(u32, Box<List>), Nil }
    impl List {
      fn new() -> List { Nil }
      fn prepend(self, elem: u32) -> List { Cons(elem, Box::new(self)) }
      fn len(&self) -> u32 {
        match *self {
          Cons(_, ref tail) => 1 + tail.len(),
          Nil => 0,
        }
      }
      fn stringify(&self) -> String {
        match *self {
          Cons(head, ref tail) => {
            format!("{}, {}", head, tail.stringify())
          },
          Nil => format!("Nil")
        }
      }
    }
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
  }

  // {
  //   // 双向链表也叫双链表，是链表的一种，它的每个数据结点中都有两个指针，分别指向直接后继和直接前驱。所以，从双向链表中的任意一个结点开始，都可以很方便地访问它的前驱结点和后继结点。一般我们都构造双向循环链表。
  //   // 循环链表是另一种形式的链式存贮结构。它的特点是表中最后一个结点的指针域指向头结点，整个链表形成一个环。
  //   use DoubleLoopLinkList::*;
  //   enum DoubleLoopLinkList { Cons(u32, Box<DoubleLoopLinkList>, Box<DoubleLoopLinkList>), Nil }
  //   impl DoubleLoopLinkList {
  //     fn new() -> DoubleLoopLinkList { Nil }
  //     fn prepend(self, elem: u32) -> DoubleLoopLinkList { 
  //       match self {
  //         Cons(_, _, ref next) => {
  //           let c = Cons(elem, Box::new(self), *next);
  //           self.next = c;
  //           return c;
  //         }
  //         Nil =>  {
  //           let mut s = Cons(elem, Box::new(Nil), Box::new(Nil));
  //           s.
  //           s.1 = &s;
  //           s.2 = &s;
  //           return s;
  //         }
  //       }
  //     }
  //     fn len(&self) -> u32 {
  //       match *self {
  //         Cons(_, ref prev, _) => 1 + prev.len(),
  //         Nil => 0,
  //       }
  //     }
  //     fn stringify(&self) -> String {
  //       match *self {
  //         Cons(head, ref prev, ref next) => {
  //           format!("{}, {}, {}", head, prev.stringify(), next.stringify())
  //         },
  //         Nil => format!("Nil")
  //       }
  //     }
  //   }
  //   let mut uni_list = DoubleLoopLinkList::new();
  //   uni_list = uni_list.prepend(1);
  //   uni_list = uni_list.prepend(2);
  //   uni_list = uni_list.prepend(3);
  //   println!("linked list has length: {}", uni_list.len());
  //   println!("{}", uni_list.stringify());
  // }

  {
    // 图的存储结构: 除了要存储图中各个顶点的本身信息外，同时还要存储顶点与顶点之间的所有关系(边的信息)，因此，图的结构比较复杂，很难以数据元素在存储区中的物理位置来表示元素之间的关系，但也正是由于其任意的特性，故物理表示方法很多。常用的图的存储结构有邻接矩阵、邻接表等。
    // 
    // 邻接矩阵表示法: 对于一个具有n个结点的图，可以使用n*n的矩阵(二维数组)来表示它们间的邻接关系。矩阵 A(i,j) = 1 表示图中存在一条边 (Vi,Vj),而A(i,j)=0表示图中不存在边 (Vi,Vj)。
    // 实际编程时，当图为不带权图时，可以在二维数组中存放 bool 值。
    //  A(i,j) = true 表示存在边 (Vi,Vj),
    //  A(i,j) = false 表示不存在边 (Vi,Vj);
    // 当图带权值时，则可以直接在二维数值中存放权值，A(i,j) = null 表示不存在边 (Vi,Vj)。
    #[derive(Debug)]
    struct Node {
      nodeid: usize,
      nodename: String,
    }
    #[derive(Debug, Clone)]
    struct Edge {
      edge: bool,
    }
    #[derive(Debug)]
    struct Graphadj {
      nodenums: usize,
      graphadj: Vec<Vec<Edge>>,
    }
    impl Node {
      fn new(nodeid: usize, nodename: String) -> Node {
        Node {
          nodeid,
          nodename,
        }
      }
    }
    impl Edge {
      fn new() -> Edge {
        Edge { edge: false }
      }
      fn have_edge() -> Edge {
        Edge { edge: true }
      }
    }
    impl Graphadj {
      fn new(nums: usize) -> Graphadj {
        Graphadj { nodenums: nums, graphadj: vec![vec![Edge::new();nums];nums] }
      }
      fn insert_edge(&mut self, v1: Node, v2: Node) {
        match v1.nodeid < self.nodenums && v2.nodeid<self.nodenums {
          true => {
            self.graphadj[v1.nodeid][v2.nodeid] = Edge::have_edge();
            // dag
            // self.graphadj[v2.nodeid][v1.nodeid] = Edge::have_edge() 
          }
          false => {
            panic!("your nodeid is bigger than nodenums!");
          }
        }
      }
    }
    let mut g = Graphadj::new(2);
    let v1 = Node::new(0, "v1".to_string());
    let v2 = Node::new(0, "v2".to_string());
    g.insert_edge(v1, v2);
    println!("{:?}", g);
  }

  {
    // 邻接表表示法: 邻接表是图的一种最主要存储结构，用来描述图上的每一个点。
    // 实现方式：对图的每个顶点建立一个容器（n个顶点建立n个容器），第i个容器中的结点包含顶点Vi的所有邻接顶点。实际上我们常用的邻接矩阵就是一种未离散化每个点的边集的邻接表。
    //  在有向图中，描述每个点向别的节点连的边（点 a->点 b 这种情况）。
    //  在无向图中，描述每个点所有的边(点 a->点 b这种情况)
    // 与邻接表相对应的存图方式叫做边集表，这种方法用一个容器存储所有的边。

  }

  Ok(())
}