use crate::bungie::str::CStrArray;
use crate::oni::game_state::DifficultyLevel;
use crate::oni::weapon::PowerupType;
use bytemuck::{Pod, Zeroable};
use strum::EnumCount;

const HealthColorMaxPoints: usize = 16;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumCount)]
pub enum EventSound {
    DoorActionSuccess = 0,
    DoorActionFail = 1,
    DoorLock = 2,
    DoorUnlock = 3,
    UseHypo = 4,
    InventoryFail = 6,
    ReceiveAmmo = 7,
    ReceiveCell = 8,
    ReceiveHypo = 9,
    ReceiveLSI = 10,
    CompassNew = 11,
    ObjectiveNew = 12,
    ObjectivePrompt = 13,
    ObjectiveComplete = 14,
    Autosave = 15,
    TimerTick = 16,
    TimerSuccess = 17,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumCount)]
pub enum ConditionSound {
    HealthLow = 0,
    HealthOver = 1,
    Shield = 2,
    Invisibility = 3,
    TimerCritical = 4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct AutoPrompt {
    pub event_name: CStrArray<32>,
    pub start_level: u16,
    pub end_level: u16,
    pub message_name: CStrArray<32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct GameSettings {
    pub boosted_health: f32,
    pub hypo_amount: f32,
    pub hypo_boost_amount: f32,
    pub over_hypo_base_damage: f32,
    pub over_hypo_max_damage: f32,

    pub health_color_numpoints: u32,
    pub health_color_values: [f32; HealthColorMaxPoints],
    pub health_color_colors: [u32; HealthColorMaxPoints],

    // instance names for powerup geometries and glow textures
    pub powerup_geom: [CStrArray<128>; PowerupType::COUNT - 1],
    pub powerup_glow: [CStrArray<128>; PowerupType::COUNT - 1],
    pub powerup_glow_size: [CStrArray<128>; PowerupType::COUNT - 1],

    // sound names
    pub event_sound_name: [CStrArray<32>; EventSound::COUNT],
    pub condition_sound_name: [CStrArray<32>; ConditionSound::COUNT],

    // difficulty settings
    pub notice_multipliers: [f32; DifficultyLevel::COUNT],
    pub blocking_multipliers: [f32; DifficultyLevel::COUNT],
    pub dodge_multipliers: [f32; DifficultyLevel::COUNT],
    pub inaccuracy_multipliers: [f32; DifficultyLevel::COUNT],
    pub enemy_hp_multipliers: [f32; DifficultyLevel::COUNT],
    pub player_hp_multipliers: [f32; DifficultyLevel::COUNT],

    // auto prompt messages
    num_auto_prompts: u32,
    auto_prompts: [AutoPrompt; 16],
}

impl GameSettings {
    pub fn auto_prompts(&self) -> &[AutoPrompt] {
        &self.auto_prompts[..self.num_auto_prompts as usize]
    }
}
