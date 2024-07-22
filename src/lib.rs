#![no_std]
#![allow(clippy::redundant_pattern_matching)]
use gstd::*;
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

// 游戏中有两种难度级别：
// DifficultyLevel::Easy 和 DifficultyLevel::Hard。
// 在简单模式下，程序应随机选择每轮可以移除的鹅卵石数量
// 而在困难模式下，程序应找到最佳的鹅卵石数量（找到获胜策略）。

// 使用以下辅助函数获取一个随机的32位数字：
pub fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

// 在简单模式下，程序随机选择每轮可以移除的鹅卵石数量。
// 在困难模式下，为了确保程序尽可能获得最终胜利
// 需要使程序每次尽可能获得指定的鹅卵石数量。
pub fn pebbles_auto_remove(game_state: &GameState) -> u32 {
    match game_state.difficulty {
        DifficultyLevel::Easy => (get_random_u32() % (game_state.max_pebbles_per_turn)) + 1,

        DifficultyLevel::Hard => {
            let peb_rem = game_state.pebbles_remaining;

            if peb_rem <= game_state.max_pebbles_per_turn {
                peb_rem
            } else {
                let ret_peb = peb_rem % (game_state.max_pebbles_per_turn + 1);
                // check `return pebbles` is valid or is not zero.
                if ret_peb == 0 {
                    1
                } else {
                    ret_peb
                }
            }
        }
    }
}

// 随机选择谁是第一个玩家。
pub fn choose_first_player() -> Player {
    match get_random_u32() % 2 {
        0 => Player::User,
        _ => Player::Program,
    }
}

// 验证输入的鹅卵石数量和每轮最大鹅卵石数量。
pub fn check_pebbles_input(
    init_msg_pebbles_count: u32,
    init_msg_max_pebbles_per_turn: u32,
) -> bool {
    if init_msg_pebbles_count < 1
        || init_msg_max_pebbles_per_turn < 1
        || init_msg_max_pebbles_per_turn >= init_msg_pebbles_count
    {
        return false;
    }
    true
}

// 开始或重启游戏。
pub fn restart_game(
    init_msg_difficulty: DifficultyLevel,
    init_msg_pebbles_count: u32,
    init_msg_max_pebbles_per_turn: u32,
) {
    // 检查输入数据的有效性
    if !check_pebbles_input(init_msg_pebbles_count, init_msg_max_pebbles_per_turn) {
        panic!("Invalid input message: Pebbles Count or Max Pebbles per Turn is invalid.");
    }
    // 使用`choose_first_player()`函数选择第一个玩家
    let first_player: Player = choose_first_player();

    // 填充`GameState`结构
    let mut pebbles_game = GameState {
        difficulty: init_msg_difficulty,
        pebbles_count: init_msg_pebbles_count,
        max_pebbles_per_turn: init_msg_max_pebbles_per_turn,
        pebbles_remaining: init_msg_pebbles_count,
        first_player: first_player.clone(),
        winner: None,
    };

    // 如果第一个玩家是程序，处理第一个回合
    if first_player == Player::Program {
        let program_take = pebbles_auto_remove(&pebbles_game);
        pebbles_game.pebbles_remaining =
            pebbles_game.pebbles_remaining.saturating_sub(program_take);
    }

    unsafe { PEBBLES_GAME = Some(pebbles_game) };
}

#[no_mangle]
pub extern "C" fn init() {
    // 使用 `msg::load` 函数接收 `PebblesInit`
    let load_init_msg = msg::load::<PebblesInit>().expect("Unable to load message");

    restart_game(
        load_init_msg.difficulty,
        load_init_msg.pebbles_count,
        load_init_msg.max_pebbles_per_turn,
    );
}

#[no_mangle]
pub extern "C" fn handle() {
    let load_action_msg = msg::load::<PebblesAction>().expect("Unable to load message");

    let get_pebbles_game = unsafe { PEBBLES_GAME.get_or_insert(Default::default()) };

    match load_action_msg {
        PebblesAction::GiveUp => {
            // 程序是赢家。
            get_pebbles_game.winner = Some(Player::Program);

            msg::reply(
                PebblesEvent::Won(
                    get_pebbles_game
                        .winner
                        .as_ref()
                        .expect("The Program Win")
                        .clone(),
                ),
                0,
            )
            .expect("Unable to reply GiveUp");
        }

        PebblesAction::Restart {
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
        } => {
            restart_game(difficulty.clone(), pebbles_count, max_pebbles_per_turn);

            msg::reply(
                PebblesInit {
                    difficulty,
                    pebbles_count,
                    max_pebbles_per_turn,
                },
                0,
            )
            .expect("Unable to reply Restart");
        }

        PebblesAction::Turn(mut x) => {
            // 用户回合执行
            if x > get_pebbles_game.max_pebbles_per_turn {
                x = get_pebbles_game.max_pebbles_per_turn;
            }

            get_pebbles_game.pebbles_remaining =
                get_pebbles_game.pebbles_remaining.saturating_sub(x);

            // 检查输入数据的有效性
            if !check_pebbles_input(
                get_pebbles_game.pebbles_count,
                get_pebbles_game.max_pebbles_per_turn,
            ) {
                panic!("Invalid PebblesAction User turn message: Pebbles Count or Max Pebbles per Turn is invalid.");
            }

            let peb_rem = get_pebbles_game.pebbles_remaining;

            if peb_rem == 0 {
                let won_exist = get_pebbles_game.winner.clone();

                if let Some(_) = &won_exist {
                    // 程序是赢家
                    msg::reply(
                        PebblesEvent::Won(won_exist.as_ref().expect("Game Over.").clone()),
                        0,
                    )
                    .expect("Unable to reply Turn for Winner");

                    exec::leave();
                } else {
                    // 用户是赢家
                    get_pebbles_game.winner = Some(Player::User);

                    msg::reply(
                        PebblesEvent::Won(
                            get_pebbles_game
                                .winner
                                .as_ref()
                                .expect("Game Over.")
                                .clone(),
                        ),
                        0,
                    )
                    .expect("Unable to reply Turn for Winner");

                    exec::leave();
                    // msg::send(ActorId::new(id), get_pebbles_game.clone(), 0).expect("Unable to send");
                }
            } else {
                msg::reply(PebblesEvent::CounterTurn(peb_rem), 0).expect("Unable to reply");
                // 程序执行下一轮
                let program_take = pebbles_auto_remove(get_pebbles_game);

                // 校验输入数据的有效性
                if !check_pebbles_input(
                    get_pebbles_game.pebbles_count,
                    get_pebbles_game.max_pebbles_per_turn,
                ) {
                    panic!("Invalid PebblesAction Program turn message: Pebbles Count or Max Pebbles per Turn is invalid.");
                }

                get_pebbles_game.pebbles_remaining = get_pebbles_game
                    .pebbles_remaining
                    .saturating_sub(program_take);

                let peb_rem = get_pebbles_game.pebbles_remaining;

                if peb_rem == 0 {
                    // 程序是赢家
                    get_pebbles_game.winner = Some(Player::Program);
                }
            }
        }
    };
}

#[no_mangle]
pub extern "C" fn state() {
    let pebbles_game = unsafe { PEBBLES_GAME.take().expect("Error in taking current state") };

    // 校验输入数据的有效性
    if !check_pebbles_input(
        pebbles_game.pebbles_count,
        pebbles_game.max_pebbles_per_turn,
    ) {
        panic!("Invalid PebblesAction User turn message: Pebbles Count or Max Pebbles per Turn is invalid.");
    }

    // 使用 `msg::reply` 函数返回 `GameState` 结构体
    msg::reply(pebbles_game, 0).expect("Failed to reply state");
}

#[cfg(test)]
mod tests {
    use crate::check_pebbles_input;
    use gstd::*;

    #[test]
    fn test_check_pebbles_input() {
        let res: bool = check_pebbles_input(0, 0);
        assert!(!res);
        let res: bool = check_pebbles_input(10, 3);
        assert!(res);
        let res: bool = check_pebbles_input(1, 2);
        assert!(!res);
    }
}
