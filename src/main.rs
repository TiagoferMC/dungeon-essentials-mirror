use enum_derived::Rand;
use rand::seq::IndexedRandom;
use std::io::BufRead;

use dungeon_essentials::{
    character::{enemy::*, player::*, *},
    items::*,
    *,
};

fn main() {
    let input_window_size: usize = 3;

    let mut enemy: Enemy = Enemy::random();

    let sword: Weapon = Weapon {
        base_damage: 200,
        stat_scaling: (Stat::Strike, 50),
        hard_damage: 2,
        climb: 4,
        max_level: 6,
        ..Default::default()
    };
    let sword: Holdable = Holdable::Weapon(sword);

    let inventory: Inventory = Inventory {
        main_hand: Some(sword),
        ..Default::default()
    };

    let mut player: Player = Player {
        name: "Adventurer".to_string(),
        level: 1,
        class: Class::Hero,
        health: 500,
        charge: 1000,
        essence: 1000,
        inventory,
        stats: Stats::default(),
    };

    println!(
        "Player: {}, Class: {}, Level: {}",
        player.name, player.class, player.level
    );
    println!("Enemy: {}", enemy.kind);

    loop {
        println!("Player: {} HP", player.health);
        println!("Enemy: {} HP", enemy.health);

        let mut line: String = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut line)
            .expect("What The Fuck");
        let input: &String = &line.trim().to_uppercase();

        if input.len() > input_window_size {
            println!(
                "Input too large. You only have {} slots",
                input_window_size
            );
            continue;
        }

        let commands: Option<Vec<Command>> = get_commands(input);
        if commands.is_none() {
            println!("Invalid Command Sequence");
            continue;
        }

        for command in commands.unwrap() {
            match command {
                Command::Light | Command::Medium | Command::Heavy => {
                    player.attack(&mut enemy, command)
                }
                _ => (),
            }

            if enemy.health == 0 {
                println!("You Win!");
                std::process::exit(0);
            }
        }

        // Enemy attack
        for _ in 0..input_window_size {
            let favored: bool = rand::random_bool(80.0 / 100.0);
            let command: Command = if favored {
                let mut rng = rand::rng();
                *enemy.favored_commands.choose(&mut rng).unwrap()
            } else {
                Command::rand()
            };

            println!("Enemy Command: {command}");
            enemy.attack(&mut player, command);
        }

        if player.health == 0 {
            println!("You died...");
            std::process::exit(0);
        }
    }
}
