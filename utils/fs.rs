use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::io::{Error, ErrorKind};

pub fn readfile(fp: &str) -> Result<String> {
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

pub fn parse_file<T>(fp: &str, f: fn(String) -> T) -> Result<T> {
    match readfile(fp) {
        Err(_) => Err(Error::new(ErrorKind::NotFound, format!("file {} not found", fp))),
        Ok(buf) => Ok(f(buf))
    }
}