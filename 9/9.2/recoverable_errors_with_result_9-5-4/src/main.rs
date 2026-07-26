use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
    //
    .expect("hello.txt should be inncluded in this project");

    println!("{:?}", greeting_file);
}
