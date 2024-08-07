//Constants

fn main() {

    // const globals cannot be mutable

    let mut x = 5;
    println!("The value of x is: {}", x);
    const y: i32 = 10;
    println!("The value of y is: {}", y);
    println!("The value of pi is: {}", PI);
    println!("The value of Three hour in seconds is: {}s", three_hours_seconds);

     // you can declare constant with a type annotation
     const PI: f64 = 3.1415;
     const three_hours_seconds: u32 = 60*60*3;

}
