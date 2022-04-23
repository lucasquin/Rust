/// Create date: 23/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let name: &str = "Lucas";
    say_hello(name);
    println!("{}", name);
}

fn say_hello(name: &str){
    println!("Hello, {} ", name);
}
