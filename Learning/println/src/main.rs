/// Create date: 16/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino

fn main() {

    // Examples of how to use the println!
    println!("Hello, world!");   
    println!("My name is {} and I'm {} years old", "Lucas", 21);
    
    // Use of the arithmetic expressions
    println!("a + b = {}", 4 + 5 * 9);

    println!("{1} has three {0} and no {2}. {1} loves {0}.", "dogs", "Lucas", "cats");
    println!("{name} {surname}", surname = "Lopes", name = "Lucas");
    println!("binary {:b}, hexa {:x} and octa {:o}", 50, 50, 50);

    // Here basically transform the array of integers in a string with operator :?
    println!("Array: {:?}", [1, 2, 3, 52]); // without ":?" gets the error: `[{integer}; 4]` cannot be formatted with the default formatter
}