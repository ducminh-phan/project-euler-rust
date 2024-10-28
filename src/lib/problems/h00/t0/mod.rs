use std::collections::HashMap;

pub fn get_map() -> HashMap<u32, fn()> {
    HashMap::from([
        (1, p0001::main as fn()),
        (2, p0002::main),
        (3, p0003::main),
        (4, p0004::main),
        (5, p0005::main),
        (6, p0006::main),
        (7, p0007::main),
        (8, p0008::main),
        (9, p0009::main),
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
