use utils::{partition, Content};

#[derive(Debug)]
struct Bound {
    lower: usize,
    upper: usize,
}

impl Bound {
    fn dashed_notation(note: &str) -> Self {
        let bounds = partition(&note, '-');
        Bound {
            lower: bounds.0.parse().unwrap(),
            upper: bounds.1.parse().unwrap(),
        }
    }

    fn contains(&self, other: &Bound) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &Bound) -> bool {
        self.lower <= other.lower && self.upper >= other.lower
    }
}

fn main() {
    let reader = Content::read_from_file("input.txt");
    let mut s = 0;
    for line in reader {
        let parts = partition(&line, ',');
        let first = Bound::dashed_notation(parts.0);
        let second = Bound::dashed_notation(parts.1);
        //if first.contains(&second) || second.contains(&first) {
        if first.overlaps(&second) || second.overlaps(&first) {
            s += 1;
        }
    }
    dbg!(s);
}
