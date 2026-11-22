use std::fmt::Display;

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

pub enum DurabilityStatus {
    Prime,
    WornOut1,
    WornOut2,
    WornOut3,
    Broken,
}

impl Display for DurabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prime => write!(f, "Prime"),
            Self::WornOut1 => write!(f, "Worn Out (1)"),
            Self::WornOut2 => write!(f, "Worn Out (2)"),
            Self::WornOut3 => write!(f, "Worn Out (3)"),
            Self::Broken => write!(f, "Broken"),
        }
    }
}

pub trait ItemDamageable {
    fn get_durability(&self) -> usize;
    fn get_durability_mut(&mut self) -> &mut usize;
    fn get_max_durability(&self) -> usize;
    fn get_durability_percent(&self) -> usize;

    fn take_damage(&mut self, damage: usize) {
        let durability: &mut usize = self.get_durability_mut();
        *durability = durability.saturating_sub(damage);
        println!("Weapon Took {damage} damage");
        println!("New durability {}", durability);
    }

    fn get_durability_status(&self) -> DurabilityStatus {
        match self.get_durability_percent() {
            0 => DurabilityStatus::Broken,
            x if x <= 5 => DurabilityStatus::WornOut3,
            x if x <= 20 => DurabilityStatus::WornOut2,
            x if x <= 50 => DurabilityStatus::WornOut1,
            x if x <= 100 => DurabilityStatus::Prime,
            _ => panic!("Invalid percentage"),
        }
    }
}

macro_rules! impl_item_damageable {
    ($($t:ty),*) => {
        $(
            impl ItemDamageable for $t {
                fn get_durability(&self) -> usize {
                    self.durability
                }

                fn get_durability_mut(&mut self) -> &mut usize {
                    &mut self.durability
                }

                fn get_max_durability(&self) -> usize {
                    self.max_durability
                }

                fn get_durability_percent(&self) -> usize {
                    (self.durability as f32 / self.max_durability as f32 * 100.0) as usize
                }
            }
        )*
    };
}

impl_item_damageable!(Weapon, Armor);

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

macro_rules! make_holdable {
    ($($f:ident => $t:ty),*) => {
        impl Holdable {
            $(
                pub fn $f(&mut self) -> Option<$t> {
                    match self {
                        Holdable::Weapon(weapon) => Some(weapon.$f()),
                        Holdable::Shield(shield) => Some(shield.$f()),
                        Holdable::Consumable(_) => None,
                    }
                }
            )*
        }
    };
}

make_holdable! {
    get_durability => usize,
    get_durability_mut => &mut usize,
    get_max_durability => usize,
    get_durability_percent => usize,
    get_durability_status => DurabilityStatus
}
