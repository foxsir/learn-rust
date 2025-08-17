use std::hash::{DefaultHasher, Hash, Hasher};
use std::io;

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn _read_line() {
    println!("Please enter a line of text:");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("x");

    println!("read line: {line}");
}

pub fn main() {
    println!("\nThe rust functions");


    println!("\ncalculate string hash value");
    let mut hasher = DefaultHasher::new();
    "x".hash(&mut hasher);
    println!("{:?}", hasher.finish());

    another_function(10);
    // read_line();




}