fn main() {
//This is tubple   
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}
//This function gets a tuple.
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}