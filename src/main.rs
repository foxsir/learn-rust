mod chapter;

use crate::chapter::{
    variables_and_mutability,
    data_type,
    control_flow,
    functions,
    ownership,
    references_and_borrowing,
    slices,
    structure
};

fn main() {
    println!("\nVariables and Mutability\n");
    variables_and_mutability::main();

    println!("\nData Types\n");
    data_type::main();

    println!("\nData Types\n");
    functions::main();

    println!("\nControl Flow\n");
    control_flow::main();

    println!("\nOwnership\n");
    ownership::main();

    println!("\nReferences and Borrowing\n");
    references_and_borrowing::main();

    println!("\nSlices Type\n");
    slices::main();

    println!("\nStructs\n");
    structure::main();
}