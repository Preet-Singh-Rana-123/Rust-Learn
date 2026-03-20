fn main() {
    let age: u16 = 18;
    if age >= 18 {
        println!("You can drive car.");
    } else {
        println!("You cannot drive car.")
    }

    let condition = false;
    let number = if condition == true { 5 } else { 6 };
    println!("{}", number);
}
