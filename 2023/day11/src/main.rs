use std::fs;

fn col_is_all_dots(matrix: &Vec<Vec<char>>, col_index: usize) -> bool {
    return matrix.iter().all(|row| row[col_index] == '.');
}

fn part_one(data: &str) -> i32 {   
    let mut universe: Vec<Vec<char>> = Vec::new();

    data
        .lines()
        .for_each(|line| {
            if line.chars().all(|c| c == '.') {
                universe.push(line.chars().collect::<Vec<char>>());
            }
            universe.push(line.chars().collect::<Vec<char>>());
        }); 
    
    let mut col_insertions = Vec::new();
    for col_idx in 0..universe[0].len() {
        if col_is_all_dots(&universe, col_idx) {
            col_insertions.push(col_idx);
        }
    }

    for idx in 0..col_insertions.len() {
        universe
            .iter_mut()
            .for_each(|row| {
                row.insert(col_insertions[idx] + idx, '.');
            });
    }

    let mut galaxies: Vec<(i32, i32)> = Vec::new();

    for i in 0..universe.len() {
        for j in 0..universe[0].len() {
            if universe[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    let mut total_distance = 0;
    
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total_distance += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }

    return total_distance;
}

fn main() {
    let data = fs::read_to_string("day11")
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", part_one(&data));
}
