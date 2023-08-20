use std::collections::HashMap;
fn main() {
    // Create a vector with initial values
    let mut vec: Vec<i32> = vec![1, 2, 3, 4];
    // Increment and print each value in the vector
    for  element in &mut vec {
let incr: i32 = *element + 1;
        println!("Increment was: {}", incr);
        
    }
    let numbers: Vec<i32> = vec![1, 2, 3];
    let letters: Vec<char> = vec!['a', 'b', 'c'];

    let zipped: HashMap<&i32, &char> = numbers.iter().zip(letters.iter()).collect();

    for (number, letter) in &zipped {
        println!("Number: {}, Letter: {}", number, letter);
    }
    let scores: HashMap<&i32, &char> = zipped;
    println!("Scores: {:#?}", scores);
    let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);
// Next line won't compile because `String` doesn't implement `Copy`
println!("Field name: {}, Field value: {}", field_name, field_value);
}
