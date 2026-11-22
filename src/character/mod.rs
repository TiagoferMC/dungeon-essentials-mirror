pub mod enemy;
pub mod player;

use std::fmt::Display;

use crate::{
    character::{enemy::Enemy, player::Player},
    items::{Holdable, Weapon},
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
    fn get_weapon(&self) -> Weapon;

    fn attack(&self, target: &mut impl Damageable, command: Command) {
        let mut damage: usize = self.get_weapon().base_damage;
        let mul: f32 = match command {
            Command::Light => 1.0,
            Command::Medium => 1.5,
            Command::Heavy => 2.0,
            _ => 0.0,
        };

        damage = (damage as f32 * mul) as usize;
        target.take_damage(damage);
    }
}

impl Attacker for Player {
    fn get_weapon(&self) -> Weapon {
        if let Some(Holdable::Weapon(weapon)) = self.inventory.main_hand {
            weapon
        } else {
            Weapon {
                ..Weapon::default()
            }
        }
    }
}

impl Attacker for Enemy {
    fn get_weapon(&self) -> Weapon {
        Weapon {
            ..Weapon::default()
        }
    }
}
