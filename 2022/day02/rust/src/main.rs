use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Invalid number of arguments");
        println!("Usage: ./day02 <1|2> file");
        return;
    } else if args[1] != "1" && args[1] != "2" {
        println!("Invalid argument");
        println!("Usage: ./day02 <1|2> file");
        return;
    }

    let win:i16 = 6;
    let draw:i16 = 3;

    let mut score:i16 = 0;

    if args[1] == "1" {
        let data = fs::read_to_string(&args[2]).expect("Unable to read file");
        

        let outcomes = HashMap::from(
            [
                ("A X", draw + 1),
                ("A Y", win + 2),
                ("A Z", 3),
                ("B X", 1),
                ("B Y", draw + 2),
                ("B Z", win + 3),
                ("C X", win + 1),
                ("C Y", 2),
                ("C Z", draw + 3)
            ]
        );
        
        data
            .lines()
            .for_each(|line| 
                score += outcomes.get(&line).unwrap());

        println!("Score: {}", score);
    } else {

        fn get_winning_score(enemy_score: char) -> i16  {
            if enemy_score == 'A' {
                return 2;
            }
            else if enemy_score == 'B' {
                return 3;
            }
            else {
                return 1;
            }
        }

        fn get_losing_score(enemy_score: char) -> i16  {
            if enemy_score == 'A' {
                return 3;
            }
            else if enemy_score == 'B' {
                return 1;
            }
            else {
                return 2;
            }
        }

        fn get_draw_score(enemy_score: char) -> i16  {
            if enemy_score == 'A' {
                return 1;
            }
            else if enemy_score == 'B' {
                return 2;
            }
            else {
                return 3;
            }
        }

        let data = fs::read_to_string(&args[2]).expect("Unable to read file");

        data
            .lines()
            .for_each(|line| 
                if line.chars().nth(2).unwrap() == 'X' {
                    score += get_losing_score(line.chars().nth(0).unwrap());
                } else if line.chars().nth(2).unwrap() == 'Y' {
                    score += draw + get_draw_score(line.chars().nth(0).unwrap());
                } else {
                    score += get_winning_score(line.chars().nth(0).unwrap()) + win;
                });

        println!("Score: {}", score);
    }

}