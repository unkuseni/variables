fn main() {
    // Create a vector with initial values
    let vec = vec![1, 2, 3, 4];
    // Increment and print each value in the vector
    for  element in &vec {
let incr = *element + 1;
        println!("Increment was: {}", incr);
        
    }
}