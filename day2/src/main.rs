use std::{fs::File,io::prelude::*};

#[derive(Copy, Clone, PartialEq, Eq)]
enum gesture {
    Rock,
    Paper,
    Scissor,
}

impl gesture {
    fn from_later(later: char) -> Self {
        match later {
            'A' | 'X' => gesture::Rock,
            'B' | 'Y' => gesture::Paper,
            'C' | 'Z' => gesture::Scissor,
            _ => panic!(),
        }
    }

    fn value(&self) -> u32 {
        match self {
            gesture::Rock => 1,
            gesture::Paper => 2,
            gesture::Scissor => 3,
        }
    }

    fn result(&self, other: &gesture) -> u32 {
        if *self == *other {
            return 3;
        }
        //return (self.value() & other.value()) + (((self.value() & 0x00000002)>>2) |((other.value() & 0x00000002) >> 2)) as u32;
        match (*self, *other) {
            (gesture::Rock, gesture::Paper) => 0,
            (gesture::Rock, gesture::Scissor) => 6,
            (gesture::Paper, gesture::Rock) => 6,
            (gesture::Paper, gesture::Scissor) => 0,
            (gesture::Scissor, gesture::Rock) => 0,
            (gesture::Scissor, gesture::Paper) => 6,
            (_, _) => 3,
        }
    }
}

fn partition(haystack: &str, pin: char) -> (&str, &str) {
    let pos = haystack.find(pin).unwrap();
    (&haystack[..pos], &haystack[pos + 1..])
}

fn main() {
    //println!("Hello, world!");
    let mut data = File::open("input.txt").unwrap();
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    data.read_to_end(&mut buffer).unwrap();
    let buffer = String::from_utf8_lossy(&buffer);
    let mut result = 0;
    let mut ind = 0;
    for (i,line) in buffer.lines().enumerate() {
        //dbg!(partition(line, ' '));
        let laters = partition(line,' ');
        let p1 = gesture::from_later(laters.0.chars().next().unwrap());
        let p2 = gesture::from_later(laters.1.chars().next().unwrap());
        result += p2.value() + p2.result(&p1);
        ind = i;
    }

    dbg!(ind,result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_winners() {
        let a = gesture::Rock;
        let b = gesture::Paper;
        let c = gesture::Scissor;
        assert_eq!(a.result(&b), 0);
        assert_eq!(a.result(&c), 6);
        assert_eq!(b.result(&c), 0);
        assert_eq!(a.result(&a), 3);
    }
}
