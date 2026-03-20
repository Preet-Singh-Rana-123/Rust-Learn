// STRUCTS
// Structs are used to name and package related value similar to tuples.

#![allow(warnings)]
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("preet_rana"),
        email: String::from("preetrana1263@gmail.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("preet123@gmail.com"),
        ..user1
    };

    println!("User 2 name: {}", user2.username);

    let black = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
}

struct Book {
    title: String,
    author: String,
    pages: u32,
    avilable: bool,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
