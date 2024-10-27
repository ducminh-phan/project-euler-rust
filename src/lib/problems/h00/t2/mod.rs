use std::collections::HashMap;

pub fn get_map() -> HashMap<u32, fn()> {
    HashMap::from([(0, p0::main as fn()), (1, p1::main)])
}

mod p0;
mod p1;
