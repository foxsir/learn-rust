fn variables() {
    println!("***Variables***");
}

fn mutability_of_variables() {
    let var = "Variables";
    println!("{var}");
    let var = &*vec!["is a ", var].join("");
    println!("{}", var);
    let var = &*format!("Hello, {}! You are {} years old.", var, "age");
    println!("{}", var);
}

fn constant_value() {
    const CONS: u32 = 60 * 60 * 3;
    println!("CONS is constant: {CONS}");
}

fn shadowing() {
    println!("\n\nvariables shadowing");
    let x = 100;
    const CONS: u32 = 60 * 60 * 3;

    {
        let x = "partial variable";
        const CONS: u32 = 60 * 60 * 3;
        println!("{}", x);
        println!("x len: {}", x.len());
        println!("{}", CONS);
    }

    println!("{}", x);
    println!("{}", CONS);
}

pub fn main() {
    variables();
    mutability_of_variables();
    constant_value();
    shadowing();
}