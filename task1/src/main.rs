use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;


fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn aoc_task(input: &str) -> Result<i64, ParseIntError> {
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

    let mut result = 0;

    for i in 0..input_len {
        result += (left_col[i] - right_col[i]).abs();
    }

    Ok(result)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let filename = "input.txt";

    let input = read_file(filename)?;

    let result = aoc_task(input.as_str());

    println!("Result: {}", result.unwrap());

    Ok(())
}
