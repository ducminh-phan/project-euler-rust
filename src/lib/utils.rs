use std::env;
use std::fs::OpenOptions;
use std::io::Read;

pub fn read_file<P: AsRef<str>>(path: P, split_by: char) -> Vec<String> {
    let file = OpenOptions::new().read(true).open(path.as_ref()).unwrap();

    let mut content = String::new();
    std::io::BufReader::new(file)
        .read_to_string(&mut content)
        .unwrap();

    content
        .trim()
        .split(split_by)
        .map(|s| s.replace('"', ""))
        .collect()
}

pub fn word_score<S: AsRef<str>>(s: S) -> u32 {
    s.as_ref()
        .as_bytes()
        .iter()
        .map(|c| (c - 65 + 1) as u32)
        .sum()
}

pub fn parse_env<T: std::str::FromStr>(name: &str, default: T) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    env::var(name)
        .map(|s| s.parse())
        .map(|v| v.unwrap())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use crate::numbers::num_from_digits;

    #[test]
    fn test_num_from_digits() {
        assert_eq!(num_from_digits(vec![4, 2, 3, 1]), 4231);
        assert_eq!(num_from_digits(vec![2, 3, 5, 7]), 2357);
        assert_eq!(num_from_digits(vec![3, 1, 4, 1, 6]), 31416);
        assert_eq!(num_from_digits(vec![0, 1, 4, 1, 6]), 1416);
    }
}
