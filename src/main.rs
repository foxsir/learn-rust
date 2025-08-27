mod chapter;

use crate::chapter::{
    variables_and_mutability,
    data_type,
    control_flow,
    functions,
    ownership,
    references_and_borrowing,
    slices,
    structure,
    enums_and_pattern,
    package_crates,
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

    println!("\nEnums and Pattern\n");
    enums_and_pattern::main();

    println!("\nPackages and Crates\n");
    println!("called hosting::add_to_waitlist{}", package_crates::main());
    // let pc = package_crates::hosting::add_to_waitlist();
    // println!("{}", pc)


}