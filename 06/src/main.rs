use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Part one: {}", part_one()?);
    println!("Part two: {}", part_two()?);

    Ok(())
}

fn part_one() -> Result<i64, Box<dyn Error>> {
    let mut product = 1;

    let mut races = parse_input_into_races("input.txt")?;

    races.iter_mut().for_each(|race| { 
        race.count_number_of_wins(); 
        product *= race.number_of_wins;
    });

    Ok(product)
}

fn part_two() -> Result<i64, Box<dyn Error>> {
    let mut race = parse_input_into_one_race("input.txt")?;

    race.count_number_of_wins();

    Ok(race.number_of_wins)
}

#[derive(Copy, Clone)]
struct Race {
    time: i64,
    record_distance: i64,
    number_of_wins: i64,
}

impl Race {
    fn count_number_of_wins(&mut self) {
        for t in 1..self.time {
            let actual_distance = (self.time - t)*t;  
            if actual_distance > self.record_distance {
                self.number_of_wins += 1;
            }
        }
    } 
}

fn parse_input_into_one_race(file_path: &str) -> Result<Race, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    let time: String = lines.next().unwrap()?.chars().filter(|c| c.is_digit(10)).collect();
    let distance: String = lines.next().unwrap()?.chars().filter(|c| c.is_digit(10)).collect();

    Ok(Race { time: time.parse()?, record_distance: distance.parse()?, number_of_wins: 0 })

}

fn parse_input_into_races(file_path: &str) -> Result<Vec<Race>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut races: Vec<Race> = vec![];

    let mut lines = reader.lines();

    let times: Vec<i64> = lines.next().unwrap()?.split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect();
    let distances: Vec<i64> = lines.next().unwrap()?.split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect();

    for i in 0..times.len() {
        races.push(Race { time: times[i], record_distance: distances[i], number_of_wins: 0 });
    }

    Ok(races)
}

#[test]
fn test_parse_into_races() {
    let races = parse_input_into_races("input.txt").unwrap();

    assert_eq!(races[0].time, 62);
    assert_eq!(races[0].record_distance, 553);

    assert_eq!(races[1].time, 64);
    assert_eq!(races[1].record_distance, 1010);

    assert_eq!(races[2].time, 91);
    assert_eq!(races[2].record_distance, 1473);

    assert_eq!(races[3].time, 90);
    assert_eq!(races[3].record_distance, 1074);
}

#[test]
fn test_count_number_of_wins() {
    let mut first = Race { time: 7, record_distance: 9, number_of_wins: 0};
    let mut second = Race { time: 15, record_distance: 40, number_of_wins: 0};
    let mut third = Race { time: 30, record_distance: 200, number_of_wins: 0};
    let mut fourth = Race { time: 71530, record_distance: 940200, number_of_wins: 0};

    first.count_number_of_wins();
    second.count_number_of_wins();
    third.count_number_of_wins();
    fourth.count_number_of_wins();

    assert_eq!(first.number_of_wins, 4);
    assert_eq!(second.number_of_wins, 8);
    assert_eq!(third.number_of_wins, 9);
    assert_eq!(fourth.number_of_wins, 71503);
}
