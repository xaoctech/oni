use strum::EnumCount;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumCount)]
pub enum PowerupType {
    AmmoBallistic,
    AmmoEnergy,
    Hypo,
    ShieldBelt,
    Invisibility,
    LSI,

    None = u32::MAX,
}
