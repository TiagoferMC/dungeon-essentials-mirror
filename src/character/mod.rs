pub mod enemy;
pub mod player;

use std::fmt::Display;

use crate::{
    character::{enemy::Enemy, player::Player},
    items::{DurabilityStatus, Holdable, ItemDamageable, Weapon},
    Command,
};

macro_rules! define_stats {
    ($($variant:ident => $field:ident),+ $(,)?) => {
        #[derive(Copy, Clone)]
        pub enum Stat {
            $($variant),+
        }

        #[derive(Copy, Clone)]
        pub struct Stats {
            $(pub $field: usize),+
        }

        impl Default for Stats {
            fn default() -> Self {
                Self {
                    $($field: 5),+
                }
            }
        }

        impl Stats {
            pub fn get(&self, stat: Stat) -> usize {
                match stat {
                    $(Stat::$variant => self.$field),+
                }
            }

            pub fn boost(&mut self, stat: Stat, amount: usize) {
                match stat {
                    $(Stat::$variant => self.$field += amount),+
                }
            }
        }
    };
}

define_stats! {
    Strike => strike,
    Nimbleness => nimbleness,
    Vitality => vitality,
    Guts => guts,
    Fortune => fortune,
    Magus => magus,
    Violence => violence,
    Stance => stance,
    Element => element,
    Reflexes => reflexes,
}

pub trait Damageable: Display {
    fn health_mut(&mut self) -> &mut usize;

    fn take_damage(&mut self, damage: usize) {
        let health: &mut usize = self.health_mut();
        *health = health.saturating_sub(damage);

        println!("{} took {} damage.", self, damage);
    }
}

macro_rules! impl_damageable {
    ($($t:ty),*) => {
        $(
            impl Damageable for $t {
                fn health_mut(&mut self) -> &mut usize {
                    &mut self.health
                }
            }
        )*
    };
}

impl_damageable!(Player, Enemy);

pub trait Attacker {
    fn get_weapon(&self) -> Option<&Weapon>;

    fn get_weapon_mut(&mut self) -> Option<&mut Weapon>;

    fn attack(&self, target: &mut impl Damageable, command: Command) {
        let default_weapon: Weapon = Weapon {
            ..Weapon::default()
        };

        let weapon: &Weapon = self.get_weapon().unwrap_or(&default_weapon);
        let mut damage: f32 = weapon.base_damage as f32;

        damage *= match command {
            Command::Light => 1.0,
            Command::Medium => 1.5,
            Command::Heavy => 2.0,
            _ => 0.0,
        };

        let durability_status: DurabilityStatus =
            weapon.get_durability_status();

        damage *= match durability_status {
            DurabilityStatus::Prime => 1.2,
            DurabilityStatus::WornOut1 => 1.0,
            DurabilityStatus::WornOut2 => 0.9,
            DurabilityStatus::WornOut3 => 0.5,
            DurabilityStatus::Broken => 0.0,
        };

        target.take_damage(damage as usize);
    }
}

impl Attacker for Player {
    fn get_weapon(&self) -> Option<&Weapon> {
        match &self.inventory.main_hand {
            Some(Holdable::Weapon(weapon)) => Some(weapon),
            _ => None,
        }
    }

    fn get_weapon_mut(&mut self) -> Option<&mut Weapon> {
        match &mut self.inventory.main_hand {
            Some(Holdable::Weapon(weapon)) => Some(weapon),
            _ => None,
        }
    }
}

impl Attacker for Enemy {
    fn get_weapon(&self) -> Option<&Weapon> {
        None
    }

    fn get_weapon_mut(&mut self) -> Option<&mut Weapon> {
        None
    }
}
