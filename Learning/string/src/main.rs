/// Create date: 16/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino


// Used to hide alerts for unused variables
# [allow(unused_variables)]

// Used to hide alerts of changing values ​​of variables that will not be used later.
# [allow(unused_assignments)]

fn main() {
    let dog: &str = "Kriss";
    let tatu: &'static str = "Fuleco";
    println!("{}", dog);
    println!("{}", tatu);
}
