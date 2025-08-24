struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn create_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 10,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub(crate) fn main() {
    let mut user = create_user("rust", "rust@qq.com");

    println!("user name: {}", user.username);
    user.email = String::from("python@qq.com");
    println!("user email: {}", user.email);
    println!("user sign_in_count: {}", user.sign_in_count);
    println!("user active: {}", user.active);

    println!("{:?}", area(&Rectangle { width: 10, height: 10 }));

    let rect = Rectangle { width: 10, height: dbg!(100 * 100) };

    println!("{rect:#?}");
    dbg!(&rect);
    println!("{}", dbg!(100 * 100));

    let u: u32 = dbg!(100 * 100).to_string().parse().unwrap();
    println!("{}", u);


    println!("{}", rect.area());
}