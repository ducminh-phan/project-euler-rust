use std::error::Error;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use log::{debug, info, warn};
use minijinja as j2;
use path_macro::path;
pub use utils::ModuleStructure;

mod macros;
mod utils;

const BASE_PROBLEMS_MOD_PATH: &str = "./src/lib/problems";

pub fn new(id: u32) -> Result<(), Box<dyn Error>> {
    info!("New ID: {}", id);

    let ms = ModuleStructure::new(id);
    let j2_env = create_env();

    write_p_mod(&j2_env, &ms)?;
    rebuild_modules(&j2_env)?;

    Ok(())
}

fn create_env() -> j2::Environment<'static> {
    let mut env = j2::Environment::new();
    env.set_loader(j2::path_loader("./templates"));
    env.set_keep_trailing_newline(true);
    env
}

fn write_p_mod(
    j2_env: &j2::Environment,
    ms: &ModuleStructure,
) -> Result<(), Box<dyn Error>> {
    let path = path!(
        BASE_PROBLEMS_MOD_PATH
            / ms.h_mod
            / ms.t_mod
            / format!("{}.rs", ms.p_mod)
    );
    let path = path.as_path();

    if path.exists() {
        warn!("File {path:?} already exists, skip writing!");
        return Ok(());
    }

    let dir = path.parent().unwrap();
    debug!("Creating directory: {dir:?}");
    std::fs::create_dir_all(dir)?;

    // Fetch description and format as module docs
    let description = reqwest::blocking::get(format!(
        "https://projecteuler.net/minimal={}",
        ms.id,
    ))?
    .text()?
    .replace("<p>", "\n")
    .replace("</p>", "\n")
    .replace("\n\n\n", "\n\n")
    .replace("$", "`")
    .trim()
    .lines()
    .map(html_escape::decode_html_entities)
    .map(|line| format!("//! {line}"))
    .join("\n");

    write_file(
        path,
        j2_env
            .get_template("p_mod.j2")
            .unwrap()
            .render(j2::context! {description => description})
            .unwrap()
            .as_bytes(),
    )?;

    Ok(())
}

fn rebuild_modules(j2_env: &j2::Environment) -> Result<(), Box<dyn Error>> {
    let problems_mod_dir = Path::new(BASE_PROBLEMS_MOD_PATH);
    let h_mod_dirs = get_sub_mods(problems_mod_dir);
    let h_mod_names = get_names(&h_mod_dirs);

    let problems_mod_file = problems_mod_dir.join("mod.rs");
    write_file(
        problems_mod_file,
        j2_env
            .get_template("problems_mod.j2")
            .unwrap()
            .render(j2::context! {h_mods => h_mod_names})
            .unwrap()
            .as_bytes(),
    )?;

    h_mod_dirs
        .iter()
        .for_each(|h_mod_dir| rebuild_h_mod(j2_env, h_mod_dir).unwrap());

    Ok(())
}

fn rebuild_h_mod(
    j2_env: &j2::Environment,
    h_mod_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let t_mod_dirs = get_sub_mods(h_mod_dir);
    let t_mod_names = get_names(&t_mod_dirs);

    let h_mod_path = h_mod_dir.join("mod.rs");
    write_file(
        h_mod_path,
        j2_env
            .get_template("h_mod.j2")
            .unwrap()
            .render(j2::context! {t_mods => t_mod_names})
            .unwrap()
            .as_bytes(),
    )?;

    t_mod_dirs
        .iter()
        .for_each(|t_mod_dir| rebuild_t_mod(j2_env, t_mod_dir).unwrap());

    Ok(())
}

fn rebuild_t_mod(
    j2_env: &j2::Environment,
    t_mod_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let p_mods = get_sub_mods(t_mod_dir);
    let p_mod_names = get_names(&p_mods);

    let t_mod_path = t_mod_dir.join("mod.rs");
    write_file(
        t_mod_path,
        j2_env
            .get_template("t_mod.j2")
            .unwrap()
            .render(j2::context! {p_mods => p_mod_names})
            .unwrap()
            .as_bytes(),
    )?;

    Ok(())
}

fn write_file<P: AsRef<Path> + Debug>(
    path: P,
    buf: &[u8],
) -> std::io::Result<()> {
    debug!("Writing to file: {path:?}");
    OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?
        .write_all(buf)?;

    Ok(())
}

fn get_sub_mods(path: impl AsRef<Path>) -> Vec<PathBuf> {
    path.as_ref()
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .map(|d| d.path())
        .filter(|p| p.file_name() != Some("mod.rs".as_ref()))
        .collect::<Vec<_>>()
}

fn get_names(paths: &[PathBuf]) -> Vec<&str> {
    paths
        .iter()
        .map(|p| p.file_stem().unwrap().to_str().unwrap())
        .sorted()
        .collect::<Vec<_>>()
}
