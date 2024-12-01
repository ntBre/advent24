use std::{collections::HashSet, fs::read_to_string, process::exit};

fn main() {
    let input = read_to_string("../input").unwrap();

    let mut fst: HashSet<isize> = HashSet::new();
    let mut snd: Vec<isize> = Vec::new();
    for line in input.lines() {
        let [a, b] = &line.split_ascii_whitespace().collect::<Vec<_>>()[..]
        else {
            exit(1);
        };
        fst.insert(a.parse().unwrap());
        snd.push(b.parse().unwrap());
    }

    snd.sort();

    let mut dist = 0;
    for f in fst {
        let mut s = snd.clone();
        s.retain(|x| *x == f);
        dist += f * s.len() as isize;
    }

    println!("{}", dist);
}
