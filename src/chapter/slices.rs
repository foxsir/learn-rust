fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn main() {
    let word = String::from("h e y");

    let w = first_word(&word);
    println!("{}", w);
    println!("{}", word);

    let s = String::from("你好");
    println!("{}", &s[..3]);


}