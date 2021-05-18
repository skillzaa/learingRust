
fn main() {
   let mut v:Vec<i32> = vec![1,2,3,4,5];
   //==== Immutable borrow 
   let first = &v[0];
   //==== Mutable borrow
   v.push(6);
   println!("first{}",first);
}