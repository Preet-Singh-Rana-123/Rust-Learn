// ERROR HANDLING
// There are two techniques to handle errors in rust:-
// 1. Option<T>:-
//    enum Option<T>{ // Define generic Option type
//        Some(T), // Represent a presence of value
//        None,    // Represent a absence of value
//    }
//
//  2. Result<T,E>:-
//     enum Result<T,E>{  // Define generic Result type
//         Ok(T),   // Represent a value
//         Err(E),  // Represent a error
//     }

fn main() {
    let result = divide_option(10.0, 2.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

    let result2 = divide_result(100.0, 5.0);
    match result2 {
        Ok(x) => println!("Result: {}", x),
        Err(err) => println!("Error: {}", err),
    }
}

fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}
