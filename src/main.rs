fn main() {
       let original_string = String::from("Hello, World!");
   
       let transferred_string = transfer_ownership(original_string);
   
       // The original_string is no longer valid here, as ownership has been transferred
       // to transferred_string
   
       println!("Transferred string: {}", transferred_string);
   }
   
   fn transfer_ownership(mut string: String) -> String {
       string.push_str(" This is transferred chaos!");
       string
   }
