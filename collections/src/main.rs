use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    //Vectors

    let mut v: Vec<i32> = Vec::new();
    //add to vector
    v.push(1);
    v.push(2);

    //or
    let v1 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v1[2];
    println!("The third element is {}", third);

    let does_not_exist = v1.get(100);
    match does_not_exist {
        Some(value) => println!("Some"),
        None => println!("None"),
    }

    //immutable iterations
    for i in &v1 {
        println!("{}", i);
    }

    //mutable iterations
    for i in &mut v {
        *i += 100;
        println!("{}", i);
    }

    //HashMaps
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let score = scores.get("Blue");
    match score {
        Some(value) => println!("Blue score {}", value),
        None => println!("Error"),
    }

    let words = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in words.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
