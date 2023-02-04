use utils::Content;

fn get_row(string: &str) -> Vec<Option<char>> {
    let mut res = Vec::<Option<char>>::new();
    for chunk in string.as_bytes().chunks(4) {
        let temp = chunk[1] as char;
        if temp != ' ' {
            res.push(Some(temp));
        } else {
            res.push(None);
        }
    }
    //res.as_mut_slice().reverse();
    res
}

fn clean_raw_data(raw: &Vec<String>) -> Vec<Vec<Option<char>>> {
    let mut res = Vec::<Vec<Option<char>>>::new();
    //let mut res = Vec::Vec<char>::new();
    let last = raw.last().unwrap().split("  ");
    //dbg!(last.next());
    for _ in last {
        res.push(Vec::<Option<char>>::new());
    }
    for row in &raw[..raw.len() - 1] {
        let temp = get_row(&row);
        for (idx, t) in temp.iter().enumerate() {
            res[idx].push(*t);
        }
    }
    for r in &mut res {
        r.as_mut_slice().reverse();
    }

    for r in &mut res {
        loop {
            if r.last().unwrap().is_none() {
                r.pop();
            } else {
                break;
            }
        }
    }
    res
}

fn main() {
    let mut reader = Content::read_from_file("input.txt");
    let mut raw_data = Vec::<String>::new();

    // g
    loop {
        let line = reader.next();
        match line {
            Some(l) => {
                if l == "" {
                    break;
                }
                //dbg!(&l);
                raw_data.push(l);
            }
            None => break,
        }
    }
    let mut data = clean_raw_data(&raw_data);
    //dbg!(&data);
    for line in reader {
        let parts: Vec<&str> = line.split(' ').collect();
        let nums = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;
        //dbg!(&data[to].last());
        /*
        for _ in 0..nums {
            let container = data[from].pop().unwrap();
            data[to].push(container);
        }*/
        let mut temp: Vec<Option<char>> = Vec::new();
        for _ in 0..nums {
            temp.push(data[from].pop().unwrap());
        }
        temp.as_mut_slice().reverse();
        data[to].extend_from_slice(&temp);
    }
    let mut out = String::new();
    for d in data {
        out.push(d.last().unwrap().unwrap());
    }
    dbg!(&out);
}
