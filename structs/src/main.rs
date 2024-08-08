#![allow(warnings)]
//structs

fn main (){
    //tuple
    let rect: (i32,i32) = (200, 500);

    //struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: String,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@m.com"),
        sign_in_count: 1,
    };

    //changing initial email to this one
    user1.email = String::from("axyz@m.com");
    println!("User email is: {}", user1.email);

    //Return a struct from a function
    // -> User it means we are returning user struct from build_user function
    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    //Create instances from other instances
    let user2: User = User{
        email: String::from("another@m.com"),
        ..user1 //it means there is no change in other fields of user1
    };

    //Tuple Structs
    struct Color(i32, i32, i32);
    struct POint(i32, i32, i32);

    let black: Color = Color(0,0,0);
    let white: Color = Color(255,255,255);

    //Unit-like struct
    struct AlwaysEqual;
    let subjet: AlwaysEqual = AlwaysEqual;
}