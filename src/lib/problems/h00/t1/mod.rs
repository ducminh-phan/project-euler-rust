use std::collections::HashMap;

pub fn get_map() -> HashMap<u32, fn()> {
    HashMap::from([
        (0, p0010::solve as fn()),
        (1, p0011::solve),
        (2, p0012::solve),
        (3, p0013::solve),
        (4, p0014::solve),
        (5, p0015::solve),
        (6, p0016::solve),
        (7, p0017::solve),
        (8, p0018::solve),
        (9, p0019::solve),
    ])
}

mod p0010;
mod p0011;
mod p0012;
mod p0013;
mod p0014;
mod p0015;
mod p0016;
mod p0017;
mod p0018;
mod p0019;
