use std::fs;

struct Game {
    id: u64,
    red: Vec<u64>,
    green: Vec<u64>,
    blue: Vec<u64>,
}

impl Default for Game {
    fn default() -> Game {
        Game { id: 0, red: vec![0], green: vec![0], blue: vec![0]}
    }
}

impl Game {
    fn parse_from_line(line: &str) -> Game { 
        let mut line_parts = line.split_whitespace();
        line_parts.next();
        let Some(second_part_game_id) = line_parts.next() else {
            panic!("Could not parse game id");
        };

        let game_id_str = second_part_game_id.trim_end_matches(':'); 
        let Ok(game_id) = game_id_str.parse::<u64>() else {
            panic!("Could not parse game id");
        };

        let mut tmp_num: u64 = 0;
        let mut idx: usize = 0;
        let mut game = Game { id: game_id, ..Default::default() };
        for str in line_parts.into_iter() {
            let try_u64 = str.parse::<u64>(); 
            match try_u64 {
                Ok(ok) => {
                    tmp_num = ok;
                }
                _ => {
                    match &str[0..1] { 
                        "r" => { game.red[idx] += tmp_num; },
                        "g" => { game.green[idx] += tmp_num; },
                        "b" => { game.blue[idx] += tmp_num; },
                        _ => {
                            panic!("Could not parse color");
                        } 
                    }
                    if str.ends_with(";") {
                        idx += 1; 
                        game.red.push(0);
                        game.green.push(0);
                        game.blue.push(0);
                    }
                } 
            }
        }
        return game; 
    }

    fn is_possible(&self) -> bool {
        // check if any of the colors are over 12, 13, 14 respectively
        for i in 0..self.red.len() {
            if self.red[i] > 12 {
                return false;
            }
        }
        for i in 0..self.green.len() {
            if self.green[i] > 13 {
                return false;
            }
        }
        for i in 0..self.blue.len() {
            if self.blue[i] > 14 {
                return false;
            }
        }
        return true;
    }
    
    // returns a vector of the max numbers of each color
    // [0] = red, [1] = green, [2] = blue
    // didnt want to use a hashmap because it wasnt necessary
    fn check_max_colors(&self) -> Vec<u64> {
        return vec![self.red.iter().max().unwrap().to_owned(), self.green.iter().max().unwrap().to_owned(), self.blue.iter().max().unwrap().to_owned()];
    }
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut red_str = String::new();
        let mut green_str = String::new();
        let mut blue_str = String::new();
        for i in 0..self.red.len() {
            red_str.push_str(&format!("{} ", self.red[i]));
        }
        for i in 0..self.green.len() {
            green_str.push_str(&format!("{} ", self.green[i]));
        }
        for i in 0..self.blue.len() {
            blue_str.push_str(&format!("{} ", self.blue[i]));
        }
        write!(f, "Game {{ id: {}, red: {}, green: {}, blue: {} }}", self.id, red_str, green_str, blue_str) 
    }
}

fn part_one(data: &str) -> u64 {
    let mut games: Vec<Game> = Vec::new();
    data
        .lines()
        .for_each(|line| {
            games.push(Game::parse_from_line(line));
        });
     
    let mut game_id_sum = 0;
    games
        .iter()
        .filter(|game| game.is_possible())
        .for_each(|game| {
            game_id_sum += game.id;
        });

    return game_id_sum;
}

fn part_two(data: &str) -> u64 {
    let mut games: Vec<Game> = Vec::new();

    data
        .lines()
        .for_each(|line| {
            games.push(Game::parse_from_line(line));
        });
    let mut game_multiplier_sum = 0;

    games
        .iter()
        .for_each(|game|{
            let max_colors = game.check_max_colors();
            let mut multiplyied = 1;
            max_colors.iter().for_each(|color| {
                multiplyied *= color;
            });
            game_multiplier_sum += multiplyied;
        });

    return game_multiplier_sum;
}

fn main() {
   let data = fs::read_to_string("day02")
       .expect("Something went wrong reading the file");
    println!("Part 1: {}, Part 2: {}", part_one(&data), part_two(&data));
}
