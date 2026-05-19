# Ferrous

Ferrous is a text-based Multi-User Dungeon (MUD) written in Rust. It is designed with a focus on scalability and a top-down development approach, starting with core game logic before expanding into networking and advanced features.

## Features

- **Text-based Engine**: A classic MUD experience driven by text commands.
- **Rust Powered**: Built using Rust for safety, performance, and concurrency.
- **Modular Design**: Decoupled world data and game logic to allow for easy expansion and potential external data loading.
- **GPL v3 Licensed**: Open-source and free to use and modify.

## Getting Started

### Prerequisites

To build and run Ferrous, you need to have the Rust toolchain installed. If you don't have it, you can get it from [rustup.rs](https://rustup.rs/).

### Installation

Clone the repository:

```bash
git clone https://github.com/foundev/ferrous.git
cd ferrous
```

### Running the Game

You can run the game directly using `cargo`:

```bash
cargo run
```

## Development

### Coding Standards

We follow standard Rust idioms. Before committing any changes, please ensure you run the following checks:

1. **Formatting**:
   ```bash
   cargo fmt
   ```

2. **Linting**:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

3. **Testing**:
   ```bash
   cargo test --all-features
   ```

### Project Structure

- `src/main.rs`: Entry point of the application.
- `src/game.rs`: Core game loop and command processing logic.
- `src/world.rs`: World model, including room and player logic.

## License

Ferrous is licensed under the GNU General Public License v3. See the [LICENSE](LICENSE) file for more details.
