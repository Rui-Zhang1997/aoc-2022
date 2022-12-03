use std::collections::HashSet;
use crate::utils::fs;

fn parse_input(buf: String) -> Vec<String> {
    return buf.split("\n").map(String::from).collect();
}

pub fn solution1(fp: &str) -> u32 {
    let (chr_a, chr_ca, chr_cz) = ('a' as u8, 'A' as u8, 'Z' as u8);
    let packages = match fs::parse_file(&fp, parse_input) {
        Err(e) => {
            println!("{:?}", e);
            return 0;
        },
        Ok(parsed) => parsed,
    };
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