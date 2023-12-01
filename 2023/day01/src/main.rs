use std::fs;

// Time complexity: O(n * m) where n is the number of lines and m is the number of characters in each line
// Space complexity: O(1)
fn part_one(data: &str) -> i32 {
    let total = data
        .lines()
        .map(|line| {
            let mut nums: Vec<i32> = Vec::new();
            line
                .chars()
                .for_each(|c| {
                    if c.is_numeric() {
                        if nums.len() == 2 { 
                            nums[1] = c.to_digit(10).unwrap() as i32;
                        } else {
                            nums.push(c.to_digit(10).unwrap() as i32);
                        }
                    }
                });
            if nums.len() == 1 {
                nums.push(nums[0]);
            }

            nums.iter().fold(0, |acc, n| acc * 10 + n)
        })
        .sum();
    return total;
}

fn part_two_helper(s: &str) -> Vec<i32> {
    let mut new_str = Vec::new();
    for (idx, c) in s.chars().enumerate() {
        if c.is_numeric() {
            new_str.push(c.to_digit(10).unwrap() as i32);
        } else {
            let remaining = s.len() - idx + 1;
            if remaining > 3 {
                match &s[idx..idx+3] {
                    "one" => new_str.push(1),
                    "two" => new_str.push(2),
                    "six" => new_str.push(6),
                    _ => (),
                }
            }
            if remaining > 4 {
                match &s[idx..idx+4] {
                    "four" => new_str.push(4),
                    "five" => new_str.push(5),
                    "nine" => new_str.push(9),
                    _ => (),
                }
            }
            if remaining > 5 {
                match &s[idx..idx+5] {
                    "three" => new_str.push(3),
                    "seven" => new_str.push(7),
                    "eight" => new_str.push(8),
                    _ => (),
                }
            }
        }
    }
    return new_str;
}

// Time complexity: O(n * m * k) where n is the number of lines, m is the number of characters in each line, and k is the length of the input string to part_two_helper
// Space complexity: O(k)
fn part_two(data: &str) -> i32 {
    let total = data
        .lines()
        .map(|line| {
            let new_line = part_two_helper(line);
            let first = new_line.first().unwrap().clone();
            let last = new_line.last().unwrap().clone();
            return first * 10 + last 
        })
        .sum();
    return total;
}

fn main() {
    let data = fs::read_to_string("day01")
        .expect("Something went wrong reading the file");      
    println!("Part 1: {} Part 2: {}", part_one(&data), part_two(&data));
}
