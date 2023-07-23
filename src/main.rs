fn main() {
    // Declare a mutable variable
    let mut x: i32 = 5;
    // Print the value of x
    println!("The value of x is {}", x);
    // reassign the value of x
    x = 6;
    // print the new value of x
    println!("The value of x is  {}", x);
    // declare a constant
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);
    // let us  explain shadowing in rust
    let y = 578_900;
    let y = y + 1;
    let y = y * 5;
    println!("The value of y is {}", y);
    let spaces = "   ";
    println!("The value of spaces is {}", spaces.len());
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);
    // floating point types
    let float = 2.7;
    println!("The value of float is {}", float);
    // f32 tuple
    let float32: f32 = 5.5;
    println!("The value of float32 is {}", float32);
    let chaos = false;
    println!("The value of chaos is {}", chaos);
    // creating a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x,y,z is {}, {}, {}", x, y, z);
    // create an array
    let arr: [i32; 4] = [1, 2, 3, 4];
    // print the value of arr
    println!("The value of arr is {}", arr[2]);
    // Declare sum function
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    println!("The value of sum is {}", sum(2, 3));
    
}
