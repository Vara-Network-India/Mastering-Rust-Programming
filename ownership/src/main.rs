// Ownership, Borrowing and References

// Ownership
//--------------

// C, C++ _> Memory management control issue
// Garbage collector solved this issue, but creating a new issue -> slow performance
// [stopping/Resuming the program]

//-----------------------------------------------------------------------------------
// Ownership introduced by rust to solve memory safety issues and high performance at the same time
// What is Ownership?
// Every value has a single owner [every variable has one value annd it is its sole owner]

//------------------------------------------------------------------------------------
//RULES
// 1) Each value in Rust has owner
// 2) There can be only one owner at a time
// 3) When the owner goes out off scope, the value will be dropped

// Example 1: Each value in Rust has owner
fn main(){
    let first_letter: String = String::from("its me");
    let referance_one = calculate_length(&first_letter);
    println!("My letter {} has: {} letter", first_letter,referance_one);
}

fn calculate_length(first_letter: &String) -> usize {
    first_letter.len()
}


