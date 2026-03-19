// Compound Data Types:-
//
// 1. Arrays
// 2. Tuples
// 3. Slices
// 4. Strings

fn main() {
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruit Array: {:?}", fruits);
    println!("Fruit Array 1st element: {}", fruits[0]);

    // Tuples
    let human: (&str, i32, bool) = ("Preet", 21, true);
    println!("Human Tuple: {:?}", human);
    let mix_tuple = ("Preet Rana", 21, [1, 2, 3, 4, 5]);
    println!("Mix Tuple: {:?}", mix_tuple);

    // Slices: Just like array but they are Dynamically sized
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);

    // String VS string slices (&str)
    // String are mutable and Dynamic in size. Also they are owned string types. They are very slow
    // to acces
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says: {}", stone_cold);

    // &str or string slices are not owned string but they are reference to other string. Also they
    // are imutable and have specific size.
    let string: String = String::from("Hello World");
    let string_slice: &str = &string[0..5];
    println!("String Value: {}", string_slice);
}
