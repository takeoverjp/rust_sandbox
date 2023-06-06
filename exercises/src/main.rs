use std::io;
use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let words = line.split(' ');
    let mut map = HashMap::new();
    for word in words {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    let mut v = map.into_iter().collect::<Vec<_>>();
    v.sort_by(|x, y| y.1.cmp(&x.1));

    for m in v {
        println!("{} - {}", m.0, m.1);
    }
}
