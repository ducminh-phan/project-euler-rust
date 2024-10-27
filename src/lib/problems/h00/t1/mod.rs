use std::collections::HashMap;

pub fn get_map() -> HashMap<u32, fn()> {
    HashMap::from([
        (0, p0::main as fn()),
        (1, p1::main),
        (2, p2::main),
        (3, p3::main),
        (4, p4::main),
        (5, p5::main),
        (6, p6::main),
        (7, p7::main),
        (8, p8::main),
        (9, p9::main),
    ])
}

mod p0;
mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;
