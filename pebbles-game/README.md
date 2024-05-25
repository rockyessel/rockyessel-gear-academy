# Pebbles Game

## Overview

The Pebbles Game is a smart contract implementation of a simple game where players take turns removing pebbles from a pile. The game supports two players: a user and a program. The player who removes the last pebble wins the game. The game can be played with different difficulty levels, and it allows for restarting and giving up.

## Project Structure

The project is organized as follows:

- `src/`
  - `lib.rs`: Contains the main logic of the Pebbles Game smart contract.
  - `pebbles_game_io.rs`: Defines the input/output data structures and the metadata for the Pebbles Game.
  - `utils.rs`: Utility functions used in the game logic.
- `tests/`
  - `lib.rs`: Contains the integration tests for the Pebbles Game smart contract.
- `Cargo.toml`: Cargo configuration file for the project.

## Game Logic

### Initialization

The game is initialized with the following parameters:

- `difficulty`: The difficulty level of the game (Easy or Hard).
- `pebbles_count`: The total number of pebbles in the pile.
- `max_pebbles_per_turn`: The maximum number of pebbles a player can remove in a single turn.

The first player is chosen randomly.

### Actions

The game supports the following actions:

- `Turn(u32)`: The user takes a turn by removing the specified number of pebbles.
- `GiveUp`: The user gives up, and the program wins the game.
- `Restart { difficulty, pebbles_count, max_pebbles_per_turn }`: Restart the game with new parameters.

### Events

The game emits the following events:

- `CounterTurn(u32)`: The program takes a counter turn by removing the specified number of pebbles.
- `Won(Player)`: The specified player wins the game.

### Game State

The game state includes:

- `pebbles_count`: The initial number of pebbles.
- `max_pebbles_per_turn`: The maximum number of pebbles per turn.
- `pebbles_remaining`: The number of pebbles remaining in the pile.
- `difficulty`: The difficulty level of the game.
- `first_player`: The player who takes the first turn.
- `winner`: The player who won the game (if any).

## Blockchain Deployment

The Pebbles Game smart contract is deployed on the Vara Network with the following details:

- **Program ID**: `0x939a4ebed3bafe02bcf861562f711aedd73e105ff1338ab7300011dc88417da5`
- **Smart Contract Link**: [Pebbles Game on Vara Network](https://idea.gear-tech.io/programs/0x939a4ebed3bafe02bcf861562f711aedd73e105ff1338ab7300011dc88417da5?node=wss%3A%2F%2Ftestnet.vara.network)

## Building the Project

To build the project, run the following command:

```bash
cargo build --release
```

This will compile the project in release mode, optimizing the code for performance.

## Testing the Project

To run the tests for the project, use the following command:

```bash
cargo test
```

This will execute the test suite defined in the `tests/lib.rs` file. The tests cover various scenarios, including game initialization, user turns, program turns, restarting the game, and giving up.

## Running Tests

Here are the integration tests included in the project:

1. **Initialization Test**

   - Verifies that the game is initialized with the correct parameters and state.

2. **User Turn Winning Test**

   - Tests a scenario where the user wins the game by removing all remaining pebbles.

3. **Program Turn Winning Test**

   - Tests a scenario where the program wins the game.

4. **Restart Game Test**

   - Tests restarting the game with new parameters.

5. **Give Up Test**
   - Tests the user giving up, resulting in the program winning.

Each test ensures that the corresponding action produces the expected game state and events.

## Usage

To use the Pebbles Game smart contract, deploy the contract and send the appropriate messages for initialization and actions. The contract will reply with the corresponding events, indicating the outcome of each action.

## Example

```rust
let sys = System::new();
let program = init_game(&sys, DifficultyLevel::Easy, 10, 3);
let res = program.send(1, PebblesAction::Turn(3));
assert!(res.contains(&(
    1,
    PebblesEvent::Won(Player::User).encode()
)));
```

In this example, the game is initialized with an easy difficulty level, 10 pebbles, and a maximum of 3 pebbles per turn. The user takes a turn by removing 3 pebbles, resulting in a win for the user.

## Contributing

Contributions to the Pebbles Game project are welcome. Please open an issue or submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## GitHub Repository

The source code for this project is available on GitHub:

[GitHub Repository](https://github.com/rockyessel/rockyessel-gear-academy/tree/master/pebbles-game)
