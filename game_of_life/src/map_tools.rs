/*
** EPITECH PROJECT, 2021
** G_O_L
** File description:
** Wow, such map, many place!
*/

pub fn display_map(map: &Vec<Vec<bool>>) {
    for line in map {
        for cell in line {
            if *cell {
                print!("#");
            } else {
                print!("o");
            }
        }
        println!("");
    }
}

pub fn init_map(size_x:usize, size_y:usize) -> Vec<Vec<bool>> {
    let map = vec![vec![false; size_x]; size_y];
    return map;
}
