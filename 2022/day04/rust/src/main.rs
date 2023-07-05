use std::fs;
use std::env;

fn pair_contains_or_overlaps_pair(pair1: Vec<&str>, pair2: Vec<&str>, part: i32) -> bool{
    let pair1_start = pair1[0].parse::<i32>().unwrap();
    let pair1_end = pair1[1].parse::<i32>().unwrap();
    let pair2_start = pair2[0].parse::<i32>().unwrap();
    let pair2_end = pair2[1].parse::<i32>().unwrap();
    let pair1_list = (pair1_start..=pair1_end).collect::<Vec<i32>>();
    let pair2_list = (pair2_start..=pair2_end).collect::<Vec<i32>>();

    if part == 1 {
        if pair1_list.contains(&pair2_start) && pair1_list.contains(&pair2_end) {
            return true;
        } else if pair2_list.contains(&pair1_start) && pair2_list.contains(&pair1_end) {
            return true;
        }
        return false;
    } else if part == 2 {
        if pair1_list.contains(&pair2_start) || pair1_list.contains(&pair2_end) {
            return true;
        } else if pair2_list.contains(&pair1_start) || pair2_list.contains(&pair1_end) {
            return true;
        } else {
            return false;
        }
    } else {
        panic!("Usage: ./day04 <1|2> <input file>");
    }

}


fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        panic!("Usage: ./day04 <1|2> <input file>");
    }

    let data = fs::read_to_string(&args[2]).expect("Unable to read file");

    let mut sum_pairs_full_contain:i32 = 0;

    if args[1] == "1" {
        data
            .lines()
            .for_each(|line| {
                let pairs: Vec<&str> = line.split(",").collect();
                let pairs_left: Vec<&str> = pairs[0].split("-").collect::<Vec<&str>>();
                let pairs_right: Vec<&str> = pairs[1].split("-").collect::<Vec<&str>>();
                if pair_contains_or_overlaps_pair(pairs_left, pairs_right, 1) {
                    sum_pairs_full_contain += 1;
                }
            });
        println!("In {} assignment pairs one range fully contains the other. ", sum_pairs_full_contain)
    } else if args[1] == "2" {
        data
            .lines()
            .for_each(|line| {
                let pairs: Vec<&str> = line.split(",").collect();
                let pairs_left: Vec<&str> = pairs[0].split("-").collect::<Vec<&str>>();
                let pairs_right: Vec<&str> = pairs[1].split("-").collect::<Vec<&str>>();
                if pair_contains_or_overlaps_pair(pairs_left, pairs_right, 2) {
                    sum_pairs_full_contain += 1;
                }
            });
        println!("In {} assignment pairs the ranges overlap. ", sum_pairs_full_contain)
    } else {
        panic!("Usage: ./day04 <1|2> <input file>");
    }
}
