use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "input2.txt";

    let input = read_file(filename)?;

    println!("{}", count(input.as_str()));

    Ok(())
}

fn count(input: &str) -> usize {
    let mut word = [0; 4];
    let map = input.split(|c| c == '\n').collect::<Vec<_>>();
    (0..map[0].len() as isize)
        .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
        .map(|(x, y)| {
            [
                (x + 1, y + 1), // Center
                (x, y),         // NE
                (x, y + 2),     // SE
                (x + 2, y),     // NW
                (x + 2, y + 2), // SW
            ]
        })
        .filter(|coords| {
            let mut iter = coords.iter().map(|(x, y)| {
                map.get(*y as usize)
                    .and_then(|row| row.as_bytes().get(*x as usize).copied())
                    .unwrap_or_default()
            });

            // if A is not in center we definetaly do not have our solution
            if iter.next().is_none_or(|n| n != b'A') {
                return false;
            }

            word.fill_with(|| iter.next().unwrap_or_default());
            &word == b"MMSS" || &word == b"MSMS" || &word == b"SSMM" || &word == b"SMSM"
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        assert_eq!(18, count(input));
    }
}
