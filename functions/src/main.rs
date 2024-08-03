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
}

fn hello_world(){
    println!("Hello, Rust!")
}

//you can insert input values
fn tall_height(height: u32){
    println!("My height: {} cm", height);
}

//you can insert more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is: {}, I am {} years old, an my height is {} cm", name, age, height)
}