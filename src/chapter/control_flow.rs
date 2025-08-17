use std::thread::sleep;
use std::time::Duration;

// Loops with names
fn loop_name() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    println!("Use while for looping:");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    let mut i = 0;
    for element in a {
        println!("index is: {i}");
        println!("the value is: {element}");
        i += 1;
    }
}

fn for_number_range() {
    for number in (0..4).rev().rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn sleep_secs(seconds: u64) {
    println!("waiting for {seconds} seconds...");
    sleep(Duration::from_secs(seconds));
}

pub fn main() {
    if 3 > 2 {
        println!("3")
    } else {
        println!("2")
    }


    let number = 3;

    if number.eq(&3) {
        println!("number was three");
    }

    // loop {
    //     println!("again!");
    //     sleep(Duration::from_secs(2));
    // }


    // 从循环中返回值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    loop_name();
    sleep_secs(1);
    while_loop();
    sleep_secs(1);
    for_loop();
    sleep_secs(1);
    for_number_range();
}