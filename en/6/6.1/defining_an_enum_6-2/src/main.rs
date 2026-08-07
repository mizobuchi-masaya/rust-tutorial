#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    dbg!(Message::Quit);
    dbg!(Message::Move{x:0, y:0});
    dbg!(Message::Write(String::from("")));
    dbg!(Message::ChangeColor(0, 0, 0));
}
