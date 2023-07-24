use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        panic!("Usage: ./day05 <1|2> <input file>");
    }

    let contents = fs::read_to_string(&args[2])
        .expect("Something went wrong reading the file");
    
    fn get_first_marker(contents: &str, marker_length: usize) -> Option<usize> {
        for i in 0..(contents.len() - marker_length - 1) {
            let chars = &contents[i..(i + marker_length)];
            if chars.chars().collect::<HashSet<char>>().len() == marker_length {
                return Some(i + marker_length);
            }
        }
        None
    }

    if args[1] == "1" {
        println!("Part 1: {}", get_first_marker(&contents, 4).unwrap());
    } else if args[1] == "2" {
        println!("Part 2: {}", get_first_marker(&contents, 14).unwrap());
    } else {
        panic!("Usage: ./day06 <1|2> <input file>");
    }
}