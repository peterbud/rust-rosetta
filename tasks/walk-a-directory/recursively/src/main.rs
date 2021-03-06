// http://rosettacode.org/wiki/Walk_a_directory/Recursively
extern crate regex;

use regex::Regex;
use std::convert::AsRef;
use std::fs;
use std::path::Path;

fn walk<P>(pth: P, regex: &Regex)
where
    P: AsRef<Path>,
{
    let result = match fs::read_dir(pth) {
        Ok(result) => result,
        Err(_) => return,
    };

    for subp in result.into_iter().flatten() {
        if let Ok(filename) = subp.path().into_os_string().into_string() {
            if regex.is_match(&filename) {
                println!("{}", filename);
            }
            walk(filename, regex);
        }
    }
}

fn main() {
    walk(".", &Regex::new(r".*\.rs").unwrap());
}
