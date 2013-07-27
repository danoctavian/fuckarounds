
extern mod extra;
use std::num;
use std::uint;
use std::sys;
use std::to_bytes::ToBytes;
use std::vec;
use std::iterator;
use extra::sha2::*;
use extra::digest;
/*stages:

*  get work
*  do work
*  send work back

strategy:
  do it linearly at first - you might not need to do it in any other manner...

 */


fn main() {
  println("coin digger 0.1");
  let mut x : u32  = 1;
  let arr : ~[u8] = ~[1, 2, 3, 4, 0, 0];
  //let sz = arr.iter().size();
//  println(sz.second().unwrap().to_str());
  println(count_trailing_zeros(arr).to_str());

}


fn dig() {
  // get work

  let mut sh = ~Sha256::new();       
  let hashF = |data : &[u8]| -> ~[u8] {
    let interm = apply_hash(sh, data);
    apply_hash(sh, interm)
  };
  // do work
  //send result
}

struct Task {
  data: ~[u8],
  difficulty: uint,  
  nonce: u32,
  work_size: u32  
}


/* just stick it at the end */
fn append_nonce(data : &mut [u8], nonce : u32) {
  let bytes = nonce.to_bytes(false);
  uint::iterate(0, bytes.len(), |i| {
    data[data.len() - bytes.len() + i] = bytes[i];
    true
  });
}

fn apply_hash<D: digest::Digest>(sh : &mut D, data: &[u8]) -> ~[u8] {
  sh.input(data);
  let mut output = ~[];
  sh.result(output);
  sh.reset();
  output
}

/*
   fn test_hash<D: Digest>(sh: &mut D, tests: &[Test]) {
 for tests.iter().advance() |t| {
     sh.input_str(t.input);

     let out_str = sh.result_str();
     assert!(out_str == t.output_str);

     sh.reset();
*/


fn hash(data: &[u8]) -> ~[u8] {

  ~[]
}



  /* strange that i cannot say b == 0 because b has type &&u8
    the more iterators i add the more &s :)
    also strange how from iter doesn't work really...

  let x : ~[u8] = iterator::FromIterator::from_iterator(&arr.iter());
  */
fn count_trailing_zeros(bytes : &[u8]) -> uint {
  let mut count = 0;
   bytes.rev_iter().take_while(|b| {b.equals(&0)}).
   advance(|_| {count += 1; true});
  count
}

/* given a task, find the block
for which the hash meets difficulty requirements */
fn findBlock(task : &Task,
            hashFunc : &fn(&[u8]) -> ~[u8]) -> Option<~[u8]> {
  let mut data = task.data.clone();
  let mut nonce = task.nonce;
  let mut work_size = task.work_size;
  while work_size > 0 {
    append_nonce(data, nonce);
    let hash = hashFunc(data);
    if count_trailing_zeros(hash) >= task.difficulty { //success!
      return Some(data);   
    }
    nonce += 1;
    work_size -= 1;
  }
  None 
 }

//fn get_bytes<T : IterBytes>(x : &T) 

/* iterate for integers*/
pub fn iterate<N: Integer + Clone>(lo: N, hi: N, it: &fn(&N) -> bool) -> bool {
    let mut i : N = lo.clone();
    while i < hi {
      if (!it(&i)) { return false; }
      i = i + num::One::one();
    }
  true
}
