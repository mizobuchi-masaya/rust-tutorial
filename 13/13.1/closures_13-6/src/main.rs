use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From threa: {:?}", list))
        .join()
        .unwrap();
}
