use firstword::first_word;

fn main() {
    let s = String::from("hello, world");

    let word = first_word(&s); // word will get the value 5

    println!("The first word is: {word}");
}
