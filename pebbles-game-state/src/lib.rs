#[no_std]

use gmeta::metawasm;
use gstd::{exec, prelude::*};

use pebbles_game_io::*;

/// customsize game state function
#[metawasm]
pub mod metafns {
    pub type State = GameState;

    pub fn game_state(state: State) -> State {
        State {
            pebbles_count: state.pebbles_count,
            max_pebbles_per_turn: state.max_pebbles_per_turn,
            pebbles_remaining: state.pebbles_remaining,
            difficulty: state.difficulty,
            first_player: state.first_player,
            winner: state.winner,
        }
    }

    pub fn pebbles_count(state: State) -> u32 {
        state.pebbles_count
    }
    pub fn max_pebbles_per_turn(state: State) -> u32 {
        state.max_pebbles_per_turn
    }
    pub fn pebbles_remaining(state: State) -> u32 {
        state.pebbles_remaining
    }
    pub fn get_difficulty(state: State) -> DifficultyLevel {
        state.difficulty
    }
    pub fn first_player(state: State) -> Player {
        state.first_player
    }
    pub fn get_winner(state: State) -> Player {
        state.winner
    }    

}
