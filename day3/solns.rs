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
        let hlf = pkg.len() / 2;
        let mut contents: HashSet<u8> = HashSet::from_iter(pkg.as_bytes()[0..hlf].iter().cloned());

        let mut sum: u32 = 0;
        for p in pkg.as_bytes()[hlf..pkg.len()].iter() {
            if contents.contains(&p) {
                contents.remove(&p);
                let vp = if *p <= chr_cz {
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

fn intersect(h1: &HashSet<u8>, h2: &HashSet<u8>) -> HashSet<u8> {
    HashSet::from_iter(h1.iter().filter(|v| h2.contains(&v)).cloned())
}

fn to_set(vs: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(vs.iter().cloned())
}

pub fn solution2(fp: &str) -> u32 {
    let (chr_a, chr_ca, chr_cz) = ('a' as u8, 'A' as u8, 'Z' as u8);
    let packages = match fs::parse_file(fp, parse_input) {
        Err(e) => {
            println!("{:?}", e);
            return 0;
        },
        Ok(parsed) => parsed,
    };
    (2..packages.len()).step_by(3).fold(0, |acc, idx| -> u32 {
        let h1: HashSet<u8> = to_set(packages[idx].as_bytes());
        let h2: HashSet<u8> = to_set(packages[idx-1].as_bytes());
        let h3: HashSet<u8> = to_set(packages[idx-2].as_bytes());
        let ident = intersect(&h1, &intersect(&h2, &h3));
        let cm = ident.iter().next().cloned().unwrap();
        acc + if cm <= chr_cz {
            (cm - chr_ca + 27) as u32
        } else {
            (cm - chr_a + 1) as u32
        }
    })
}