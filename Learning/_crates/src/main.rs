/// Create date: 25/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino

// Original name
// use crate::archive::arch::arch_file;

// Using alias to rename function
use crate::archive::arch::arch_file as arc;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

// Importing my function
mod archive;

fn main(){
    
    // Name without alias  
    // arch_file("Naruto");
    arc("Naruto");
    
    // Generate a random number in range 0 to 3.
    let mut nu = rand::thread_rng();
    let x: i32 = nu.gen_range(0, 3);
    println!("Random number: {}", x);
    
    // Calling find_lucas
    find_lucas();
}

fn find_lucas(){
    
    let mut x = false;
    
    while x == false {
       
        // Generate alphanumeric random strings with three letters
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(3)
            .collect();
   
        println!("{}", rand_string);

        if rand_string == "luc"{
            print!("{} finded!", rand_string);
            x = true;
        }
    }
}
