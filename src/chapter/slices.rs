fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..s.len()]
}

pub fn main() {
    let word = String::from("he y");

    let w = first_word(&word[0..]);
    println!("first word: {}", w);

    let s = String::from("你好");
    println!("{}", &s[..3]);

}