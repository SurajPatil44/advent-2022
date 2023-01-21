use std::collections::BinaryHeap;
use std::{fs::File, io::prelude::*};

fn get_sum(inp: &Vec<u8>) -> u32 {
    let inp = String::from_utf8_lossy(inp);
    let mut it = inp
        .lines()
        .map(|c| c.parse::<u32>().unwrap())
        .reduce(|acc, e| acc + e)
        .unwrap();
    //dbg!(it);
    it
}

fn main() {
    let mut data = File::open("input.txt").unwrap();
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    data.read_to_end(&mut buffer).unwrap();
    let mut index = 0u32;
    let mut it = buffer.iter();
    let mut cur = it.next().unwrap();
    let mut i_buffer: Vec<u8> = Vec::with_capacity(1024);
    let mut found = false;
    let mut indices = BinaryHeap::new();
    loop {
        match it.next() {
            Some(next) => {
                //dbg!(*cur,*next);
                if *cur == b'\n' && *next == b'\n' {
                    index += 1;
                    //dbg!(&i_buffer);
                    let sum = get_sum(&i_buffer);
                    //dbg!(index, sum);
                    indices.push((sum, index));
                    i_buffer.clear();
                    found = true;
                } else {
                    //dbg!(cur);
                    if !found {
                        i_buffer.push(*cur);
                    } else {
                        found = false;
                    }
                    //dbg!(i_buffer);
                    cur = next;
                }
            }
            None => break,
        }
    }
    let ans = indices.pop().unwrap().0 + indices.pop().unwrap().0 + indices.pop().unwrap().0;
    dbg!(ans);
}
