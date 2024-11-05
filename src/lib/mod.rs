pub mod numbers;
pub mod primes;
pub mod problems;
pub mod template;
pub mod utils;

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

    solve_fn();
}
