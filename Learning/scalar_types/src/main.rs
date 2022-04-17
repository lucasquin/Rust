/// Create date: 16/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino


// Used to hide alerts for unused variables
# [allow(unused_variables)]

// Used to hide alerts of changing values ​​of variables that will not be used later.
# [allow(unused_assignments)]

fn main() {

    let var:f32 = 4.0; // Declaring the value f32 without the dot causes the mismatch error
    println!("{}", var);

    let million = 1_000_000; // The underscore is a number separator for easily reading
    println!("{}", million);


    // Boolean values
    let is_day = true;
    let is_night = false;
    println!("{}", is_day);

    // Char is declared with single quotes
    let letter = 'A';
    println!("{}", letter);

    // Char emote
    let emote = '\u{1F601}';
    println!("{}", emote);
}
