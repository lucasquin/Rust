/// Create date: 16/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino


// Used to hide alerts for unused variables
# [allow(unused_variables)]

// Used to hide alerts of changing values ​​of variables that will not be used later.
# [allow(unused_assignments)]

fn main() {

    // String slices are immutable
    let dog: &str = "Kriss";
    let br: &'static str = "Brasil";


    // String objects are mutables
    let mut cat = String::new();
    let country = String::from(br);

    println!("{}", dog);
    println!("{}", br);
    println!("{}", country);

    // format!
    let city = format!("The city is {}, located in the state of Goiás, {}.", "Goiânia", country);
    println!("{}", city);

    // String methods
    println!("{}", city.len());   // Count caracters        
    cat.push('c');                // .push only char
    cat.push_str("at");          // .push_str string
    println!("{}", cat);
}
