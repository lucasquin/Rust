/// Create date: 24/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino

// Importing module hello
mod hello;

fn main() {
    let name = "Lucas L";
    hello::say_hello(name);
    hello::say_hello("Batman");
}
