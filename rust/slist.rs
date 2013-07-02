type SListLink<T> = Option<~SListNode<T>>;

pub struct Foo {
  pub bar : int
}

pub struct SList<T> {
  priv size : uint,
  hd : SListLink<T>,
}

pub struct SListNode<T> {
  data: T,
  next : SListLink<T>
}

pub fn SList<T>() -> ~SList<T> {
  ~SList { size: 0, hd: None}
}

pub fn SListNode<T>(d : T, n : SListLink<T>) -> ~SListNode<T> {
   ~SListNode { data: d, next: n}
}

pub impl<T> SList<T> {
  fn len(& self) -> uint {self.size}
  fn set_len(&mut self, s : uint) {
    self.size = s;
  }
}

pub impl<T:Copy> SList<T> {
  fn push_front(&mut self, d : T) {
    self.hd = Some(SListNode(d, if (self.hd.is_some())
      {Some(self.hd.swap_unwrap())} else {None}));
    self.size += 1;
  }
  fn del_front(&mut self) {
    let mut old_hd = self.hd.swap_unwrap(); 
    if old_hd.next.is_some() {
      self.hd = Some(old_hd.next.swap_unwrap())
    }
    self.size -= 1;
  }
  fn peek_front(&mut self) -> Option<T> {
    self.hd.map(|head| { head. data})
  }
}

impl<T> BaseIter<T> for SList<T> {
  fn each(&self, blk: &fn(v: &T) -> bool) {  
    let mut curr = &self.hd;
    while curr.is_some() {
      let nxt = curr.get_ref();
      blk(&nxt.data);
      curr = &nxt.next;
    }
  }

  fn size_hint(&self) -> Option<uint> { Some(self.size) }
}

fn print_len<T>( list : &SList<T>) {
  println(fmt!("the length is %u", list.len())); 
}

fn print_elems<T:ToStr>(list : &SList<T>) {
  list.each(|x| {
    println(fmt!("elem: %s", x.to_str()));
    true
  });
}

fn main() {
  let mut sl = SList::<int>();
  print_len(sl); 

  sl.push_front(1);
  sl.push_front(2);

  print_len(sl); 
  let mut o = Some(1);
  print_elems(sl);
  sl.del_front();
  print_elems(sl);

/*
  let x = o.swap_unwrap();
  if (!o.is_some()) {
    println(fmt!("num: %d", x));
  }
*/

  let o1 = o.map_consume(|x| {x});
  if (o.is_some()) {
    println("nothing is gone");
  }

  let foo = Foo{bar: 2};
  let pb : &int = &foo.bar;
  println("decision tree main");
}
