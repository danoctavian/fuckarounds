/* decision tree */

extern mod std;
use std::vec;
use std::uint;
use std::iterator;

type AttrId = uint;
type AVal = uint; //attribute value - for simplicity
type Label = uint;

type DTLink = Option<~DT>;

/* case with values of attributes
  array has a[AttrId] = AVal */
type Case = (~[AVal], Label);

enum DT {
  Node(AttrId, ~[DT]),
  Leaf(Label)  
}

type AttrInfo = (AttrId, int);

fn range(start : uint, end : uint) -> ~[uint] {
  if end < start {return ~[]}
  let mut a : ~[uint] = ~[];
  let size = end - start;
  a.reserve(size);
  for uint::iterate(0, size) |i| {
    a.push(start + i);
  }  
  a
}

struct IndexIterator<'self, T> {
 pub i : uint,
 pub is : &'self [uint],
 pub elems : &'self [T]
}

impl<'self, T> IndexIterator<'self, T> {
  fn new(is : &'self [uint], es : &'self [T]) -> IndexIterator<'self, T> {
    IndexIterator {i : 0, is : is, elems : es}
  }
}

impl<'self, T> Iterator<&'self T> for IndexIterator<'self, T> {
  #[inline]
  fn next(&mut self) -> Option<&'self T> {
    if self.i == self.is.len() {None}
    else {
      let n = Some(&'self self.elems[self.is[self.i]]);
      self.i += 1;
      n
    }
  }
}
/*
fn each_with_indices<T>(is : &[uint], coll : &[T],  f : &fn(&T)) {
  for vec::each(is) |i| {
    f(&(coll[*i]))
  }
}
*/

pub fn get_mode<T : Iterator<int>>(it : & mut T) {
  loop {
    match it.next() {
      Some(c) => {
        println(c.to_str());
      }
      None => {break;}
    }
  }  
}


pub fn build_tree(attribs: ~[AttrInfo],
                  cases : ~[Case], classes : uint) -> ~DT {
  let build = |attrib_set : &[uint], case_set : &[uint]| -> ~DT {
    if attribs.len() == 0 {
      /* get mode */
      
      ~Leaf(2)//~Leaf(labels_mode(case_set, &cases, classes))
    } else if (true) {
      ~Leaf(1)
    } else {
      println("build called");
      /* for each attribute compute info gain. set best attribute
          as the node attribute; make branches for each of its 
          values */
      ~Leaf(2)
    }
  };
  let attrib_set = range(0, attribs.len());
  let case_set = range(0, cases.len());
  build(attrib_set, case_set)
}

fn main() {
  let t = build_tree(~[(0, 10), (1, 12)], ~[(~[5, 6], 0)], 2);
  println("decision tree main");
  //get_mode();
//  let i = IndexIterator{};
}
