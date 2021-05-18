struct Data {
    name:String,
    no1:u32,
    no2:u32,
}

fn main() {
let Data  = Data{
name : String::from("Mike"),
no1 : 55,
no2 : 55,
};   


let result:u32 = add(&Data);
println!("The name is  :: {}",Data.name);
println!("the result is :: {}",result);
}// main ends here

fn add(Data : &Data)->u32{
    Data.no1+ Data.no2
}