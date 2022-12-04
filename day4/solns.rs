use regex::Regex;
use crate::utils::fs;

fn parse_input(buf: String) -> Vec<(u32, u32, u32, u32)> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    re.captures_iter(&buf).map(|grp|
        (grp[1].parse::<u32>().unwrap(),
        grp[2].parse::<u32>().unwrap(),
        grp[3].parse::<u32>().unwrap(),
        grp[4].parse::<u32>().unwrap())).collect()
}

pub fn solution1(fp: &str) -> u32 {
    let parsed = match fs::parse_file(fp, parse_input) {
        Err(e) => {
            println!("{:?}", e);
            return 0;
        },
        Ok(parsed) => parsed,
    };

    parsed.iter().fold(0, |acc, (lp1h, lp1t, lp2h, lp2t)|
        if (lp1h <= lp2h && lp1t >= lp2t) || (lp1h >= lp2h && lp1t <= lp2t) {
            acc + 1
        } else {
            acc
        }
    )
}

pub fn solution2(fp: &str) -> u32 {
    let parsed = match fs::parse_file(fp, parse_input) {
        Err(e) => {
            println!("{:?}", e);
            return 0;
        },
        Ok(parsed) => parsed,
    };
    parsed.iter().fold(0, |acc, (lp1h, lp1t, lp2h, lp2t)|
        if lp1t >= lp2h && lp1h <= lp2t {
            acc + 1
        } else {
            acc
        }
    )
}