use std::env;
use std::fs;

// Part 1
fn have_common_char(str1: &str, str2: &str) -> char {
    for c in str1.chars() {
        if str2.contains(c) {
            return c;
        }
    }
    return ' ';
}

// Part 2
fn have_common_char_group(str1: &str, str2: &str, str3: &str) -> char {
    for c in str1.chars() {
        if str2.contains(c) && str3.contains(c) {
            return c;
        }
    }
    return ' ';
}

// Part 1 and 2
fn convert_char_to_priority(c: char) -> u8 {
    let mut priority:u8 = c as u8 - 64;
    // reasigning the priority to match the requirements
    if priority < 27 {
        priority += 26;
    } else if priority > 32 {
        priority -= 32;
    }
    return priority;
}


fn main() {

    let args: Vec<String> = env::args().collect();

    // Reading file data into a string
    let data = fs::read_to_string(&args[2]).expect("Unable to read file");

    let mut sum_item_priorities:i16 = 0;

    // making sure the user has entered the correct arguments
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
                
                let priority:u8 = convert_char_to_priority(common_char);
                
                sum_item_priorities += priority as i16;
            });
        println!("Sum of item priorities: {}", sum_item_priorities);
    } else if args[1] == "2" {
        // Part 2
        // new data structure to store the data in groups of 3
        let mut part2_data = vec![];
        // read the data into the new data structure
        data
            .lines()
            .for_each(|line|
                part2_data.push(line)
            );
        // iterate over the data structure and find the common char in each group of 3
        for i in 0..part2_data.len() - 2 {
            if i % 3 == 0 || i == 0 {
                let common_char:char = have_common_char_group(&part2_data[i], &part2_data[i+1], &part2_data[i+2]);

                let priority:u8 = convert_char_to_priority(common_char);                

                sum_item_priorities += priority as i16;
            }
        }
        println!("Sum of item priorities: {}", sum_item_priorities);
    } else {
        panic!("Usage: ./day03 <1|2> <input file>");
    }
}