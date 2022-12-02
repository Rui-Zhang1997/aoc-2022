use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::{Error, ErrorKind};

fn readfile(fp: &str) -> Result<String> {
    let mut f = File::open(fp)?;
    let mut buf = String::new();
    match f.read_to_string(&mut buf) {
        Err(_) => Err(Error::new(
            ErrorKind::NotFound,
            format!("file {} not found", fp),
        )),
        Ok(_) => Ok(buf),
    }
}

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
    let input = match readfile(fp) {
        Err(_) => {
            println!("file not found");
            return 0;
        }
        Ok(buf) => buf,
    };
    let parsed = parse_input(input);
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
    let input = match readfile(fp) {
        Err(_) => {
            println!("file not found");
            return 0;
        }
        Ok(buf) => buf,
    };
    let parsed = parse_input(input);
    parsed.iter().fold(0, |acc, (op, outcome)| -> u32 {
        match outcome {
            'X' => acc + outcome_matrix[op].0,
            'Y' => acc + outcome_matrix[op].1 + 3,
            'Z' => acc + outcome_matrix[op].2 + 6,
            _ => acc,
        }
    })
}
