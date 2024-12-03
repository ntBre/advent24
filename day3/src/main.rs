use std::error::Error;
use std::fs::read_to_string;

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("../input")?;

    let don = Regex::new(r"do\(\)").unwrap();
    let dont = Regex::new(r"don't\(\)").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let all = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut isdo = true;
    let mut res = 0;
    for m in all.find_iter(&input) {
        let s = m.as_str();
        if don.is_match(s) {
            isdo = true;
        } else if dont.is_match(s) {
            isdo = false;
        } else if isdo && re.is_match(s) {
            let m = re.captures(s).unwrap();
            let a: isize = m[1].parse().unwrap();
            let b: isize = m[2].parse().unwrap();
            res += a * b;
        }
    }

    println!("{}", res);

    Ok(())
}
