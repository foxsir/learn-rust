use std::collections::HashMap;

fn print_type<T>(_: &T) {
    println!("Type name: {}", std::any::type_name::<T>());
}


pub fn vector_of_collection() {
    let mut v: Vec<i32> = Vec::new();
    v.insert(0, 1);
    v.insert(1, 2);
    v.insert(2, 3);
    v.push(4);

    println!("Get Vector values.");
    // println!("{v:?}");
    // println!("{}", &v[0]);
    // println!("{}", &v[1]);
    // println!("{}", &v[v.len() - 1]);
    // println!("{}", &v.last().unwrap());
    // println!("{:?}", &v.last());
    // println!("{:?}", &v.first());
    // println!("{:?}", &v.get(100).expect("undefined index"));
    //
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    println!("use iterator\n");
    v.iter().for_each(|x| {
        println!("{}", x)
    });

    println!("use for-in\n");
    for x in v {
        println!("{}", x);
    }

    let v = vec![1, 2, 3];
    println!("{v:?}");


    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

pub fn string_of_collection() {
    let data = "initial contents";
    println!("{:?}", data);
    print_type(&data);

    let s = data.to_string();
    println!("{:?}", s);
    print_type(&s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);
    println!("{}", String::from("hello world"));

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    let s1 = String::from("你hi");
    let h = &s1[0..3];
    println!("{}", h);
}

pub fn hashmap_of_collection() {
    let mut map = HashMap::from([(String::from("a"), 100), (String::from("b"), 200)]);

    println!("{:?}", map.get(&String::from("a")).is_some());
    println!("{:?}", map.get("c").unwrap_or(&100));


    for (key, value) in &map {
        println!("{key}: {value}");
    }

    map.insert(String::from("a"), 200);
    println!("{:?}", map.get(&String::from("a")));

    println!("{:?}", map.entry(String::from("b")));
    let x = map.entry(String::from("b"));
    x.insert_entry(300);
    println!("{:?}", map.get(&String::from("b")));


    println!("根据旧值更新值");
    let text = "hello world wonderful world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        println!("{}", word);
        *count += 1;
    }
    println!("{map2:?}");




}