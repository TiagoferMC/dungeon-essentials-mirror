// use raylib::prelude::*;

pub mod character;
pub mod items;
pub mod types;

use character::player::Player;
use derive_more::Display;
use enum_derived::Rand;
use items::Holdable;

#[derive(Rand, Display, Copy, Clone)]
pub enum Command {
    // Primary
    Rush,
    Use,
    Dodge,
    Light,
    Medium,
    Heavy,
    Fan,
    Parry,
    Block,

    // Secondary
    Safe,
    Wait,
    Sneak,
    Kick,
    Jab,
    Trick,
    Aim,
    Grab,
    Cast,
}

pub fn get_commands(input: &str) -> Option<Vec<Command>> {
    let mut commands: Vec<Command> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            'R' => commands.push(Command::Rush),
            'D' => commands.push(Command::Dodge),
            'L' => commands.push(Command::Light),
            'M' => commands.push(Command::Medium),
            'H' => commands.push(Command::Heavy),
            'F' => commands.push(Command::Fan),
            'P' => commands.push(Command::Parry),
            'B' => commands.push(Command::Block),

            'U' => {
                if let Some(next_ch) = chars.next() {
                    match next_ch {
                        'R' => commands.push(Command::Safe),
                        'U' => commands.push(Command::Wait),
                        'D' => commands.push(Command::Sneak),
                        'L' => commands.push(Command::Kick),
                        'M' => commands.push(Command::Jab),
                        'H' => commands.push(Command::Trick),
                        'F' => commands.push(Command::Aim),
                        'P' => commands.push(Command::Grab),
                        'B' => commands.push(Command::Cast),

                        _ => return None,
                    }
                }
            }

            _ => return None,
        }
    }

    Some(commands)
}

pub fn get_damage(player: &Player) -> usize {
    if let Holdable::Weapon(weapon) = player.inventory.main_hand.unwrap() {
        weapon.base_damage
    } else {
        0
    }
}

// pub fn render() {
// let (mut rl, thread) = raylib::init()
//     .resizable()
//     .title("Dungeon Essentials")
//     .build();
// rl.set_target_fps(60);

//     while !rl.window_should_close() {
//         let window_width = rl.get_screen_width().as_f32();
//         let window_height = rl.get_screen_height().as_f32();

//         let mut d = rl.begin_drawing(&thread);
//         d.clear_background(Color::BLACK);

//         let padding = 0.0;
//         let battle_area = Rectangle::new(
//             0.0 + padding,
//             0.0 + padding,
//             window_width - padding - padding,
//             window_height * 2.0 / 3.0 - padding - padding,
//         );
//         d.draw_rectangle_rec(battle_area, Color::GREEN);

//         let input_area = Rectangle::new(
//             battle_area.x,
//             battle_area.height,
//             battle_area.width,
//             window_height - battle_area.height,
//         );
//         d.draw_rectangle_rec(input_area, Color::RED);
//     }
// }
