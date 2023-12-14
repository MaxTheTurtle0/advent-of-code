use std::fs;

#[derive(Debug)]
struct Card  {
    winning_nums: Vec<u32>,
    owned_nums: Vec<u32>,
}

impl Card {
    fn parse_from_line(line: &str) -> Card {
        let line_new = line.split(": ").skip(1).next().unwrap();
        let nums: Vec<&str> = line_new.split("|").collect();

        let winning_nums = nums[0]
            .split_whitespace()
            .filter_map(|i| i.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let owned_nums = nums[1]
            .split_whitespace()
            .filter_map(|i| i.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        return Card { winning_nums, owned_nums };
    }

    fn num_of_matching_nums(&self) -> u32 {
        let mut num = 0;

        self.owned_nums
            .iter()
            .for_each(|i| {
                if self.winning_nums.contains(i) {
                    num += 1;
                }
            });
        return num;
    }

    fn calc_points(&self, num_of_matchig_nums: u32) -> u32 {
        if num_of_matchig_nums == 0 {
            return 0;
        } 
        let base:u32 = 2; 
        return base.pow(num_of_matchig_nums - 1);
    }
}



fn part_one(data: &str) -> u32 {
    let mut total = 0; 
    data
        .lines()
        .for_each(|line| {
            let card = Card::parse_from_line(line);
            let num_of_matching_nums = card.num_of_matching_nums();
            total += card.calc_points(num_of_matching_nums);
        });
    return total;

}

fn part_two(data: &str) -> u32 {
    todo!();
}

fn main() {
    let data = fs::read_to_string("test")
        .expect("Unable to read file");

    println!("Part one: {} Part two: {}", part_one(&data), part_two(&data));
}
