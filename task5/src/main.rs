use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::rc::Rc;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn parse_rule(input: &str) -> (usize, usize) {
    let input = input
        .split('|')
        .into_iter()
        .map(|n| n.parse::<usize>())
        .filter_map(|n| n.ok())
        .collect::<Vec<usize>>();

    (input[0], input[1])
}

fn parse_update(input: &str) -> Vec<usize> {
    input
        .split(',')
        .into_iter()
        .map(|n| n.parse::<usize>())
        .filter_map(|n| n.ok())
        .collect()
}

type Rules = HashMap<usize, Rc<RefCell<Vec<usize>>>>;

fn parse_input(input: &str) {
    let mut rules: Rules  = HashMap::new();
    let mut answer = 0;

    let input = input.lines();

    for line in input {
        if line.is_empty() {
            continue;
        }

        if line.contains('|') {
            let (x, y) = parse_rule(line);
            if let Some(vec) = rules.get(&x) {
                vec.borrow_mut().push(y);
            } else {
                rules.insert(x, Rc::new(RefCell::new(vec![y])));
            }
        }

        #[inline]
        fn test_update(update: &Vec<usize>, rules: &Rules) -> bool {
            for (idx, v) in update.iter().enumerate() {
                if let Some(rule) = rules.get(v) {
                   let rule = rule.borrow(); 
                   let update_slice = &update[..idx];
                   for r in rule.iter() {
                       if update_slice.contains(r) {
                           return false;
                       }
                   }
                }
            }

            true
        }

        #[inline]
        fn fix_update(update: &mut Vec<usize>, rules: &Rules) -> Vec<usize> {
            let update_clone = update.clone();
            for (idx, v) in update_clone.iter().enumerate() {
                if let Some(rule) = rules.get(v) {
                   let rule = rule.borrow(); 
                   let update_slice = &update_clone[..idx];
                   for r in rule.iter() {
                       if let Some(ridx) = update_slice.iter().position(|e| e == r) {
                            update.swap(idx, ridx);
                            return update.to_vec();
                       }
                   }
                }
            }

            update.to_vec()
        }

        if line.contains(',') {
            let mut update = parse_update(line);
            if !test_update(&update, &rules) {
                while !test_update(&update, &rules) {
                    update = fix_update(&mut update, &rules); 
                }

                if let Some(middle) = update.get(update.len()/2) {
                    answer+=middle;
                }
            }
        }
    }


    println!("{}", answer);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "input2.txt";
    let input = read_file(filename)?;

    parse_input(input.as_str());

    Ok(())
}
