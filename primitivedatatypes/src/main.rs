//Primitive data types
//int,float, bool, char

//1)
//------------------------------------------------------------------
//Integer
// Rust has signed (+ and -) and Unsigned integer (only+)
// types of different sizes
// i8, i16, i32, i64, i128: Signed integers.
// u8, u16, u32, u64, u128: Unsigned integers.

// u32-> Unsigned integers can only represent positive numbers)  0 to 4294967295
// i32 -> Assigned integers represent both positive and negative Range: -2147483648 to 2147483647

fn main() {
    let x: i32 = -41;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    //Difference between i32 (32 bits) and i64(64 bits)
    //range : i32
    // i32 -> 2147483647
    // u64 -> 18446744073709551615

    let z: i32 = 2147483647;
    let k: u64 = 18446744073709551615;
    println!("Signed Integer: {}", z);
    println!("Unsigned Integer: {}", k);

    //IMP -> Values more than this will give error
    // Example 
    // let z: i32 = 2147483648;
    // let k: u64 = 18446744073709551619;

//2)
//------------------------------------------------------------------
//floats [Floating Point Types]
//f32, f64
let _pi: f64 = 3.14;
println!("Value of pi: {}", _pi);

//3)
//-------------------------------------------------------------------
//Boolean Values: true, false
let is_snowing: bool = true;
println!("Is it snowing ? {}", is_snowing);

//4)
//------------------------------------------------------------------
//Chacracter Type - char
let letter: char = 'a';
println!("My character type: {}",letter)
}
