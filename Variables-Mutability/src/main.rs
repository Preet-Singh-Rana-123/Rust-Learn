// VARIABLES AND MUTABILITY:-
// In Rust variables are immutable by default.
// So to make a variable mutable we use 'mut' keyword.
//
// CONSTANTS:-
// In constants we cannot use 'mut' keyword, which means we cannot change constants.

fn main() {
    let mut a: i32 = 5;
    println!("Value of a: {}", a);
    a = 10;
    println!("Value of a: {}", a);
}
