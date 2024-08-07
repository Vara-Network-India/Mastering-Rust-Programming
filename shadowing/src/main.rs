
//IMPORTANT
//shadowing -> initial variable is shadowed by second (new) variable
// Shadowing is not same as marking a variable as mutable.
//we are changing the value but using same {x} name

fn main() {
    let x = 5;
    //initial value was 5

    let x = x + 1;
    //now we old x is shadowed by new x now value is 6

    let x 
      
    {
    let x = x* 2; //result is 12
    println!("The value of x in inner scope is: {x}");
    }

    println!("Value of x in main function is: {x}");
}


