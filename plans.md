# Development Plan for Ferrous

This document outlines the roadmap for implementing the core features of Ferrous, following a top-down approach.

## Phase 1: Core Gameplay Mechanics (Single Player)
The goal of this phase is to build a fully functional text-adventure experience before introducing multi-user capabilities.

### 1. Item and Inventory System (Issue #1)
- [ ] Define an `Item` structure (ID, name, description).
- [ ] Update `Room` to hold a collection of items.
- [ ] Update `Player` to have an `Inventory`.
- [ ] Implement commands:
    - `get <item>`: Move item from room to player inventory.
    - `drop <item>`: Move item from player inventory to room.
    - `inventory` (or `i`): List items currently held.
- [ ] Add unit tests for item movement.

### 2. World Expansion & Narrative (Issue #2)
- [ ] Design a basic map with a central hub and diverging paths.
- [ ] Implement descriptive flavor text for each room to enhance immersion.
- [ ] Add "hidden" exits or requirements to move between certain rooms.

### 3. Data-Driven World (Issue #3)
- [ ] Select a serialization format (e.g., JSON or TOML).
- [ ] Create a world definition file (`world.json` or similar).
- [ ] Implement a loader in `src/world.rs` to populate the world state from the file.
- [ ] Refactor `World::new()` to take a file path.

### 4. Entities and Interactions (Issue #4)
- [ ] Define an `Entity` trait or struct for NPCs and Monsters.
- [ ] Implement basic entity placement in rooms.
- [ ] Implement interaction commands:
    - `talk <npc>`: Trigger a set of dialogue lines.
    - `attack <monster>`: Basic combat logic.
- [ ] Implement simple AI (e.g., monsters that move between rooms).

## Phase 2: Networking and Multi-User (Future)
Once the core logic is stable, we will transition to a networked environment.

- [ ] Introduce `tokio` for asynchronous networking.
- [ ] Implement a TCP server to handle multiple client connections.
- [ ] Refactor State Management: Use `Arc<RwLock<World>>` or a similar pattern to allow concurrent access to the game world.
- [ ] Implement a messaging system (send messages to specific players or rooms).
- [ ] Handle player authentication and persistence (saving/loading characters).

---

## Technical Reminders
- **Tests**: Every new feature must be accompanied by unit tests in the corresponding module.
- **Formatting**: Run `cargo fmt` before committing.
- **Guidelines**: Follow the rules set in `AGENTS.md`.
