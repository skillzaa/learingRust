#[allow(dead_code)]
pub fn run(){
    let a:i32 = 99;
    let b:bool = true;
    let c:char = 'c';
    println!("hello {} , {} , {}",a,b,c);
    //The type for Binary Hex and Octo
    println!("Binary: {:b} ,Hex: {:x} , Octo:{:0}",10,10,10);

    //Debug trait is used to print entire array etc 
    //Here we will print out a tuple
    println!("Print a tuple {:?}",(12,true,9.00));

}