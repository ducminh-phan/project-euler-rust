use std::collections::HashMap;

pub fn solve(id: u32) {
    let hundreds = id / 100;
    let tens = (id % 100) / 10;
    let ones = id % 10;

    let map = HashMap::from([(0, h00::get_map())]);

    let solve_fn = map
        .get(&hundreds)
        .unwrap()
        .get(&tens)
        .unwrap()
        .get(&ones)
        .unwrap();

    solve_fn();
}

mod h00;
