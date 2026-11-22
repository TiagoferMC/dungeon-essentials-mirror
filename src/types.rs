use crate::character::Stat;

pub enum DamageTypes {
    Physical(PhysicalType),
    Elemental(ElementalType),
}

#[derive(Copy, Clone)]
pub enum PhysicalType {
    Slashing,
    Piercing,
    Crushing,
}

#[derive(Copy, Clone)]
pub enum ElementalType {
    Magical,
    Flaming,
    Holy,
    Toxic,
    BloodLetting,
    Freezing,
}

#[derive(Copy, Clone)]
pub enum Effect {
    Heal(usize),
    Boost(Stat, usize),
}
