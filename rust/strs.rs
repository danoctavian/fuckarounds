use std::str::*;
fn main() {
  println("wtf");
  let s1 = ~"wtf1";
  let s2 = ~"wtf2";
  let s3 = &"wtf2";


  println(s1.append(s2));
  println(s3.to_owned().append(s1));
}
