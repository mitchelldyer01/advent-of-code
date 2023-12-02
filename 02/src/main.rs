use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // #1
    // const RED_LIMIT: i32 = 12;
    // const GREEN_LIMIT: i32 = 13;
    // const BLUE_LIMIT: i32 = 14;

    let mut sum_of_ids = 0;

    for line in reader.lines() {
        // trim up to :
        let games = line?;
        let results = games.split(':').last().unwrap();
        // #1
        // let mut possible = true;

        // #2
        let mut red_high_score = 0;
        let mut green_high_score = 0;
        let mut blue_high_score = 0;
        // split on ;

        results.split(';').for_each(|game| {
            game.split(',').for_each(|result| {
                let score = result.trim().split(' ').next().unwrap().parse::<i32>().unwrap();
                // #1
                // if result.contains("red") {
                //     if score > RED_LIMIT {
                //         possible = false;
                //     }
                // }
                // if result.contains("green") {
                //     if score > GREEN_LIMIT {
                //         possible = false;
                //     }
                // }
                // if result.contains("blue") {
                //     if score > BLUE_LIMIT {
                //         possible = false;
                //     }
                // }

                // #2
                if result.contains("red") {
                    if score > red_high_score {
                        red_high_score = score
                    }
                }

                if result.contains("green") {
                    if score > green_high_score {
                        green_high_score = score
                    }
                }

                if result.contains("blue") {
                    if score > blue_high_score {
                        blue_high_score = score
                    }
                }
            });
        });


        // #1
        // if possible {
          //  let id = games.split(':').next().unwrap().split(' ').last().unwrap().parse::<i32>().unwrap();
          //  sum_of_ids = sum_of_ids + id;
          //  println!("{} {}", id, results);
        // }
        
        // #2
        sum_of_ids = sum_of_ids + (red_high_score*green_high_score*blue_high_score);
    }

    println!("{}", sum_of_ids);
    Ok(())
}
