/// Create date: 26/04/2022
/// Project: Learning the Rust language
/// by Lucas Lopes Quintino

fn main() {
    
    let pair = [2, 4, 6, 8, 10];
    let odd : [f32; 3] = [1.0, 3.2, 9.9];
    println!("{:?} {:?}", pair, odd);
    
    let numbers = [0;15];
    println!("{:?}", numbers);
   
    const DEFAULT : f64 = 0.0;
    let mut num = [DEFAULT;12];
    println!("{:?}", num);
    
    num[3] = 4.0;

    println!("{:?}", num[3]);
    println!("{:?}", num);
    
    for i in num.iter(){
        println!("{}", i);
    }
}
