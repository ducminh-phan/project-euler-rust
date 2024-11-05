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
