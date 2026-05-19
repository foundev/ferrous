use crate::world::World;
use std::io::{self, Write};

pub struct Game {
    world: World,
}

impl Game {
    pub fn new(player_name: String) -> Self {
        Game {
            world: World::new(player_name),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to Ferrous!");
        println!("Type 'help' for a list of commands or 'quit' to exit.");

        loop {
            let room = self.world.get_current_room();
            println!("\n--- {} ---", room.name);
            println!("{}", room.description);
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "quit" {
                println!("Goodbye!");
                break;
            }

            if input == "help" {
                println!("Available commands: look, go <direction>, quit");
                continue;
            }

            if input == "look" {
                continue; // The loop will print the description anyway
            }

            if input.starts_with("go ") {
                let direction = input.strip_prefix("go ").unwrap();
                if let Err(e) = self.world.move_player(direction) {
                    println!("{}", e);
                }
                continue;
            }

            println!("I don't understand that command.");
        }
    }
}
