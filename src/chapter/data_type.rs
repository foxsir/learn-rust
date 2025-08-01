pub fn main() {
    println!("====data type====");
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("解析字符串为（u32）数字: {}", guess);

    println!("使用_符号表示千分位");
    println!("{}", 1_000);
    println!("{}", 1_000_000);

    println!("十进制：{}", 98_222);
    println!("十六进制：{}", 0xff);
    println!("八进制：{}", 0o10);
    println!("字节 a：{}", b'a');

    println!("\n浮点类型");
    println!("专为浮点数：{}", (10f32));

    println!("\n数字运算");
    println!("addition: 5 + 10 = {}", 5 + 10);
    println!("subtraction: 95.5 - 4.3 = {}", 95.5 - 4.3);
    println!("multiplication: 4 * 30 = {}", 4 * 30);
    println!("division: 56.7 / 32.2 = {}", 56.7 / 32.2);
    println!("division: -5 / 3 = {}", -5 / 3);
    println!("remainder: -43 % 5 = {}", 43 % 5);

    println!("\nchar类型");
    println!("\nℤ: {}", 'ℤ');
    println!("\n😊: {}", '😊');
    println!("\n你: {}", '你');
    println!("\n好: {}", '好');

    println!("\n复合类型");
    println!("\n元组类型");
    let tup: (i32, f64, String) = (500, 6.4, String::from("1"));
    println!("{:?}", tup);
    let tup: (i32, f64, &str) = (500, 6.4, "1");
    println!("{:?}", tup);

}