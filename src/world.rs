use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, usize>, // Direction -> Room ID
}

pub struct Player {
    pub name: String,
    pub current_room: usize,
}

pub struct World {
    pub rooms: Vec<Room>,
    pub player: Player,
}

impl World {
    pub fn new(player_name: String) -> Self {
        // Initialize a very simple world
        let mut rooms = Vec::new();

        // Room 0: The Starting Room
        let mut start_exits = HashMap::new();
        start_exits.insert("north".to_string(), 1);

        rooms.push(Room {
            name: "The Starting Room".to_string(),
            description: "You are in a cold, damp stone cell. A heavy iron door leads north."
                .to_string(),
            exits: start_exits,
        });

        // Room 1: The Hallway
        let mut hall_exits = HashMap::new();
        hall_exits.insert("south".to_string(), 0);

        rooms.push(Room {
            name: "The Hallway".to_string(),
            description: "A long, dimly lit hallway. You can hear water dripping somewhere. The cell is to the south.".to_string(),
            exits: hall_exits,
        });

        World {
            rooms,
            player: Player {
                name: player_name,
                current_room: 0,
            },
        }
    }

    pub fn get_current_room(&self) -> &Room {
        &self.rooms[self.player.current_room]
    }

    pub fn move_player(&mut self, direction: &str) -> Result<(), String> {
        let room = self.get_current_room();
        if let Some(&next_room_id) = room.exits.get(direction) {
            self.player.current_room = next_room_id;
            Ok(())
        } else {
            Err(format!("You cannot go {}.", direction))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        let world = World::new("TestPlayer".to_string());
        assert_eq!(world.player.name, "TestPlayer");
        assert_eq!(world.player.current_room, 0);
        assert_eq!(world.rooms.len(), 2);
    }

    #[test]
    fn test_move_player_success() {
        let mut world = World::new("TestPlayer".to_string());
        let result = world.move_player("north");
        assert!(result.is_ok());
        assert_eq!(world.player.current_room, 1);
    }

    #[test]
    fn test_move_player_failure() {
        let mut world = World::new("TestPlayer".to_string());
        let result = world.move_player("south"); // No south exit from starting room
        assert!(result.is_err());
        assert_eq!(world.player.current_room, 0);
        assert_eq!(result.unwrap_err(), "You cannot go south.");
    }

    #[test]
    fn test_move_back_and_forth() {
        let mut world = World::new("TestPlayer".to_string());
        world.move_player("north").unwrap();
        world.move_player("south").unwrap();
        assert_eq!(world.player.current_room, 0);
    }
}
