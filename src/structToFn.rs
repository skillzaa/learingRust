struct Numbers {
    name:String,
    no1:u32,
    no2:u32,
}

fn main() {
let numbers  = Numbers{
name : String::from("Mike"),
no1 : 55,
no2 : 55,
};   


let result:u32 = add(&numbers);
println!("The name is  :: {}",numbers.name);
println!("the result is :: {}",result);
}// main ends here

fn add(numbers : &Numbers)->u32{
    numbers.no1+ numbers.no2
}