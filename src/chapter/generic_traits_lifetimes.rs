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

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Display {
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for Point<String, i32> {
    fn summarize(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

impl Display for Point<String, i32> {}

fn notify<T: Summary + Display>(item: &T) {
    println!("{}", item.summarize2());
}

// 使用where范型
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone,
{
    return 100;
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest::<i32>(&number_list);
    println!("The largest number is {result}");

    let integer = Point::<String, i32> { x: String::from("Word"), y: 100 };

    println!("{}", integer.summarize());

    notify(&Point::<String, i32> { x: String::from("Word"), y: 100 });

    println!("{}", integer.x);
    println!("{}", integer.y);
}