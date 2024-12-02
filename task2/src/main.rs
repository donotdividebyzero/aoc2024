use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn prepare_data(input: &str) -> Vec<Vec<i64>> {
    input
        .split('\n')
        .collect::<Vec<_>>()
        .into_iter()
        .map(|levels: &str| {
            levels
                .split_whitespace()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|n: &str| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn run_solution(input: &str, solution: impl Fn(Vec<i64>) -> i64) -> Result<i64, ParseIntError> {
    let data = prepare_data(input);
    Ok(data.into_iter().map(solution).sum())
}

fn direction_way(step: i64) -> i64 {
    if step > 0 {
        1
    } else {
        -1
    }
}

fn is_proper_step(step: i64) -> bool {
    step != 0 && step.abs() <= 3
}

fn aoc_task1(levels: Vec<i64>) -> i64 {
    // RULES:
    //    The levels are either all increasing or all decreasing.
    //    Any two adjacent levels differ by at least one and at most three.
    if levels.len() < 2 {
        return 0;
    }

    let levels_count = levels.len() - 1;
    let main_dir = direction_way(levels[1] - levels[0]);

    for i in 0..levels_count {
        let step = levels[i + 1] - levels[i];

        let is_proper_step = is_proper_step(step);

        if !is_proper_step || direction_way(step) != main_dir {
            return 0;
        }
    }

    1
}

fn aoc_task2(levels: Vec<i64>) -> i64 {
    // RULES:
    //    The levels are either all increasing or all decreasing.
    //    Any two adjacent levels differ by at least one and at most three.
    //    You can have one wrong step
    if levels.len() < 2 {
        return 0;
    }

    let levels_count = levels.len() - 1;
    let mut main_dir = 0;

    let mut incline: u32 = 0;
    for i in 0..levels_count {
        let step = levels[i + 1] - levels[i];

        let is_proper_step = is_proper_step(step);

        if is_proper_step && main_dir == 0 {
            main_dir = direction_way(step);
        }

        if is_proper_step && direction_way(step) == main_dir {
            incline |= 1 << i;
        }
    }

    let correct_steps = incline.count_ones();
    if (correct_steps == levels_count as u32) || (correct_steps == (levels_count as u32 - 1)) {
        1
    } else {
        0
    }
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::*;

    static FILENAME: &'static str = "input.txt";
    static SOL1: i64 = 218;
    static SOL2: i64 = 290;

    #[test]
    fn test_sol1() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_file(FILENAME)?;
        assert_eq!(SOL1, run_solution(input.as_str(), aoc_task1).unwrap());

        Ok(())
    }

    #[test]
    fn test_sol2() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_file(FILENAME)?;
        assert_eq!(SOL2, run_solution(input.as_str(), aoc_task2).unwrap());

        Ok(())
    }
}
