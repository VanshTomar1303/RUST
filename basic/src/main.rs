
// fn -> keyword
// main -> function name
fn main() {
    // println! -> to print anything
    println!("Hello, world!");

    // variable
    let a = 10; // can't be changed
    let mut b = 10; // can be changed
    const HEART: i32 = 20; // constant

    // types
    let d: i32 = 34; // for integer i32,u32
    let e: f64 = 40.8; // float
    let character: char = 'C'; // character
    let string: &str = "Hello"; // string
    
    // printing with variable
    println!("Age: {}",a);

    // shadowing
    let x = 10;
    let mut x = x; // Temporarily mutable
    x += 5;
    let x = x;     // Permanently immutable again

    println!("{}",x)
}
