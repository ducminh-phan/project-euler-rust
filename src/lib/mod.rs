pub mod template;

pub(crate) mod continued_fractions;
pub(crate) mod misc;
pub(crate) mod numbers;
pub(crate) mod primes;
pub(crate) mod problems;
pub(crate) mod utils;

mod duration_formatter;

use log::info;
use template::ModuleStructure;

pub fn solve(id: u32) {
    let map = problems::get_map();
    let ms = ModuleStructure::new(id);

    let solve_fn = map
        .get(&&*ms.h_mod)
        .unwrap()
        .get(&&*ms.t_mod)
        .unwrap()
        .get(&&*ms.p_mod)
        .unwrap();

    let start_time = std::time::Instant::now();

    solve_fn();

    let duration = std::time::Instant::now() - start_time;
    info!(
        "Solve #{} in {}",
        id,
        duration_formatter::format_duration(duration)
    );
}
