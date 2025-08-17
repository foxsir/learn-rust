pub fn main() {
    let s = String::from("hello");

    {
        let s = "hello";
        let mut s = s.replace("h", "H");
        s.push_str(" world");
        println!("{s}");
    }

    println!("{s}");

    let mut s = String::from("hello");

    s.push_str(", world!");


    // move ownership
    let mut s1 = String::from("hello");
    println!("{}", s1);
    s1.push_str(", world!");


    println!("所有权实际同时只能属于一个属主，属主的转移实际是指针的转移，存储在栈上的数据例外，因为栈上的数据是明确的，复制成本低廉的，所以可以直接复制。");
    println!("将变量传递给函数，有着和变量赋值一样的行为，将导致所有权转移");
}