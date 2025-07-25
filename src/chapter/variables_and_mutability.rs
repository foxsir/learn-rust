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
    print!("CONS is constant: {CONS}");
}

pub fn main() {
    variables();
    mutability_of_variables();
    constant_value();
}