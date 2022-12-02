use std::fs::File;
use std::io::Result;
use std::io::{ Error, ErrorKind };
use std::io::prelude::*;

fn readfile(fp: &str) -> Result<String> {
    let mut f = File::open(fp)?;
    let mut buf = String::new();
    match f.read_to_string(&mut buf) {
        Err(_) => Err(Error::new(ErrorKind::NotFound, format!("file {} not found", fp))),
        Ok(_) => Ok(buf)
    }
}

fn parse_input(buf: String) -> Vec<Vec<u32>> {
    let parts = buf.split("\n");
    let mut result: Vec<Vec<u32>> = Vec::new();
    let mut vec: Vec<u32> = Vec::new();
    for p in parts {
        if p == "" {
            result.push(vec.clone());
            vec.clear();
        } else {
            match p.parse::<u32>() {
                Err(_) => println!("unable to parse: {}", p),
                Ok(val) => vec.push(val),
            };
        }
    }
    result
}

pub fn solution(fp: &str) {
    let input = match readfile(fp) {
        Err(_) => {
            println!("file not found");
            return;
        },
        Ok(buf) => buf,
    };
    let parsed = parse_input(input);
    let mut sums = Vec::new();
    for vl in parsed.iter() {
        let mut s = 0;
        for v in vl.iter() {
            s += v;
        }
        sums.push(s);
    }
    sums.sort();
    let sz = sums.len();
    println!("max: {}", sums[sz-1]);
    println!("sum 3: {}", sums[sz-1] + sums[sz-2] + sums[sz-3]);
}