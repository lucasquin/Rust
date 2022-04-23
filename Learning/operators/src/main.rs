/// Create date: 23/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino

fn main() {
    
    let x = 5 + 10;
    let y:f32 = 10.0 / 3.0;
    let z = 5 - 10;
    let n = 10 % 3;
    let p = 5 * 10;
    // ++ and -- are not suported.
    println!("x = {}, y = {}, z = {}, n = {}, p = {}", x, y, z, n, p);

    // Relational operators: > >= < <= == !=
     println!("{}", x > p); // False

     // Logical operators: && || !
     println!("{}", x > p || z < x); // True

/* 
     Bitwise operators:

     & = Bitwise AND
     | = Bitwise OR
     ^ = Bitwise XOR
     ! = NOT
     << = Left shift
     >> = Right shift
                        */
    println!("{}", 10<<4); // Result = 160
}
