/*
** EPITECH PROJECT, 2021
** G_O_L
** File description:
** Wow, such map, many place!
*/

use rand::Rng;

pub fn display_map(map: &Vec<Vec<bool>>) {
    for line in map {
        for cell in line {
            if *cell {
                print!("█");
            } else {
                print!("_");
            }
        }
        println!("");
    }
}

pub fn cpy_map(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_map = Vec::new();
    for line in map {
        let mut new_line = Vec::new();
        for cell in line {
            new_line.push(*cell);
        }
        new_map.push(new_line);
    }
    return new_map;
}

pub fn randomize_map(map: &mut Vec<Vec<bool>>) {
    for line in map {
        for cell in line {
            *cell = rand::thread_rng().gen::<bool>();
        }
    }
}

pub fn put_in_map(map: &mut Vec<Vec<bool>>, x: usize, y: usize, value: bool) {
    if x >= map[0].len() || y >= map.len() {
        return;
    }
    map[y][x] = value;
}

pub fn init_map(size_x:usize, size_y:usize) -> Vec<Vec<bool>> {
    let map = vec![vec![false; size_x]; size_y];
    return map;
}
