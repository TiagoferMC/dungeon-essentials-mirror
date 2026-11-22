use crate::character::*;
use crate::types::*;

#[derive(Copy, Clone)]
pub enum Item {
    Armor(Armor),
    Holdable(Holdable),
}

#[derive(Copy, Clone)]
pub struct Armor {
    pub physical_defense: (PhysicalType, usize),
    pub elemental_defense: (ElementalType, usize),
    pub passive: Option<Effect>,
    pub durability: usize,
    pub max_durability: usize,
}

#[derive(Copy, Clone)]
pub enum Holdable {
    Weapon(Weapon),
    Shield(Shield),
    Consumable(Consumable),
}

#[derive(Copy, Clone)]
pub struct Weapon {
    pub level: usize,
    pub base_damage: usize,
    pub stat_scaling: (Stat, usize),
    pub hard_damage: usize,
    pub climb: usize,
    pub max_level: usize,
    pub durability: usize,
    pub max_durability: usize,
}

#[derive(Copy, Clone)]
pub struct Consumable {
    pub action_cost: usize,
    pub price: usize,
    pub effect: Effect,
}

pub type Shield = Armor;

impl Default for Weapon {
    fn default() -> Self {
        Self {
            level: 0,
            base_damage: 50,
            stat_scaling: (Stat::Strike, 10),
            hard_damage: 0,
            climb: 0,
            max_level: 0,
            durability: 1000,
            max_durability: 1000,
        }
    }
}

pub trait Equippable {
    fn get_damage_percent(&self) -> f32;

    fn get_damage_status(&self) -> String {
        match self.get_damage_percent() {
            0.0 => "Broken".to_string(),
            x if x < 5.0 => "Worn Out (3)".to_string(),
            5.0..20.0 => "Worn Out (2)".to_string(),
            20.0..50.0 => "Worn Out (1)".to_string(),
            50.0..=100.0 => "Prime".to_string(),
            _ => panic!("Invalid percentage!"),
        }
    }
}

impl Equippable for Armor {
    fn get_damage_percent(&self) -> f32 {
        self.durability as f32 / self.max_durability as f32
    }
}

impl Equippable for Weapon {
    fn get_damage_percent(&self) -> f32 {
        self.durability as f32 / self.max_durability as f32
    }
}
