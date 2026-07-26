use std::fs::File;

fn main() {
    let greeting_file_path = File::open("hello.txt");

    println!("{:?}", greeting_file_path);
}
