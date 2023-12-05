use std::fs::File;
use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut copies: HashMap<usize, usize> = HashMap::new();

    let mut sum = 0;
    let mut sum_of_cards = 0;

    for line in reader.lines() {
        // trim up to :
        let games = line?;
        let results = games.split(':').last().unwrap();
        let current_game = games.split(':').next().unwrap().split(' ').last().unwrap().parse::<usize>().unwrap();

        let copies_of_current_game: usize;
        match copies.get(&current_game) {
            Some(amount) => {
                copies_of_current_game = *amount;
            },
            None => {
                copies_of_current_game = 0;
            }
        }

        let numbers: Vec<&str> = results.split('|').map(|s| s.trim()).collect();

        // 1.
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

                // 2.
                for i in 0..intersection.len() {
                    let scratchcard = current_game+i+1;
                    
                    match copies.get(&scratchcard) {
                        Some(amount) => {
                            copies.insert(scratchcard, 1 + amount + copies_of_current_game);
                        },
                        None => {
                            copies.insert(scratchcard, 1 + copies_of_current_game);
                        },

                    }
                }
            }

            println!("game {}: copies: {}", current_game, copies_of_current_game);

            sum_of_cards += 1 + copies_of_current_game;
        }


    }

    println!("Sum of winning picks: {}", sum);
    println!("Sum of total scratchcards and copies: {}", sum_of_cards);
    Ok(())
}
