/* decision tree */

extern mod std;
use std::vec;
use std::uint;
use std::iterator;
use std::tuple;

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

/* iterator unusuable? because of bug;
adding each */

struct IndexIterator<'self, T> {
 pub i : uint,
 pub is : &'self [uint],
 pub elems : &'self [T]
}
impl<'self, T> IndexIterator<'self, T> {
  fn new(is : &'self [uint], es : &'self [T]) -> IndexIterator<'self, T> {
    IndexIterator {i : 0, is : is, elems : es}
  }
  fn each<T>(&self, f : &fn(&T) -> bool) {
    do each(&mut self.is.iter()) |i : &uint| {
      f(&(self.elems[*i]))
    }
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


fn each<T, U : Iterator<T>>(it : &mut U, f : &fn(T) -> bool) {
  loop {
    match it.next() {
      Some(e) => { if !f(e) {break;}}
      None => { break; }
    }
  } 
}

fn each_with_indices<T>(is : &[uint], coll : &[T],  f : &fn(&T) -> bool) {
  do each(&mut is.iter()) |i : &uint| {
    f(&(coll[*i]))
  }
}

/* find the index with the biggest value */
fn get_i_with_max(xs : &[uint]) -> uint {
  let mut max = xs[0];
  let mut max_i = 0;
  for uint::range(0, xs.len()) |i| {
    if xs[i] > max {
      max = xs[i];
      max_i = i;
    }
  }  
  max_i
}

fn all_same_label(case_set : &[uint], cases : &[Case]) -> bool {
  let fstLabel = cases[case_set[0]].second();
  let mut all_same = true;
  each_with_indices(case_set, cases, |case| {
    if case.second() != fstLabel {all_same = false;}
    all_same
  });
  all_same
}


pub fn build_tree(attribs: ~[AttrInfo],
                  cases : ~[Case], classes : uint) -> ~DT {
  let build = |attrib_set : &[uint], case_set : &[uint]| -> ~DT {
    if attribs.len() == 0 {
      /* get mode */
      let mut classCount = vec::from_elem(classes, 0);
      each_with_indices(case_set, cases, |case| {
        classCount[case.second()] += 1;
        true 
      });
      ~Leaf(get_i_with_max(classCount))//~Leaf(labels_mode(case_set, &cases, classes))
    } else if all_same_label(case_set, cases) {
      ~Leaf(cases[case_set[0]].second())
    } else {
      println("computing info gain");
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
