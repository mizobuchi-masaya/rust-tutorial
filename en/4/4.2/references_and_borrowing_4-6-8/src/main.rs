fn main() {
 /*
    let reference_to_nothing = dangle();

    println!("{:?}", reference_to_nothing);
*/

    let returns = no_dangle();
    println!("{:?}", returns);
}

/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
