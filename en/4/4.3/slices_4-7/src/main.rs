fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let s = String::from("Hello, world");
    println!("{s} : {}", first_word(&s));

    let s = String::from("Hello,");
    println!("{s} : {}", first_word(&s));
}
