# Agents Guidelines for Ferrous

Ferrous is a GPL-licensed text-based MUD written in Rust.

## General Rules
- **Language**: All code, documentation, and commit messages must be in **English**. Communication with the user should be in **French** (as per user instructions).
- **Coding Style**: Follow standard Rust idioms and `cargo fmt` formatting.
- **Approach**: We are following a top-down approach, starting with core game logic before introducing networking.

## Technical Guidelines
- **State Management**: Prefer clear ownership and borrowing. Avoid unnecessary `Arc<Mutex<T>>` until multi-threading/networking is introduced.
- **World Design**: Keep the world data decoupled from the game loop logic to facilitate future migration to external data files.
- **Error Handling**: Use `Result` for recoverable errors instead of `panic!`.

## Project Structure
- `src/main.rs`: Entry point.
- `src/game.rs`: Game loop and command processing.
- `src/world.rs`: World model, room and player logic.
- `LICENSE`: GPL v3 license.

## Workflow
1. Break complex tasks into smaller steps.
2. Implement tests for new logic.
3. Verify functionality with `cargo test` and `cargo run`.
