

//Referances and Borrowing
// Safety and perormance
//Borrowing and referance are powerful concepts

// Understanding Referances
// Referances: Enable you to borrow values without taking ownership
// Immutable Referance.
// Mutable Referance.
// Create refferance by add "&";
// -I- Immutable Referance

//------------------------------------------------------------------------------
fn main(){
let mut _x = 5;
let mut _r = &mut _x;
*_r += 1;
println!("Value is x: {}",_x);
}