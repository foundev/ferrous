mod game;
mod world;

use game::Game;
use std::io::{self, Write};

fn main() {
    print!("What is your name, traveler? ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim().to_string();

    if name.is_empty() {
        println!("You must have a name to enter this world.");
        return;
    }

    let mut game = Game::new(name);
    game.run();
}
