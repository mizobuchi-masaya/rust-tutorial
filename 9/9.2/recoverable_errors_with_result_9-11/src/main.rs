fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    println!("{:?}", last_char_of_first_line(""));
    println!("{:?}", last_char_of_first_line("Hello, world!Fine."));
    println!("{:?}", last_char_of_first_line("Hello, world!\nFine."));
}
