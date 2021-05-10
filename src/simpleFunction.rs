// mod print;
// mod variables;
fn main() {
let no1:u16 = 33;
let no2:u16 = 3433;
let result = add(no1, no2);
println!("the result is :: {}",result);
}// main ends here

fn add(no1:u16,no2:u16)->u16{
    no1 + no2
}