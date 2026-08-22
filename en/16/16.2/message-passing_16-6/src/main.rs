use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    println!("tx: {tx:?}");
    println!("rx: {rx:?}");
}
