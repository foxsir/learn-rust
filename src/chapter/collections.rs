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

}