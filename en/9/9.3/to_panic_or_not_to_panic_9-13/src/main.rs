pub mod guessing_game;

use guessing_game::Guess;

fn main() {
    let g = Guess::new(64);
    println!("{}", g.value());

    let g = Guess::new(164);
    println!("{}", g.value());
}
