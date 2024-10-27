use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};
use std::path::Path;

use log::{info, warn};
use path_macro::path;

pub fn new(id: u32) -> Result<(), Box<dyn Error>> {
    info!("New ID: {}", id);

    let hundreds = id / 100;
    let tens = (id % 100) / 10;
    let ones = id % 10;

    let path = path!(
        "./src/lib/problems"
            / format!("p{hundreds:02}")
            / format!("p{tens}")
            / format!("p{ones}.rs")
    );
    let path = path.as_path();

    if path.exists() {
        warn!("File {path:?} already exists, do nothing!");
        return Ok(());
    }

    let dir = path.parent().unwrap();
    info!("Creating directory: {dir:?}");
    std::fs::create_dir_all(dir)?;

    info!("Writing to file: {path:?}");
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?
        .write(format!("fn main() {{}}\n").as_bytes())?;

    let tens_mod_path = path!(
        "./src/lib/problems"
            / format!("p{hundreds:02}")
            / format!("p{tens}")
            / format!("mod.rs")
    );
    info!("Writing to file: {tens_mod_path:?}");
    add_to_mod(&tens_mod_path, &format!("p{ones}"));

    let hundreds_mod_path = path!(
        "./src/lib/problems" / format!("p{hundreds:02}") / format!("mod.rs")
    );
    info!("Writing to file: {hundreds_mod_path:?}");
    add_to_mod(&hundreds_mod_path, &format!("p{tens}"));

    let problems_mod_path = path!("./src/lib/problems" / format!("mod.rs"));
    info!("Writing to file: {problems_mod_path:?}");
    add_to_mod(&problems_mod_path, &format!("p{hundreds:02}"));

    Ok(())
}

fn add_to_mod(mod_file_path: &Path, name: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(mod_file_path)
        .unwrap();

    let line = format!("pub mod {name};");

    if std::io::BufReader::new(&file)
        .lines()
        .flatten()
        .filter(|_line| (*_line) == line)
        .count()
        == 0
    {
        writeln!(file, "{}", format!("{line}")).unwrap();
    }
}
