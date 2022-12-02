use std::env;
use std::fs;

pub fn read_file(fp: String) -> String {
    let mut f = fs::open(fp);
    let mut buf = String::new();

    f.read_to_string(&mut buf);
    return buf;
}
