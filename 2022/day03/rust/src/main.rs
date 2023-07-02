use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: ./day03 <1|2> <input file>");
    } else if args[1] == "1" {
        println!("Part 1");
    } else if args[1] == "2" {
        println!("Part 2");
    } else {
        panic!("Usage: ./day03 <1|2> <input file>");
    }
}