use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let input = read_to_string("../input")?;

    let mut safe = 0;
    'outer: for line in input.lines() {
        let sp: Vec<_> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();

        if is_safe(&sp) {
            safe += 1;
        } else {
            for i in 0..sp.len() {
                let mut x = sp.clone();
                x.remove(i);
                if is_safe(&x) {
                    safe += 1;
                    continue 'outer;
                }
            }
        }
    }

    println!("{}", safe);

    Ok(())
}

fn is_safe(sp: &[isize]) -> bool {
    let mut last = sp[0];
    let mut asc: Option<bool> = None;
    for l in &sp[1..] {
        let diff = last - l;
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 && asc.is_some_and(|a| !a) {
            return false;
        }
        if diff < 0 && asc.is_some_and(|a| a) {
            return false;
        }
        asc = Some(diff > 0);
        last = *l;
    }
    true
}
