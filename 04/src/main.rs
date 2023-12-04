use std::fs::File;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        // trim up to :
        let games = line?;
        let results = games.split(':').last().unwrap();

        let numbers: Vec<&str> = results.split('|').map(|s| s.trim()).collect();
        if let [winners, picks] = numbers.as_slice() {
            let set_winners: HashSet<u32> = winners
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let set_picks: HashSet<u32> = picks
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let intersection: HashSet<_> = set_winners.intersection(&set_picks).collect();

            if intersection.len() > 0 {
                let local_sum = usize::pow(2, (intersection.len()-1).try_into().unwrap());
                sum += local_sum;
            }
        }
    }

    println!("Sum of winning picks: {}", sum);
    Ok(())
}
