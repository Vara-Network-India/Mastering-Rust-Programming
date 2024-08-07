fn main (){
    //Loop keyword
    //It will continuously run "Hello world"
    // loop {
    //     println!("Hello World");
    // }

    //loop with break
    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter - 100;
        }
    };
    println!("The result is {result}");

    // Example
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

}