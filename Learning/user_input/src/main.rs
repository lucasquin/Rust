/// Create date: 16/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino

// Import library
use std::io;

fn main() {

    // Create variable that we can change
    let mut input = String::new();

    // Print Say something in the screen
    println!("Say something");

    // Receving the read_line from the standard in library
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("You said {}", input);
        }
        Err(e) =>{
            println!("Something went wrong {}", e);
        }
    }
}
