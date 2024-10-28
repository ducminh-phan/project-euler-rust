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

    let h_mod = format!("h{hundreds:02}");
    let t_mod = format!("t{tens}");
    let p_mod = format!("p{id:04}");

    let path =
        path!("./src/lib/problems" / h_mod / t_mod / format!("{p_mod}.rs"));
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
        .write_all("pub fn solve() {}\n".as_bytes())?;

    let tens_mod_path = path!("./src/lib/problems" / h_mod / t_mod / "mod.rs");
    info!("Writing to file: {tens_mod_path:?}");
    add_to_mod(&tens_mod_path, &p_mod);

    let hundreds_mod_path = path!("./src/lib/problems" / h_mod / "mod.rs");
    info!("Writing to file: {hundreds_mod_path:?}");
    add_to_mod(&hundreds_mod_path, &t_mod);

    let problems_mod_path = path!("./src/lib/problems/mod.rs");
    info!("Writing to file: {problems_mod_path:?}");
    add_to_mod(&problems_mod_path, &h_mod);

    Ok(())
}

fn add_to_mod(mod_file_path: &Path, name: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(mod_file_path)
        .unwrap();

    let line = format!("mod {name};");

    if std::io::BufReader::new(&file)
        .lines()
        .map_while(Result::ok)
        .filter(|_line| (*_line) == line)
        .count()
        == 0
    {
        writeln!(file, "{}", line).unwrap();
    }
}
