// FUNCTIONS
//
// main() function is entry point of rust.
// Rust provide hoisting of functions means we can call function anywhere in our code.
//
// Expression and Statement:-
// Expressions return a value.
// Statement do not return value.

fn main() {
    hello_world();

    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is: {}", x);

    let y = add(7, 18);
    println!("Value of y: {}", y);
}

fn hello_world() {
    println!("Hello, Rust🦀!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
