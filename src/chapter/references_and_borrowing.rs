pub fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");


    println!("可变引用");
    let mut s = String::from("hello");

    change(&mut s);

    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{}", r1);

    let r2 = &mut s;
    // println!("{}", r1); 在同时创建多个引用之后使用变量将报错
    println!("{}", r2);

    println!("创建悬空引用不被允许。")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}