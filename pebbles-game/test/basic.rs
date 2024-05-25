#![cfg(test)]

use pebbles_game::*;
use pebbles_game_io::*;

#[test]
fn test_turn_valid_move() {
    let mut game_state = GameState {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 10,
        first_player: Player::User,
        winner: None,
    };

    let event = game_state.turn(2);
    assert_eq!(game_state.pebbles_remaining, 8);
    assert_eq!(event, PebblesEvent::CounterTurn(1)); // Program's turn
}

#[test]
fn test_turn_invalid_move() {
    let mut game_state = GameState {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 10,
        first_player: Player::User,
        winner: None,
    };

    let event = game_state.turn(5);
    assert_eq!(game_state.pebbles_remaining, 10); // No pebbles taken
    assert_eq!(event, PebblesEvent::CounterTurn(0)); // Invalid move, no pebbles taken
}

#[test]
fn test_turn_user_wins() {
    let mut game_state = GameState {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 2,
        max_pebbles_per_turn: 1,
        pebbles_remaining: 2,
        first_player: Player::User,
        winner: None,
    };

    let event = game_state.turn(2); // User takes the last pebble
    assert_eq!(game_state.pebbles_remaining, 0);
    assert_eq!(game_state.winner, Some(Player::User));
    assert_eq!(event, PebblesEvent::Won(Player::User));
}

#[test]
fn test_turn_program_wins() {
    let mut game_state = GameState {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 3,
        max_pebbles_per_turn: 1,
        pebbles_remaining: 3,
        first_player: Player::User,
        winner: None,
    };

    let event = game_state.turn(2); // User takes 2 pebbles, leaving 1 for the program
    assert_eq!(game_state.pebbles_remaining, 0);
    assert_eq!(game_state.winner, Some(Player::Program));
    assert_eq!(event, PebblesEvent::Won(Player::Program));
}

#[test]
fn test_give_up() {
    let mut game_state = GameState {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 10,
        first_player: Player::User,
        winner: None,
    };

    let event = game_state.give_up();
    assert_eq!(game_state.winner, Some(Player::Program));
    assert_eq!(event, PebblesEvent::Won(Player::Program));
}

#[test]
fn test_restart() {
    let mut game_state = GameState {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
        pebbles_remaining: 10,
        first_player: Player::User,
        winner: Some(Player::Program),
    };

    let event = game_state.restart(DifficultyLevel::Hard, 15, 4);
    assert_eq!(game_state.difficulty, DifficultyLevel::Hard);
    assert_eq!(game_state.pebbles_count, 15);
    assert_eq!(game_state.max_pebbles_per_turn, 4);
    assert_eq!(game_state.pebbles_remaining, 15);
    assert_eq!(game_state.first_player, Player::User);
    assert_eq!(game_state.winner, None);
    assert_eq!(event, PebblesEvent::CounterTurn(0)); // Game restarted
}
