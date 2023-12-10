use std::{fs, collections::HashMap};

fn part_one(data: &str) -> i32 {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let instructions = data.split_once("\n\n").unwrap().0;
    data
        .lines()
        .skip(2)
        .for_each(|line| {
            let name = &line[0..3];
            let parts = line[7..line.len() - 1]
                .split(", ")
                .collect::<Vec<&str>>();
            let parts = (parts[0], parts[1]);
            nodes.insert(name, parts);
        });


    let mut idx = 0;
    let mut curr_node = "AAA";
    let mut steps = 0;
    loop {
        if idx == instructions.len() {
            idx = 0;
        }

        if curr_node == "ZZZ" {
            break;
        }

        match instructions.chars().nth(idx).unwrap() {
            'L' => {
                if let Some((left, _)) = nodes.get(curr_node) {
                    curr_node = left;
                } else {
                    panic!("Invalid instruction");
                }
            }
            'R' => {
                if let Some((_, right)) = nodes.get(curr_node) {
                    curr_node = right;
                } else {
                    panic!("Invalid instruction");
                }
            }
            _ => panic!("Invalid instruction"),
        };
        idx += 1;
        steps += 1;
    }
    return steps;
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn calculate_lcm(numbers: Vec<u64>) -> u64 {
    let mut result = 1;
    for &num in numbers.iter() {
        result = lcm(result, num);
    }
    result
}


fn part_two(data: &str) -> u64 {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let instructions = data.split_once("\n\n").unwrap().0;
    data
        .lines()
        .skip(2)
        .for_each(|line| {
            let name = &line[0..3];
            let parts = line[7..line.len() - 1]
                .split(", ")
                .collect::<Vec<&str>>();
            let parts = (parts[0], parts[1]);
            nodes.insert(name, parts);
        });
   
    let starting_nodes = nodes.keys().filter(|node| node.ends_with("A"));
    let mut steps_vec: Vec<u64> = Vec::new();  

    for node in starting_nodes {
        let mut idx = 0;
        let mut curr_node = node;
        let mut steps = 0;
        loop {
            if idx == instructions.len() {
                idx = 0;
            }

            if curr_node.ends_with("Z") {
                break;
            }

            match instructions.chars().nth(idx).unwrap() {
                'L' => {
                    if let Some((left, _)) = nodes.get(curr_node) {
                        curr_node = left;
                    } else {
                        panic!("Invalid instruction");
                    }
                }
                'R' => {
                    if let Some((_, right)) = nodes.get(curr_node) {
                        curr_node = right;
                    } else {
                        panic!("Invalid instruction");
                    }
                }
                _ => panic!("Invalid instruction"),
            };
            idx += 1;
            steps += 1;
        }
        steps_vec.push(steps);
    }
    return calculate_lcm(steps_vec);
}



fn main() {
    let data = fs::read_to_string("day08")
        .expect("Unable to read file");
    println!("Part 1: {}, Part 2: {}", part_one(&data), part_two(&data));
}
