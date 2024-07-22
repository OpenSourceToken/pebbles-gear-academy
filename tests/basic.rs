use crate::scale_info::prelude::time::SystemTime;
use crate::scale_info::prelude::time::UNIX_EPOCH;
use gstd::*;
use gtest::{Log, Program, System};
use pebbles_game_io::*;

const ADMIN: u64 = 100;
const MAX_NUMBER_OF_TURNS: u32 = 8;
const MAX_PEBBLES_PER_TURN: u32 = 2;
const PEBBLES_COUNT: u32 = 15;
const DIFFICULTY_EASY: DifficultyLevel = DifficultyLevel::Easy;
const DIFFICULTY_HARD: DifficultyLevel = DifficultyLevel::Hard;

#[test]
fn success_restart_game() {
    let system = System::new();

    system.init_logger();
    let game = Program::current(&system);
    // 启动游戏
    let game_init_result = game.send(
        ADMIN,
        PebblesInit {
            difficulty: DIFFICULTY_EASY,
            pebbles_count: PEBBLES_COUNT,
            max_pebbles_per_turn: MAX_PEBBLES_PER_TURN,
        },
    );

    assert!(!game_init_result.main_failed());

    for i in 1..MAX_NUMBER_OF_TURNS {
        // 在第三轮，执行重启
        if i == 5 {
            let game_turn_5_result = game.send(
                ADMIN,
                PebblesAction::Restart {
                    difficulty: DIFFICULTY_EASY,
                    pebbles_count: PEBBLES_COUNT,
                    max_pebbles_per_turn: MAX_PEBBLES_PER_TURN,
                },
            );
            assert!(!game_turn_5_result.main_failed());
            break;
        }

        let random_removes = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("After restart removes")
            .subsec_nanos()
            % MAX_PEBBLES_PER_TURN)
            + 1;

        let game_user_result = game.send(ADMIN, PebblesAction::Turn(random_removes));

        assert!(!game_user_result.main_failed());
    }
    let random_removes = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Round removes")
        .subsec_nanos()
        % MAX_PEBBLES_PER_TURN)
        + 1;

    let game_user_result = game.send(ADMIN, PebblesAction::Turn(random_removes));
    assert!(!game_user_result.main_failed());

    let state: GameState = game.read_state(b"").unwrap();
    let pebbles_remaining: u32 = state.pebbles_remaining;
    let winner: Option<Player> = state.winner.clone();

    assert_ne!(pebbles_remaining, 0);
    assert_eq!(winner, None);
}

#[test]
fn success_giveup() {
    let system = System::new();

    system.init_logger();
    let game = Program::current(&system);

    let game_init_result = game.send(
        ADMIN,
        PebblesInit {
            difficulty: DIFFICULTY_EASY,
            pebbles_count: PEBBLES_COUNT,
            max_pebbles_per_turn: MAX_PEBBLES_PER_TURN,
        },
    );
    assert!(!game_init_result.main_failed());

    for i in 1..MAX_NUMBER_OF_TURNS {
        if i == 3 {
            let user_giveup_result = game.send(ADMIN, PebblesAction::GiveUp);

            let program_win_log = Log::builder()
                .dest(ADMIN)
                .payload(PebblesEvent::Won(Player::Program));
            assert!(user_giveup_result.contains(&program_win_log));

            break;
        }

        let random_removes = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("After user giveup removes")
            .subsec_nanos()
            % MAX_PEBBLES_PER_TURN)
            + 1;

        let user_choice = random_removes;
        let game_user_result = game.send(ADMIN, PebblesAction::Turn(user_choice));

        assert!(!game_user_result.main_failed())
    }
    let state: GameState = game.read_state(b"").unwrap();
    let pebbles_remaining: u32 = state.pebbles_remaining;
    let winner: Player = state.winner.as_ref().expect("The Program win").clone();

    assert_ne!(pebbles_remaining, 0);
    assert_eq!(winner, Player::Program);
}

#[test]
fn success_run_game_with_difficulty_easy() {
    let system = System::new();

    system.init_logger();
    let game = Program::current(&system);

    let game_init_result = game.send(
        ADMIN,
        PebblesInit {
            difficulty: DIFFICULTY_EASY,
            pebbles_count: PEBBLES_COUNT,
            max_pebbles_per_turn: MAX_PEBBLES_PER_TURN,
        },
    );
    assert!(!game_init_result.main_failed());

    for _ in 1..MAX_NUMBER_OF_TURNS {
        let random_removes = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("REASON")
            .subsec_nanos()
            % MAX_PEBBLES_PER_TURN)
            + 1;

        let game_user_result = game.send(ADMIN, PebblesAction::Turn(random_removes));

        assert!(!game_user_result.main_failed());

        let state: GameState = game.read_state(b"").unwrap();
        let pebbles_remaining: u32 = state.pebbles_remaining;

        if pebbles_remaining == 0 {
            break;
        }
    }

    let state: GameState = game.read_state(b"").unwrap();
    let pebbles_remaining: u32 = state.pebbles_remaining;
    let winner: Player = state.winner.as_ref().expect("May be Win").clone();

    assert_eq!(pebbles_remaining, 0);
    assert!(winner == Player::Program || winner == Player::User);
}

#[test]
fn success_run_game_with_difficulty_hard() {
    let system = System::new();

    system.init_logger();
    let game = Program::current(&system);

    let game_init_result = game.send(
        ADMIN,
        PebblesInit {
            difficulty: DIFFICULTY_HARD,
            pebbles_count: PEBBLES_COUNT,
            max_pebbles_per_turn: MAX_PEBBLES_PER_TURN,
        },
    );
    assert!(!game_init_result.main_failed());

    for _ in 1..MAX_NUMBER_OF_TURNS {
        let random_removes = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("REASON")
            .subsec_nanos()
            % MAX_PEBBLES_PER_TURN)
            + 1;

        let game_user_result = game.send(ADMIN, PebblesAction::Turn(random_removes));

        assert!(!game_user_result.main_failed());

        let state: GameState = game.read_state(b"").unwrap();
        let pebbles_remaining: u32 = state.pebbles_remaining;

        if pebbles_remaining == 0 {
            break;
        }
    }

    let state: GameState = game.read_state(b"").unwrap();
    let pebbles_remaining: u32 = state.pebbles_remaining;
    let winner: Player = state.winner.as_ref().expect("May be Win").clone();

    assert_eq!(pebbles_remaining, 0);
    assert!(winner == Player::Program || winner == Player::User);
}
