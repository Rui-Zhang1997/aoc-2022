use std::collections::HashMap;
use crate::utils::fs;

fn parse_input(buf: String) -> Vec<(char, char)> {
    let parts = buf.split("\n");
    parts
        .map(|rps| -> (char, char) { (rps.chars().nth(0).unwrap(), rps.chars().nth(2).unwrap()) })
        .collect()
}

pub fn solution1(fp: &str) -> u32 {
    let result_map: HashMap<char, (u32, u32, u32)> =
        HashMap::from([
            ('A', (3, 6, 0)),
            ('B', (0, 3, 6)),
            ('C', (6, 0, 3))]);
    let parsed = match fs::parse_file(fp, parse_input) {
        Err(e) => {
            println!("{:?}", e);
            return 0;
        }
        Ok(parsed) => parsed
    };
    parsed.iter().fold(0, |acc, (op, me)| -> u32 {
        match me {
            'X' => acc + result_map[op].0 + 1,
            'Y' => acc + result_map[op].1 + 2,
            'Z' => acc + result_map[op].2 + 3,
            _ => acc,
        }
    })
}

pub fn solution2(fp: &str) -> u32 {
    let outcome_matrix: HashMap<char, (u32, u32, u32)> =
        HashMap::from([
            ('A', (3, 1, 2)),
            ('B', (1, 2, 3)),
            ('C', (2, 3, 1))]);
    let parsed = match fs::parse_file(fp, parse_input) {
        Err(e) => {
            println!("{:?}", e);
            return 0;
        }
        Ok(parsed) => parsed,
    };
    parsed.iter().fold(0, |acc, (op, outcome)| -> u32 {
        match outcome {
            'X' => acc + outcome_matrix[op].0,
            'Y' => acc + outcome_matrix[op].1 + 3,
            'Z' => acc + outcome_matrix[op].2 + 6,
            _ => acc,
        }
    })
}
