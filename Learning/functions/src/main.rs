/// Create date: 23/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]

fn main() {

    let name: &str = "Lucas";
    let mut surname: &str = "";
    let hi = say_hi(name);
    
    say_hello(name);
    say_surname(name, &mut surname);
    println!("{}", hi);
}

fn say_hello(name: &str){
    println!("Hello, {} ", name);
}

fn say_surname(name: &str, surname: &mut &str){
    
    *surname = "Lopes";
    println!("{} {}", name, surname);
}

fn say_hi(name: &str) -> String {
    let hi: String = format!("Hi, {}", name);
    return hi;
}
