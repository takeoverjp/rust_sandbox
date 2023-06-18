use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::read_to_string;
use std::collections::HashSet;

fn first(line: &str) -> String {
    let path = Path::new("data/stop_words.txt");
    let mut file = File::open(&path).unwrap();
    let mut stop_words = String::new();
    file.read_to_string(&mut stop_words).unwrap();
    let stop_words = stop_words.split(',').collect::<HashSet<&str>>();
    // println!("{:?}", stop_words);

    let words = line.split(' ');
    let mut map = HashMap::new();
    for word in words {
        if stop_words.contains(&word) {
            continue;
        }
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
    for line in  read_to_string(&env::args().nth(1).unwrap()).unwrap().lines() {
        println!("{}", first(&line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        // arrange 
        let input = "White tigers live mostly in India Wild lions live mostly in Africa".to_string();
        let expected = "live - 2\nmostly - 2\nAfrica - 1\nWhite - 1\nWild - 1\nlions - 1\nAfrica - 1\nIndia - 1\n".to_string();

        // act
        let actual = first(&input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_same_rank() {
        // arrange 
        let input = "a a a b b c".to_string();
        let expected = "b - 2\nc - 1\n".to_string();

        // act
        let actual = first(&input);

        // assert
        assert_eq!(expected, actual);
    }
}
