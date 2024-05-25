#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, Encode};

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PebblesMetadata;

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum PebblesAction {
    Turn(u32),
    GiveUp,
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum PebblesEvent {
    CounterTurn(u32),
    Won(Player),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Player {
    #[default]
    User,
    Program,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}

impl GameState {
    pub fn turn(&mut self, turn_count: u32) -> PebblesEvent {
        // User's turn
        if turn_count < 1 || turn_count > self.max_pebbles_per_turn {
            return PebblesEvent::CounterTurn(0); // Invalid move, no pebbles taken
        }

        self.pebbles_remaining -= turn_count;
        if self.pebbles_remaining == 0 {
            self.winner = Some(Player::User);
            return PebblesEvent::Won(Player::User);
        }

        // Program's turn
        let program_pebbles_taken = match self.difficulty {
            DifficultyLevel::Easy => 1, // Simple strategy for easy mode
            DifficultyLevel::Hard => {
                if self.pebbles_remaining <= self.max_pebbles_per_turn {
                    self.pebbles_remaining
                } else {
                    let optimal_move =
                        (self.pebbles_remaining - 1) % (self.max_pebbles_per_turn + 1);
                    if optimal_move == 0 {
                        1
                    } else {
                        optimal_move
                    }
                }
            }
        };

        self.pebbles_remaining -= program_pebbles_taken;
        if self.pebbles_remaining == 0 {
            self.winner = Some(Player::Program);
            PebblesEvent::Won(Player::Program)
        } else {
            PebblesEvent::CounterTurn(program_pebbles_taken)
        }
    }
    pub fn give_up(&mut self) -> PebblesEvent {
        self.winner = Some(Player::Program);
        PebblesEvent::Won(Player::Program)
    }
    pub fn restart(
        &mut self,
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    ) -> PebblesEvent {
        self.difficulty = difficulty;
        self.pebbles_count = pebbles_count;
        self.max_pebbles_per_turn = max_pebbles_per_turn;
        self.pebbles_remaining = pebbles_count;
        self.first_player = Player::User;
        self.winner = None;
        PebblesEvent::CounterTurn(0)
    }
}

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
