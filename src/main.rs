mod chapter;

use crate::chapter::{control_flow, functions, ownership, references_and_borrowing, slices};
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

    println!("Ownership\n");
    ownership::main();

    println!("References and Borrowing\n");
    references_and_borrowing::main();

    println!("Slices Type\n");
    slices::main();
}