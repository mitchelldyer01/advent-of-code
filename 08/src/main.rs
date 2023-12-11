use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part one: {:#?}", part_one()?);
    println!("Part two: {:#?}", part_two()?);
    Ok(())
}

fn part_one() -> Result<usize, Box<dyn Error>> {
    let mut map = parse_input_into_map("input.txt")?;
    let steps = map.traverse("ZZZ".to_string());

    Ok(steps)
}

fn part_two() -> Result<usize, Box<dyn Error>> {
    let map = parse_input_into_map("input.txt")?;

    let mut all_steps = vec![];

    let starting_points: Vec<String> = map.addresses.iter().filter(|(key, _value)| key.ends_with('A')).map(|(key, _value)| key.to_string()).collect();

    starting_points.iter().for_each(|point| {
        let mut current_location = point.to_string();
        let mut steps = 0;
        let mut i = 0;

        while !current_location.ends_with('Z') {
            steps += 1;

            if i == map.directions.len() { i = 0; }
            
            match map.directions[i] {
                'L' => {
                    current_location = map.addresses.get(&current_location).unwrap().left.to_string();
                }, 
                'R' => {
                    current_location = map.addresses.get(&current_location).unwrap().right.to_string();
                },
                _ => {},
            }
            i += 1;
        }
        all_steps.push(steps);
    });

    println!("{:#?}", all_steps);
    Ok(lcm(&all_steps))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(numbers: &[usize]) -> usize {
    numbers.iter().cloned().fold(1, |acc, num| acc * num / gcd(acc, num))
}

fn parse_input_into_map(file_path: &str) -> Result<Map, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    // directions on first line
    let directions: Vec<char> = lines.next().unwrap()?.chars().collect();

    let mut map = Map::new("AAA".to_string(), directions);

    for line in lines {
        let text = line?;
        if text.is_empty() { continue }

        // 0 = here
        // 2 = (left,
        // 3 = right)
        let here_left_right: Vec<&str> = text.split_whitespace().collect();

        let here = here_left_right.iter().nth(0).unwrap().to_string();
        let left = here_left_right.iter().nth(2).unwrap().trim_start_matches('(').trim_end_matches(',').to_string();
        let right = here_left_right.iter().nth(3).unwrap().trim_end_matches(')').to_string();

        let address = Address::new(here, left, right);

        map.addresses.insert(here_left_right.iter().nth(0).unwrap().to_string(), address);
    }

    Ok(map)
}

#[derive(Debug)]
struct Address {
    here: String,
    left: String,
    right: String,
}

impl Address {
    fn new(here: String, left: String, right: String) -> Self {
        Address { here, left, right }
    }

    fn here(&mut self, here: String) {
        self.here = here;
    }

    fn left(&mut self, left: String) {
        self.left = left;
    }

    fn right(&mut self, right: String) {
        self.right = right;
    }
}

struct Map {
    addresses: HashMap<String, Address>,
    location: String,
    directions: Vec<char>,
}

impl Map {
    fn new(location: String, directions: Vec<char>) -> Self {
        Map {
            addresses: HashMap::new(),
            location,
            directions,
        }
    }

    fn go_left(&mut self) {
        let left = &self.addresses.get(&self.location).unwrap().left;
        self.location = left.to_string();
    }

    fn go_right(&mut self) {
        let right = &self.addresses.get(&self.location).unwrap().right;
        self.location = right.to_string();
    }

    fn traverse(&mut self, destination: String) -> usize {
        let mut steps = 0;
        let mut i = 0;

        while self.location != destination {
            steps += 1;
            
            if i == self.directions.len() { i = 0; }

            match self.directions[i] {
                'L' => self.go_left(),
                'R' => self.go_right(),
                _ => {},
            }

            i += 1;
        }

        steps
    }
}

#[test]
fn test_traverse() {
    let directions: Vec<char> = "RL".chars().collect(); 
    let directions_two: Vec<char> = "LLR".chars().collect();

    let mut map = Map::new("AAA".to_string(), directions);
    map.addresses.insert("AAA".to_string(), Address::new("AAA".to_string(), "BBB".to_string(), "CCC".to_string()));
    map.addresses.insert("BBB".to_string(), Address::new("BBB".to_string(), "DDD".to_string(), "EEE".to_string()));
    map.addresses.insert("CCC".to_string(), Address::new("CCC".to_string(), "ZZZ".to_string(), "GGG".to_string()));
    map.addresses.insert("DDD".to_string(), Address::new("DDD".to_string(), "DDD".to_string(), "DDD".to_string()));
    map.addresses.insert("EEE".to_string(), Address::new("EEE".to_string(), "EEE".to_string(), "EEE".to_string()));
    map.addresses.insert("GGG".to_string(), Address::new("GGG".to_string(), "GGG".to_string(), "GGG".to_string()));
    map.addresses.insert("ZZZ".to_string(), Address::new("ZZZ".to_string(), "ZZZ".to_string(), "ZZZ".to_string()));

    let mut map_two = Map::new("AAA".to_string(), directions_two);
    map_two.addresses.insert("AAA".to_string(), Address::new("AAA".to_string(), "BBB".to_string(), "BBB".to_string()));
    map_two.addresses.insert("BBB".to_string(), Address::new("BBB".to_string(), "AAA".to_string(), "ZZZ".to_string()));
    map_two.addresses.insert("ZZZ".to_string(), Address::new("ZZZ".to_string(), "ZZZ".to_string(), "ZZZ".to_string()));

    let steps = map.traverse("ZZZ".to_string());
    let steps_two = map_two.traverse("ZZZ".to_string());

    assert_eq!(steps, 2);
    assert_eq!(steps_two, 6);
}
