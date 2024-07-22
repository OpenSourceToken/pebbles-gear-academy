#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::prelude::*;

// the metadata to be used by the [IDEA](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara.network) portal.
pub struct PebblesMetadata;

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

// When initialising the game, it is necessary to pass some initial information.
// For example, the number of pebbles (N), maximum pebbles to be removed per turn (K), difficulty level.
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

// It needs to send actions message for every User's move and receive some event from the program.
// The action can be a turn with some count of pebbles to be removed or the give up.
// Also, there is a restart action than resets the game state .
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    Turn(u32),
    GiveUp,
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}

// And the event reflects the game state after the User's move:
// either pebbles count removed by the Program or the end of game with the information about the winner.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum PebblesEvent {
    CounterTurn(u32),
    Won(Player),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum Player {
    #[default]
    User,
    Program,
}

// Internal game state should keep all information related to the current state of the game.
// Some information is set during initialization, the first player is chosen randomly,
// some data are change during the game.
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}
