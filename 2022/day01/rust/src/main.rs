use std::fs;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        panic!("Please provide one file path");
    }

    let file_data = fs::read_to_string(&args[1]).expect("Unable to read file");

    let mut elf_count = 1;

    file_data
        .lines()
        .for_each(|line| if line == "" { 
            elf_count += 1 
        });

    let mut elf_calories = vec![0; elf_count];
    
    let mut elf_index = 0;

    file_data
        .lines()
        .for_each(|line| if line == "" {
            elf_index += 1;
        } else {
            elf_calories[elf_index] += line.parse::<i32>().unwrap();
        });

    let mut highest_calories:i32 = 0;
    let mut second_highest_calories:i32 = 0;
    let mut third_highest_calories: i32 = 0;
    
    elf_calories
        .iter()
        .for_each(|calories| if *calories > highest_calories {
            second_highest_calories = highest_calories;
            highest_calories = *calories;
        }   else if *calories > third_highest_calories && *calories < second_highest_calories {
            third_highest_calories = *calories;
        });


    println!("Highest calories: {}", highest_calories);
    println!("Calories of top 3: {}", highest_calories + second_highest_calories + third_highest_calories)
}
