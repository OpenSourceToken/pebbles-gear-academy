#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::prelude::*;

// 定义基础 PebblesMetadata 结构
pub struct PebblesMetadata;

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<GameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

// 定义游戏初始化参数
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
}

// 定义游戏难度等级
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

// 定义游戏执行动作
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

// 定义游戏返回事件
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum PebblesEvent {
    CounterTurn(u32),
    Won(Player),
}

// 定义玩家类型
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Player {
    #[default]
    User,
    Program,
}

// 定义游戏状态
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}
