use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io;
use std::fs;

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn use_ask_symbol() -> Result<File, Box<dyn Error>> {
    let greeting_file = File::open("./Cargo.lock")?;

    Ok(greeting_file)
}

pub fn main() {
    // panic!("crash and burn");
    let greeting_file_result = File::open("./src/main.rs");

    if !greeting_file_result.is_err() {
        let f = greeting_file_result.unwrap();
        println!("File exists length: {:?}", f.bytes().count());
    }

    // let _greeting_file2 = File::open("hello.txt").expect("File Not Found!");
    println!("\nread file(main.rs) result is: {}", if read_username_from_file("./src/main.rs").is_ok() {"ok"} else {"failed"} );
    println!("\nread file(main.rsx) result is: {}", if read_username_from_file("./src/main.rsx").is_ok() {"ok"} else {"failed"} );

    println!("{:?}", use_ask_symbol());

}