/*
** EPITECH PROJECT, 2021
** G_O_L
** File description:
** Wow, such step, much life!
*/

use crate::map_tools;

fn check_neighbours(map: &Vec<Vec<bool>>, x: usize, y: usize) -> i32 {
    let mut neighbours = 0;
    let x_max:usize = map[0].len() - 1;
    let y_max:usize = map.len() - 1;

    if x > 0 && map[y][x - 1] {
        neighbours += 1;
    }
    if x < x_max - 1 && map[y][x + 1] {
        neighbours += 1;
    }
    if y > 0 && map[y - 1][x] {
        neighbours += 1;
    }
    if y < y_max - 1 && map[y + 1][x] {
        neighbours += 1;
    }
    if x > 0 && y > 0 && map[y - 1][x - 1] {
        neighbours += 1;
    }
    if x < x_max - 1 && y > 0 && map[y - 1][x + 1] {
        neighbours += 1;
    }
    if x > 0 && y < y_max - 1 && map[y + 1][x - 1] {
        neighbours += 1;
    }
    if x < x_max - 1 && y < y_max - 1 && map[y + 1][x + 1] {
        neighbours += 1;
    }
    return neighbours;
}

pub fn step(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_map = map_tools::cpy_map(map);
    let mut neighbours: i32;
    let size_x = map[0].len();
    let size_y = map.len();

    for y in 0..size_y {
        for x in 0..size_x {
            neighbours = check_neighbours(map, x, y);
            if neighbours < 2 || neighbours > 3 {
                map_tools::put_in_map(&mut new_map, x, y, false);
            } else if neighbours == 3 {
                map_tools::put_in_map(&mut new_map, x, y, true);
            }
        }
    }
    return new_map;
}