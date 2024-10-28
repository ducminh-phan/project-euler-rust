use std::collections::HashMap;

pub fn get_map() -> HashMap<u32, fn()> {
    HashMap::from([
        (0, p0020::main as fn()),
        (1, p0021::main),
        (2, p0022::main),
        (3, p0023::main),
        (4, p0024::main),
        (5, p0025::main),
    ])
}

mod p0020;
mod p0021;
mod p0022;
mod p0023;
mod p0024;
mod p0025;
