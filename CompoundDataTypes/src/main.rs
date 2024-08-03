//Compound Data TYpes
// arrays, tuples, slices, and strings (slice string)

//1) Array a list of elements of the same type
fn main() {
    //[132; 5] -> i32 and 5 is size of array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    //example
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2st element: {}", fruits[1]);
    println!("Fruits Array 3st element: {}", fruits[2]);

    //2) Tuples is a collection of values of different types
    // Here you ca define specific type but without that It'll work too
    let human: (&str, i32, bool) = ("Alice", 30, false); 

    // OR

    //here we used String in place of &str
    // let human: (String, i32, bool) = ("Alice".to_string(), 30, true); 
    // println!("Human Tuple:{:?}", human);

    let my_mix_tuple = ("kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix tuple: {:?}", my_mix_tuple);

    //3) Slices: [1,2,3,4,5,6]

    //examples
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    //here we are using &str -> (string slice)
    let animal_slices:&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("animal_slices: {:?}", animal_slices);

    //here we will convert each "" by using to_string() otherwise it will throw an error
    let book_slices:&[String] = &["IT".to_string(), "Aditya".to_string(), "ZEN".to_string()];
    println!("book_slices: {:?}", book_slices);

    //Strings vs String Slices (&Str)
    //Strings [growable, mutable, owned string type]
    // By default any data-type in rust is immutable

    // 1) Strings
    let mut you_say: String = String::from("Hell, ");
    you_say.push_str("Yes Ser!"); 
    println!("We say: {}", you_say);

    //2) &Str (String Slice)
    // Not an own string but a referance to a portion of 
    // string that is stored somewhere in code
    let string: String = String::from("Hello world!");
    let slice: &str = &string[6..11];
    println!("Slice Value: {}", slice);





}
