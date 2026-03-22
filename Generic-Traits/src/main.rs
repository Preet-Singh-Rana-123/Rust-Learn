// GENERICS AND TRAITS
//
// Generics:-
// We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
//
// Traits:-
// A trait defines the functionality a particular type has and can share with other types
// Think trait like abstract function that can be share to other structs.

#![allow(warnings)]
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

// Function Generics
fn largest<T: PartialOrd + Copy>(number_list: &[T]) -> T {
    let mut largest = number_list[0];
    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// Struct Generics
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Traits
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
