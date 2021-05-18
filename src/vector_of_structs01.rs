#[derive(Debug)]
struct Obj {
no:u128,
id:u128,
}

impl Obj{
   fn new(no:u128,id:u128)->Obj{
      Obj {
         no:no,
         id:id,
      }
   }
}         

fn main (){
let mut v:Vec<Obj> = Vec::new();
v.push(Obj::new(44, 68));

println!("{:?}",v.get(0));
println!("{:?}",v.get(20));

}
