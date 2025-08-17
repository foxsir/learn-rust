mod chapter;

use crate::chapter::{control_flow, functions};
use self::chapter::variables_and_mutability;
use self::chapter::data_type;

fn main() {
    println!("Variables and Mutability\n");
    variables_and_mutability::main();

    println!("Data Types\n");
    data_type::main();

    println!("Data Types\n");
    functions::main();

    println!("Control Flow\n");
    control_flow::main();
}