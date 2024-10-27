use std::collections::HashMap;

pub fn get_map() -> HashMap<u32, HashMap<u32, fn()>> {
    HashMap::from([(0, t0::get_map()), (1, t1::get_map()), (2, t2::get_map())])
}

mod t0;
mod t1;
mod t2;
