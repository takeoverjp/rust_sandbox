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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        // arrange 
        let input = "White tigers live mostly in India Wild lions live mostly in Africa".to_string();
        let expected = "live - 2\nmostly - 2\nin - 2\nAfrica - 1\nWhite - 1\nWild - 1\nlions - 1\nAfrica - 1\nIndia - 1\n".to_string();

        // act
        let actual = first(&input);

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_same_rank() {
        // arrange 
        let input = "a a a b b c".to_string();
        let expected = "a - 3\nb - 2\nc - 1\n".to_string();

        // act
        let actual = first(&input);

        // assert
        assert_eq!(expected, actual);
    }
}
