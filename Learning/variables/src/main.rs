/// Create date: 16/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino


// Used to hide alerts for unused variables
# [allow(unused_variables)]

// Used to hide alerts of changing values ​​of variables that will not be used later.
# [allow(unused_assignments)]

fn main() {
    let name = "Lucas";                 // String
    let surname = "Lopes";              // unused variable
    let age = 21;                       // i32 is the default when not explicitly declared
    let big_number:i64 = 7128364238764; // explicit declaration
    
    println!("{} {} {}", name, age, big_number);

    // Declare variable mutable with keyword "mut"
    let mut now = 13;

    println!("{}", now);

    // Changing the value of the variable 
    now = 14;
    println!("{}", now);

    // Variable shadowing
    let color:&str = "blue";
    let color:i32 = 54;

    println!("Color is {}", color);
    
    // Declaring multiple variables
    let (x, y, z) = (4, "Hour", 4.7);
    println!("{} {} {}", x, y, z);  
}
