// Primitive Data types in rust:- int, float, char, bool
//
// INTEGER:-
// Rust has signed(- and +) and unsigned integer(only +) types of different sizes.
// i8, i16, i32, i64, i128:- Signed Integer
// u8, u16, u32, u64, u128:- Unsigned Integer
//
// FLOAT:- Contain decimal values.
// f32, f64
//
// BOOLEAN:- true, false
//
// CHARACHTER:- char

fn main() {
    let x: i32 = -42;
    let y: u32 = 100;
    println!("Signed Integer: {}", x);
    println!("Unigned Integer: {}", y);
    let pi: f32 = 3.14;
    println!("Value of PI: {}", pi);
    let is_loggedin: bool = true;
    println!("Is user logged in: {}", is_loggedin);
    let letter: char = 'a';
    println!("First letter of alphabet is: {}", letter);
}
