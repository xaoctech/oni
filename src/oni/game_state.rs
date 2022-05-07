use strum::EnumCount;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumCount)]
pub enum DifficultyLevel {
    Easy = 0,
    Normal = 1,
    Hard = 2,
}
