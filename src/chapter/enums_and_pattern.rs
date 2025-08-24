use std::net::IpAddr::V4;

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn get_teen() -> u8 {
    10
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => get_teen(),
        Coin::Quarter => {
            10
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => {
            None
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter = coin else {
        return None;
    };

    Some(String::from("quarter"))
}

pub fn main() {

    let v4 = IpAddrKind::V4(String::from("127.0.0.1"));
    println!("address: {:#?}", v4);

    println!("address: {:#?}", IpAddrKind::V6(String::from("::1")));

    dbg!(V4("127.0.0.1".to_string().parse().unwrap()));

    println!("\nMatch flow\n");

    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter));


    println!("{:?}", plus_one(Some(5)));
    println!("{:?}", plus_one(None));
    println!("{:?}", plus_one(Some(100)));

    println!("\nif let and else let\n");

    println!("{:?}", describe_state_quarter(Coin::Dime));

}
