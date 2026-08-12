fn main() {
    let hello = String::from("Hola");
    dbg!(hello);

    let hello = String::from("Здравствуйте");
    dbg!(hello);

    let hello = "Здравствуйте";
    // let answer = &hello[0];
    dbg!(hello);
    // dbg!(answer);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    dbg!(hello);
    dbg!(s);

    let hello = "Здравствуйте";
    // let s = &hello[0..1];
    dbg!(hello);
    // dbg!(s);

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
