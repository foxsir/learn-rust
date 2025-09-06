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
    collections::vector_of_collection as collections_main,
    collections
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

    // inner use "use keyword"
    use crate::chapter::package_crates;
    println!("\nPackages and Crates\n");
    println!("called hosting::add_to_waitlist{}", package_crates::main());
    // let pc = package_crates::hosting::add_to_waitlist();
    // println!("{}", pc)

    println!("\nCommon Collections\n");
    println!("use as keyword");
    collections_main();
    collections::string_of_collection();
    collections::hashmap_of_collection();
}