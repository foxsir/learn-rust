fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, B> {
    x: T,
    y: B,
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest::<i32>(&number_list);
    println!("The largest number is {result}");

    let integer = Point::<String, i32> { x: String::from("Word"), y: 100 };


    println!("{}", integer.x);
    println!("{}", integer.y);
}