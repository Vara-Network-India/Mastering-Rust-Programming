// If else [If expression] [else expression]
// if conditons are met run (If) otherwise run else

fn main(){

    // Example - 1
    let age: u16 = ;
    if age >= 18 {
        println!("You can drive");
    } else{
        println!("You can't drive");
    }

    //Example - 2
    //Multiple conditions with else if:
    let number = 6;
    if number % 4 == 0{
        println!("Number is divisible by 4");
    }else if number % 3 == 0{
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Not divisible by any of number listed above")
    }

    //Example 3
    let condition = true;
    let number = if condition {5} else {10};
    println!("Number: {number}")
}