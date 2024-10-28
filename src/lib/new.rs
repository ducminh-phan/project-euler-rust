use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};
use std::path::Path;

use log::{info, warn};
use path_macro::path;

use crate::problems::ModuleStructure;

pub fn new(id: u32) -> Result<(), Box<dyn Error>> {
    info!("New ID: {}", id);

    let ms = ModuleStructure::new(id);

    let path = path!(
        "./src/lib/problems" / ms.h_mod / ms.t_mod / format!("{}.rs", ms.p_mod)
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
        .write_all("pub fn solve() {}\n".as_bytes())?;

    let tens_mod_path =
        path!("./src/lib/problems" / ms.h_mod / ms.t_mod / "mod.rs");
    info!("Writing to file: {tens_mod_path:?}");
    add_to_mod(&tens_mod_path, &ms.p_mod);

    let hundreds_mod_path = path!("./src/lib/problems" / ms.h_mod / "mod.rs");
    info!("Writing to file: {hundreds_mod_path:?}");
    add_to_mod(&hundreds_mod_path, &ms.t_mod);

    let problems_mod_path = path!("./src/lib/problems/mod.rs");
    info!("Writing to file: {problems_mod_path:?}");
    add_to_mod(&problems_mod_path, &ms.h_mod);

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
