use std::fs;

fn part_one(data: &str) -> i32 {
    let mut race_distances: Vec<u32> = Vec::new();
    let mut race_times: Vec<u32> = Vec::new();

    data
        .lines()
        .for_each(|line| {
            if let Some(distance) = line.strip_prefix("Distance:") {
                race_distances = distance
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().expect("Unable to parse"))
                    .collect();
            }

            if let Some(time) = line.strip_prefix("Time:") {
                race_times = time
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse().expect("Unable to parse"))
                    .collect();
            }
        });

    let mut num_of_ways_to_win_races: Vec<i32> = vec![0; race_distances.len()];
    
    for race_idx in 0..race_times.len() {
        for t_btn in 0..race_times[race_idx] {
            let distance_new = (race_times[race_idx] - t_btn) * t_btn;
            if race_distances[race_idx] < distance_new {
                num_of_ways_to_win_races[race_idx] += 1;
            }
        }
    }

    let mut result = 1;
    num_of_ways_to_win_races
        .iter()
        .for_each(|&num_of_ways| {
            if num_of_ways > 0 {
                result *= num_of_ways;
            } 
        });
    return result; 
}

fn part_two(data: &str) -> u64 {
    let mut race_distance: u64 = 0;
    let mut race_time: u64 = 0;

    data
        .lines()
        .for_each(|line| {
            if let Some(distance) = line.strip_prefix("Distance:") {
                race_distance = distance
                    .chars()
                    .flat_map(|s| s.to_digit(10))
                    .collect::<Vec<u32>>()
                    .iter() 
                    .fold(0, |acc, &x| acc * 10 + x as u64);
            }

            if let Some(time) = line.strip_prefix("Time:") {
                race_time =
                    time
                    .chars()
                    .flat_map(|s| s.to_digit(10))
                    .collect::<Vec<u32>>()
                    .iter() 
                    .fold(0, |acc, &x| acc * 10 + x as u64);
            }
        });
    
    let mut num_of_ways_to_win = 0;
    for t_btn in 0..race_time {
        let distance_new = (race_time - t_btn) * t_btn;
        if race_distance < distance_new {
            num_of_ways_to_win += 1;
        }
    }
    return num_of_ways_to_win;
}

fn main() {
    let data = fs::read_to_string("day06")
        .expect("Unable to read file");

    println!("Part 1: {} Part 2: {}", part_one(&data), part_two(&data));
}
