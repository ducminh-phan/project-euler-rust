use std::collections::HashMap;

pub fn get_map() -> HashMap<u32, fn()> {
    HashMap::from([
        (0, p0010::main as fn()),
        (1, p0011::main),
        (2, p0012::main),
        (3, p0013::main),
        (4, p0014::main),
        (5, p0015::main),
        (6, p0016::main),
        (7, p0017::main),
        (8, p0018::main),
        (9, p0019::main),
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
