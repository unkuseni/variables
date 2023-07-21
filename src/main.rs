fn main() {
    // Declare a mutable variable
    let mut x: i32 = 5;
    // Print the value of x
    println!("The value of x is {}", x);
    // reassign the value of x
    x = 6;
    // print the new value of x
    println!("The value   of x is  {}", x);
    // declare a constant
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);
    // let us  explain shadowing in rust
    let y = 578_900;
    let y = y + 1;
    let y  = y * 5;
    println!("The value of y is {}", y);
}
