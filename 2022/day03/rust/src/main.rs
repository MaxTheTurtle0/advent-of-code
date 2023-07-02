use std::env;
use std::fs;

fn have_common_char(str1: &str, str2: &str) -> char {
    for c in str1.chars() {
        if str2.contains(c) {
            return c;
        }
    }
    return ' ';
}


fn main() {

    let args: Vec<String> = env::args().collect();

    // Reading file data into a string
    let data = fs::read_to_string(&args[2]).expect("Unable to read file");

    let mut sum_item_priorities = 0;

    if args.len() != 3 {
        panic!("Usage: ./day03 <1|2> <input file>");
    } else if args[1] == "1" {
        // Part 1
        let mut left_compartment:&str = "";
        let mut right_compartment:&str = "";
        data
            .lines()
            .for_each(|line| {
                let half_len = line.len() / 2;
                left_compartment = &line[..half_len];
                right_compartment = &line[half_len..];
                let common_char:char = have_common_char(&left_compartment, &right_compartment);
                let mut priority:u8 = common_char as u8 - 64;

                if priority < 27 {
                    priority += 26;
                } else if priority > 32 {
                    priority -= 32;
                }
                sum_item_priorities += priority as i32;
            });
        println!("Sum of item priorities: {}", sum_item_priorities);
    } else if args[1] == "2" {
        // Part 2
        println!("Part 2");
    } else {
        panic!("Usage: ./day03 <1|2> <input file>");
    }
}