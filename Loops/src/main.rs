fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);

    let mut number = 5;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("Next Loop!");

    let array: [i32; 5] = [10, 20, 30, 40, 50];
    for element in array {
        println!("{element}");
    }
}
