fn main() {
    println!("Hello, world!");

    let s = String::from("hello world this is a line");
    
    let first_word = first_word(&s);
    let second_word = second_word(&s);
    println!(" first word is {} and second word is {}", first_word, second_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    let mut space_seen = 0;
    let mut start = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && space_seen == 1 {
            return &string[start..i];
        }
        if item == b' ' {
            space_seen = 1;
            start = i;
        }
    }
    &string[start..]
}