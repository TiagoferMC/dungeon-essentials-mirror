use std::fmt::Display;

use crate::{character::Stats, items::*};
use derive_more::Display;

pub struct Player {
    pub name: String,
    pub level: usize,
    pub class: Class,
    pub health: usize,
    pub inventory: Inventory,
    pub charge: usize,
    pub essence: usize,
    pub stats: Stats,
}

#[derive(Display)]
pub enum Class {
    Hero,
    Fighter,
    Rogue,
    Wizard,
}

#[derive(Copy, Clone, Default)]
pub struct Inventory {
    // Armor
    pub helmet: Option<Armor>,
    pub chest: Option<Armor>,
    pub legs: Option<Armor>,
    pub boots: Option<Armor>,
    // Hands
    pub main_hand: Option<Holdable>,
    pub off_hand: Option<Holdable>,
    // Rest
    pub inventory: [Option<Item>; 9],
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
