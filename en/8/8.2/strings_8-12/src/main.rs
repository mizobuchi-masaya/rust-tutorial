fn main() {
    let data = "initial contents";

    let s = data.to_string();
    dbg!(s);

    // The method also works on a literal directly:
    let s = "initial cobtents".to_string();
    dbg!(s);
}
