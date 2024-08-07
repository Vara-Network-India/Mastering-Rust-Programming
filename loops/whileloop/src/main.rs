// While loop runs till conditions are met

fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}"); // This prints the current value of `number`.
        number -= 1; // This decrements `number` by 1.
                     // break;  // This immediately exits the loop, so only one iteration occurs.
    }
    println!("HEYY"); // This is printed after the loop exits (which happens immediately).



    // Example
    // Looping through collection of loops
    let a = [1, 2, 3, 4, 6];
    let b = ["a", "b", "c", "d", "e"];
    for element in a {
        println!("{element}")
    }
    for element in b{
        println!("{element}")
    }
}
