use std::fs::OpenOptions;
use std::io::Read;

pub fn read_file<P: AsRef<str>>(path: P) -> Vec<String> {
    let file = OpenOptions::new().read(true).open(path.as_ref()).unwrap();

    let mut content = String::new();
    std::io::BufReader::new(file)
        .read_to_string(&mut content)
        .unwrap();

    content.split(',').map(|s| s.replace('"', "")).collect()
}

pub fn word_score<S: AsRef<str>>(s: S) -> u32 {
    s.as_ref()
        .as_bytes()
        .iter()
        .map(|c| (c - 65 + 1) as u32)
        .sum()
}

pub fn num_from_digits<DS: AsRef<[u8]>>(digits: DS) -> u64 {
    let digits = digits.as_ref();
    let n = digits.len();
    digits
        .iter()
        .enumerate()
        .map(|(i, d)| (*d as u64) * 10u64.pow((n - i - 1) as u32))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::utils::num_from_digits;

    #[test]
    fn test_num_from_digits() {
        assert_eq!(num_from_digits(vec![4, 2, 3, 1]), 4231);
        assert_eq!(num_from_digits(vec![2, 3, 5, 7]), 2357);
        assert_eq!(num_from_digits(vec![3, 1, 4, 1, 6]), 31416);
        assert_eq!(num_from_digits(vec![0, 1, 4, 1, 6]), 1416);
    }
}