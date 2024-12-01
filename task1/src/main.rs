use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;


fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn aoc_task(input: &str) -> Result<(i64, i64), ParseIntError> {
    let input: Vec<&str> = input.split('\n').collect();
    let input_len = input.len() - 1;
    let mut left_col: Vec<i64> = Vec::with_capacity(input_len);
    let mut right_col: Vec<i64> = Vec::with_capacity(input_len);

    for elem in input {
        let left_right: Vec<&str> = elem.split_whitespace().collect();
        if !left_right.is_empty() {
            left_col.push(left_right[0].parse::<i64>()?);
            right_col.push(left_right[1].parse::<i64>()?);
        }
    }

    left_col.sort();
    right_col.sort();
    let mut apperance: HashMap<i64, usize> = HashMap::with_capacity(input_len);
    for elem in &right_col {
        apperance.insert(*elem, match apperance.get(elem) {
            Some(v) => v + 1,
            None => 1
        });
    }

    let mut result = 0;
    let mut similarity_score = 0;

    for i in 0..input_len {
        result += (left_col[i] - right_col[i]).abs();
        similarity_score += left_col[i] * (match apperance.get(&left_col[i]) {
            Some(v) => *v as i64,
            None => 0 as i64
        });
    }

    Ok((result, similarity_score))
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let filename = "input.txt";

    let input = read_file(filename)?;

    let (result, similarity) = aoc_task(input.as_str()).unwrap();

    println!("Result: {}, Similarity: {}", result, similarity);

    Ok(())
}
