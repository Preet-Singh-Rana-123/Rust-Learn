// OWNERSHIP
// OWNERSHIP is inntroduced by rust to solve memory safty issues and high performance at a same
// time.
// OWNERSHIP RULES:-
// 1. Each value in Rust has a variable that's its owner.(Each value has a single owner)
// 2. There can be only one owner at time.
// 3. When owner goes out of scope, the value will be dropped.

fn main() {
    let s1: String = String::from("Preet");
    let len = calculate_lenght(&s1); // If we see here we only give refence of s1 to the function
    // not its full ownership. This means len does not have ownership of s1.
    println!("The length of string '{}' is {}.", s1, len);

    let s2 = String::from("Hello, Rust!");
    let s3 = s2;
    // println!("{}", s2);  // This line give error because now s2's owner is s3 and we can no
    // longer use s2 and its permanently deleted from memory.
    println!("{}", s3);
}
// Now s1 and s3 both are out of scope and its value is dropped.

fn calculate_lenght(s: &String) -> usize {
    s.len()
}
