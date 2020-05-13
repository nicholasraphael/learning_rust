fn main() {
    let s1 = String::from("hello");

    takes_ownership(s1);

    let x = 5;

    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("world");
    let s2 = takes_and_gives_back(s2);
    println!("{}", s2);

    let len = calculate_length(&s2);
    println!("The length of {} is {}", s2, len);
}

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn makes_copy(int: i32) {
    println!("{}", int);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(string: String) -> String {
    string
}

fn calculate_length(string: &String) -> usize {
    string.len()
}

fn no_dangle() -> String {
    let s = String::from("");

    s
}