use std::io;
use std::collections::HashMap;

fn first(line: &str) -> String {
    let words = line.split(' ');
    let mut map = HashMap::new();
    for word in words {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    let mut v = map.into_iter().collect::<Vec<_>>();
    v.sort_by(|x, y| y.1.cmp(&x.1));

    let mut ret = String::new();
    for m in v {
        ret += &format!("{} - {}\n", m.0, m.1);
    }
    ret
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    println!("{}", first(&line));
}
