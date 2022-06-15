/*
** EPITECH PROJECT, 2021
** game_of_life_rust
** File description:
** Wow, such game, much life!
*/
use std::env;

fn display_map(map: &Vec<Vec<bool>>) {
    for line in map {
        for cell in line {
            if *cell {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() == 2 &&
    (arguments[1] == "--help" || arguments[1] == "-h") {
        println!("Usage: ./game_of_life sizeX sizeY");
        return;
    }
    if arguments.len() != 3 {
        println!("Error: invalid number of arguments");
        return;
    }
    let width = arguments[1].parse::<i32>().unwrap();
    let height = arguments[2].parse::<i32>().unwrap();
    let width:usize = width as usize;
    let height:usize = height as usize;

    if  width < 2 || height < 2 {
        println!("Invalid map size");
        return;
    }
    let map = vec![vec![true; width]; height];
    display_map(&map);
}
