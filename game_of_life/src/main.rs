/*
** EPITECH PROJECT, 2021
** G_O_L
** File description:
** Wow, such game, much life!
*/

use std::env;
mod map_tools;

fn error(arguments:&Vec<String>) -> bool {
    if arguments.len() != 3 {
        println!("Error: invalid number of arguments");
        return true;
    }
    return false;
}

fn main() {
    let arguments:Vec<String> = env::args().collect();
    if arguments.len() == 2 &&
    (arguments[1] == "--help" || arguments[1] == "-h") {
        println!("Usage: ./game_of_life sizeX sizeY");
        return;
    }
    if error(&arguments) {
        return;
    }
    let width = arguments[1].parse::<i32>().unwrap() as usize;
    let height = arguments[2].parse::<i32>().unwrap() as usize;
    if  width < 2 || height < 2 {
        println!("Invalid map size");
        return;
    }
    let map = map_tools::init_map(width, height);
    map_tools::display_map(&map);
}
