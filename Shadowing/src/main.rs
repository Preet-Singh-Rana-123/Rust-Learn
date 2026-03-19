// SHADOWING
// It means when we declare new variable with same name and replaces it with previous one.
//
// Its not same as mutability because here we reasigning previous variable with new new one with
// same name.

fn main() {
    let x: i32 = 10;
    println!("Value before shadowing: {}", x);
    let x: i32 = x + 5;
    println!("Value after shadowing: {}", x);
}
