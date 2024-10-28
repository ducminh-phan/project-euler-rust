use std::collections::HashMap;

pub fn get_map() -> HashMap<u32, fn()> {
    HashMap::from([
        (1, p0001::solve as fn()),
        (2, p0002::solve),
        (3, p0003::solve),
        (4, p0004::solve),
        (5, p0005::solve),
        (6, p0006::solve),
        (7, p0007::solve),
        (8, p0008::solve),
        (9, p0009::solve),
    ])
}

mod p0001;
mod p0002;
mod p0003;
mod p0004;
mod p0005;
mod p0006;
mod p0007;
mod p0008;
mod p0009;
