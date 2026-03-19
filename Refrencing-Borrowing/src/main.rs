// Some Common Memory Saftey Errors:-
// 1. Use After Free:- Using memory after it has been freed.
//
//    let s1 = String::from("hello");
//    let s2 = s1;
//    println!("{}", s1); // ❌ compile error
//
// 2. Dangling References:- Reference pointing to a memory that does not exists.
//
//    fn dangle() -> &String {
//        let s = String::from("hello");
//        &s // ❌ returning reference to dead data
//    }
//
// 3. Double Free:- Freeing the same memory twice.
//
// 4. Buffer Overflow:- Accessing memory outside an array.
//
// 5. Uninitialized Memory:- Using memory before giving it a value.
// ------------------------------------------------------------------------------------
// Since Rust does not allow multiple owner of a value so we use the concept of reference and
// borrowing.
// References allow us to borrow value without taking ownership.
// References are of two types:-
// 1. Mutable References
// 2. Imutable References
//
// We can create References by adding '&' sign before variable we want to borrow.
//
// ***NOTE*** -- we can have one Mutable Reference for a variable or we can have multiple Imutable
// References, but we can not have multiple Mutable References and Mutable and Imutable References
// at a same time for a variable.

fn main() {
    let mut x = 5;
    let r = &mut x;

    *r += 3;

    println!("Value of x: {}", x);
}
