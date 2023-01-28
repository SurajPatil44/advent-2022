use std::collections::HashSet;
use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn priority(c: char) -> usize {
    let delta = c as u8 - b'A' + 1;
    if delta > 26 {
        return (c as u8 - b'a' + 1) as usize;
    } else {
        return (delta + 26) as usize;
    }
}

fn get_badges(lines: &str) -> char {
    let mut set = HashSet::new();
    for line in lines.lines() {
        //dbg!(&set);
        if set.is_empty() {
            set = line.chars().collect::<_>();
        } else {
            let im_set = line.chars().collect::<HashSet<char>>();
            set = set
                .intersection(&im_set)
                .map(|c| *c)
                .collect::<HashSet<char>>();
        }
    }
    set.iter().map(|c| *c).collect::<Vec<char>>().pop().unwrap()
}

fn duplicates(line: &str) -> Vec<char> {
    let mid: usize = line.len() >> 1;
    let first_part: HashSet<char> = line[..mid].chars().collect::<_>();
    let second_part: HashSet<char> = line[mid..].chars().collect::<_>();
    first_part
        .intersection(&second_part)
        .map(|c| *c)
        .collect::<_>()
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut s = 0;
    let mut group = 3;
    let mut all_lines = String::new();
    loop {
        if group == 0 {
            //dbg!(&all_lines.chars().filter(|c| *c=='\n').count());
            let n = all_lines.len();
            let badge = get_badges(&all_lines[..n - 1]);
            dbg!(badge);
            s += priority(badge);
            all_lines.clear();
            group = 3;
        }
        let n = reader.read_line(&mut line).unwrap();
        if n == 0 {
            break;
        }
        /* part 1
        let dups = duplicates(&line[..n - 1]);
        for d in dups{
            s += priority(d);
        }
        */
        all_lines.write_str(&line);
        line.clear();
        group -= 1;
    }
    dbg!(s);
    //assert_eq!(s,157);
}
