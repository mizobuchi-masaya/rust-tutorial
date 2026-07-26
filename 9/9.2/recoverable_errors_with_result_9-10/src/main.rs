use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;

    println!("{:?}", greeting_file);
}
