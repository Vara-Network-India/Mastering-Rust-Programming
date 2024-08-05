//functions
// Entry point
// an function / variables should be written in snake case
// snake case: hello_world
// kebab case: hello-world



// Hoisiting - can call functions anywhere in your code
fn main(){
    hello_world();
    tall_height(173);
    human_id("Vara", 2, 167.5);
    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // no need to use ; here it will already evaluate it till last line
                    // so already it will evaluate to (50)
    };
    println!("My x value is: {}", _x);
    let result:i32 = add(4, 3);
    println!("Result is: {}", result);
  
    let _result:i32 = 7;
    println!("Result is: {}", _result);

    let mut _inc_number: i32 = 0;
    for _ in 0..5 { // Loop 5 times
        _inc_number += 4; // Increment the number
    }
    println!("My result is: {}", _inc_number);

}

// functions returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}



// IMPORTANT
// Any variable that is declared outside of the main variable
// we can't use let we have to use (const or static) in global variable
// Example -
// const _X: () = {
    //code

// };


fn hello_world(){
    println!("Hello, Rust!")
}

//you can insert input values
fn tall_height(height: u32){
    println!("My height: {} cm", height);
}

//you can insert more than one value
// u -> unassigned integer
// f -> floats
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is: {}, I am {} years old, an my height is {} cm", name, age, height)
}

//Expressions and Statements
//Expression: Anything that returns a value.
//statements: Anything that does not return a value.

// Expression
//--------------------------------------------------------
// 5
// true & false
// add(3,4)
// if condition {value1} else {value2}
// ({code})

// Statements
// -------------------------------------------------------
// almost all statements in rust end with ;
// let y = let x = 10;
// 1) Variable definitions: fn foo() {}
// 2) Functions definitions: fn foo() {}
// 3) Control flow  statements: if condition {code}
// else {code}, while condition {code} etc.