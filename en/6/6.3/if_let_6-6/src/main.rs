fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("This maximum is configured to be {max}"),
        _ => (),
    }
}
