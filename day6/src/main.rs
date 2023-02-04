use std::collections::HashSet;
use utils::Content;

fn process(window: &[u8], win_size: usize) -> bool {
    let set: HashSet<u8> = window.iter().map(|c| *c).collect();
    //dbg!(&window,&set);
    set.len() == win_size
}

fn main() {
    let mut reader = Content::read_from_file("sample.txt");
    let line = reader.next().unwrap();
    let win_size = 14;
    let mut pos = 0;
    for (idx, win) in line.as_bytes().windows(win_size).enumerate() {
        if process(&win, win_size) {
            pos = idx + win_size;
            break;
        }
    }
    dbg!(pos);
}
