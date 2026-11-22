use std::fmt::Display;

use derive_more::Display;
use enum_derived::Rand;

use crate::{
    character::{
        enemy::EnemyType::{Bandit, DeathHound, Dragon, Golem, WildDog},
        Stats,
    },
    Command,
};

pub struct Enemy {
    pub kind: EnemyType,
    pub health: usize,
    pub stagger_meter: usize,
    pub poison_meter: usize,
    pub burn_meter: usize,
    pub stats: Stats,
    pub favored_commands: Vec<Command>,
}

#[derive(Display, Rand)]
pub enum EnemyType {
    Bandit,
    Dragon,
    Golem,
    DeathHound,
    WildDog,
}

impl Enemy {
    pub fn random() -> Self {
        match EnemyType::rand() {
            Bandit => Self::bandit(),
            Dragon => Self::dragon(),
            Golem => Self::golem(),
            DeathHound => Self::death_hound(),
            WildDog => Self::wild_dog(),
        }
    }

    pub fn bandit() -> Self {
        let stats = Stats {
            ..Default::default()
        };

        Self {
            kind: EnemyType::Bandit,
            health: rand::random_range(10..=20) * 10,
            stagger_meter: 100,
            poison_meter: 100,
            burn_meter: 100,
            stats,
            favored_commands: vec![Command::Light, Command::Medium],
        }
    }

    pub fn dragon() -> Self {
        let stats = Stats {
            element: 9,
            ..Default::default()
        };

        Self {
            kind: EnemyType::Dragon,
            health: rand::random_range(200..=500) * 10,
            stagger_meter: 500,
            poison_meter: 300,
            burn_meter: 700,
            stats,
            favored_commands: vec![Command::Medium, Command::Heavy],
        }
    }

    pub fn golem() -> Self {
        let stats = Stats {
            guts: 8,
            ..Default::default()
        };

        Self {
            kind: EnemyType::Golem,
            health: rand::random_range(50..=100) * 10,
            stagger_meter: 1000,
            poison_meter: 1000,
            burn_meter: 1000,
            stats,
            favored_commands: vec![Command::Heavy],
        }
    }

    pub fn death_hound() -> Self {
        let stats = Stats {
            violence: 8,
            ..Default::default()
        };

        Self {
            kind: EnemyType::DeathHound,
            health: rand::random_range(15..=30) * 10,
            stagger_meter: 50,
            poison_meter: 50,
            burn_meter: 50,
            stats,
            favored_commands: vec![Command::Light, Command::Medium],
        }
    }

    pub fn wild_dog() -> Self {
        let stats = Stats {
            ..Default::default()
        };

        Self {
            kind: EnemyType::WildDog,
            health: rand::random_range(5..=15) * 10,
            stagger_meter: 50,
            poison_meter: 50,
            burn_meter: 50,
            stats,
            favored_commands: vec![Command::Light, Command::Medium],
        }
    }
}

impl Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
