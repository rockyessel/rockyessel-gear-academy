#![no_std]

use gstd::Encode;
use gtest::System;
use pebbles_game_io::*;

fn init_game(
    sys: &System,
    difficulty: DifficultyLevel,
    pebbles_count: u32,
    max_pebbles_per_turn: u32,
) -> &gtest::Program {
    let init_msg = PebblesInit {
        difficulty,
        pebbles_count,
        max_pebbles_per_turn,
    };
    let program = sys.get_program(1);
    let res = program.send(1, init_msg);
    assert!(res.log().is_some());
    assert!(res.log().unwrap().contains("Successfully initialized"));
    program
}

#[test]
fn test_init_game() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Easy, 10, 3);
    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.pebbles_remaining, 10);
    assert_eq!(state.max_pebbles_per_turn, 3);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
}

#[test]
fn test_user_turn_winning() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Easy, 3, 3);
    let res = program.send(1, PebblesAction::Turn(3));
    assert!(res.contains(&(1, PebblesEvent::Won(Player::User).encode())));
    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.pebbles_remaining, 0);
    assert_eq!(state.winner, Some(Player::User));
}

#[test]
fn test_program_turn_winning() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Hard, 4, 3);
    let res = program.send(1, PebblesAction::Turn(1));
    assert!(res.contains(&(1, PebblesEvent::Won(Player::Program).encode())));
    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.pebbles_remaining, 0);
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_restart_game() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Easy, 10, 3);
    let res = program.send(
        1,
        PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 5,
            max_pebbles_per_turn: 2,
        },
    );
    assert!(res.contains(&(1, PebblesEvent::CounterTurn(0).encode())));
    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.pebbles_remaining, 5);
    assert_eq!(state.max_pebbles_per_turn, 2);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
}

#[test]
fn test_give_up() {
    let sys = System::new();
    let program = init_game(&sys, DifficultyLevel::Easy, 10, 3);
    let res = program.send(1, PebblesAction::GiveUp);
    assert!(res.contains(&(1, PebblesEvent::Won(Player::Program).encode())));
    let state: GameState = program.read_state().unwrap();
    assert_eq!(state.winner, Some(Player::Program));
}
