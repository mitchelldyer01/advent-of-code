use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_input_into_vec("input.txt")?;

    let part_one: i64 = input.iter().map(|vec| predict(vec.to_vec())).sum();
    let part_two: i64 = input.iter().map(|vec| predict(vec.iter().rev().copied().collect())).sum();

    println!("Part one: {:#?}", part_one);
    println!("Part two: {:#?}", part_two);
    Ok(())
}

fn predict(mut diffs: Vec<i64>) -> i64 {
    let mut prediction = 0;

    while diffs.iter().any(|&d| d != 0) {
        prediction += update_diffs(&mut diffs);
    }

    prediction
}

fn update_diffs(values: &mut Vec<i64>) -> i64 {
    for i in 0..values.len() - 1 {
        values[i] = values[i + 1] - values[i];
    }

    values.pop().unwrap()
}

fn parse_input_into_vec(file_path: &str) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    let mut vector_of_vectors: Vec<Vec<i64>> = vec![];

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mut local_vector: Vec<i64> = vec![];
        let text = line?;
        text.split_whitespace().for_each(|element| local_vector.push(element.parse::<i64>().unwrap()));
        vector_of_vectors.push(local_vector);
    }

    Ok(vector_of_vectors)
}
