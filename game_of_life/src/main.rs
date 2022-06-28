/*
** EPITECH PROJECT, 2021
** G_O_L
** File description:
** Wow, such game, much life!
*/

use std::env;
pub mod map_tools;
mod step;

fn error(arguments:&Vec<String>) -> bool {
    let args_len = arguments.len();

    if args_len > 5 || args_len < 3 {
        println!("Error: invalid number of arguments");
        return true;
    }
    let width = arguments[1].parse::<usize>().unwrap_or_else(|_| {
        println!("Error: required int for size_x");
        return 0;
    });
    let height = arguments[2].parse::<usize>().unwrap_or_else(|_| {
        println!("Error: required int for size_y");
        return 0;
    });
    if  width < 2 || height < 2 {
        println!("Invalid map size");
        return true;
    }
    if args_len >= 4 && arguments[3] != "--glider" && arguments[3] != "--rand" {
        println!("Error: invalid argument");
        return true;
    }
    if  args_len == 5 {
        let _speed = arguments[4].parse::<i32>().unwrap_or_else(|_| {
            println!("Error: required int for speed");
            return 0;
        });
    }
    return false;
}

fn add_glider(map:&mut Vec<Vec<bool>>, x:usize, y:usize) {
    map_tools::put_in_map(map, x, y + 1, true);
    map_tools::put_in_map(map, x + 1, y + 2, true);
    map_tools::put_in_map(map, x + 2, y, true);
    map_tools::put_in_map(map, x + 2, y + 1, true);
    map_tools::put_in_map(map, x + 2, y + 2, true);
}

fn setup_map(arguments:Vec<String>) -> Vec<Vec<bool>> {
    let width = arguments[1].parse::<i32>().unwrap() as usize;
    let height = arguments[2].parse::<i32>().unwrap() as usize;

    let mut map = map_tools::init_map(width, height);
    if arguments.len() >= 4 && arguments[3] == "--glider" {
        add_glider(&mut map, 1, 1);
    } else if arguments.len() >= 4 && arguments[3] == "--rand" {
        map_tools::randomize_map(& mut map);
    }
    if arguments.len() == 3 {
        map_tools::randomize_map(& mut map);
    }
    return map;
}

fn main() {
    let arguments:Vec<String> = env::args().collect();
    let ac = arguments.len();
    if ac == 2 &&
    (arguments[1] == "--help" || arguments[1] == "-h") {
        println!("Usage: ./game_of_life sizeX sizeY [--glider | --rand] (optional)-> speed");
        return;
    }
    if error(&arguments) {
        return;
    }
    let speed = if ac == 5 {arguments[4].parse::<u64>().unwrap()} else {1};
    let mut map = setup_map(arguments);
    map_tools::display_map(&map);
    for _i in 0..1000 {
        for _j in 0..map.len() {
            println!("");
        }
        map_tools::display_map(&map);
        std::thread::sleep(std::time::Duration::from_millis(1000 / speed));
        map = step::step(&map);
    }
}
