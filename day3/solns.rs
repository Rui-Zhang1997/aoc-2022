use std::collections::HashSet;
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

fn parse_file(fp: &str) -> Vec<String> {
    let input = match readfile(fp) {
        Err(_) => {
            println!("file not found");
            return vec![];
        }
        Ok(buf) => buf,
    };
    parse_input(input)
}

fn parse_input(buf: String) -> Vec<String> {
    return buf.split("\n").map(String::from).collect();
}

pub fn solution1(fp: &str) -> u32 {
    let (chr_a, chr_ca, chr_cz) = ('a' as u8, 'A' as u8, 'Z' as u8);
    let packages = parse_file(&fp);
    packages.iter().fold(0, |acc, pkg| -> u32 {
        let mut contents: HashSet<u8> = HashSet::new();
        let hlf = pkg.len() / 2;
        let pkg_chrs = pkg.as_bytes();
        for i in 0..hlf {
            contents.insert(pkg_chrs[i]);
        }

        let mut sum: u32 = 0;
        for i in 0..hlf {
            let p = pkg_chrs[hlf + i];
            if contents.contains(&p) {
                contents.remove(&p);
                let vp = if p <= chr_cz {
                    p - chr_ca + 27
                } else {
                    p - chr_a + 1
                };
                sum += vp as u32;
            }
        }
        acc + sum
    })
}

// pub fn solution2(fp: &str) -> u32 {
//     return 0;
// }