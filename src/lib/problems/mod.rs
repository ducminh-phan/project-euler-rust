use std::collections::HashMap;

#[macro_export]
macro_rules! declare_problems {
    ($($x:ident),+ $(,)?) => {
        use std::collections::HashMap;

        $(
            mod $x;
        )*

        pub fn get_map<'a>() -> HashMap<&'a str, fn()> {
            HashMap::from([
                $(
                    (stringify!($x), $x::solve as fn()),
                )*
            ])
        }
    };
}

#[macro_export]
macro_rules! declare_t_collections {
    ($($x:ident),+ $(,)?) => {
        use std::collections::HashMap;

        $(
            mod $x;
        )*

        pub fn get_map<'a>() -> HashMap<&'a str, HashMap<&'a str, fn()>> {
            HashMap::from([
                $(
                    (stringify!($x).into(), $x::get_map()),
                )*
            ])
        }
    };
}

pub struct ModuleStructure {
    pub hundreds: u32,
    pub tens: u32,

    pub h_mod: String,
    pub t_mod: String,
    pub p_mod: String,
}

impl ModuleStructure {
    pub fn new(id: u32) -> Self {
        let hundreds = id / 100;
        let tens = (id % 100) / 10;

        let h_mod = format!("h{hundreds:02}");
        let t_mod = format!("t{tens}");
        let p_mod = format!("p{id:04}");

        Self {
            hundreds,
            tens,
            h_mod,
            t_mod,
            p_mod,
        }
    }
}

pub fn solve(id: u32) {
    let map = HashMap::from([(0, h00::get_map())]);
    let ms = ModuleStructure::new(id);

    let solve_fn = map
        .get(&ms.hundreds)
        .unwrap()
        .get(&&*ms.t_mod)
        .unwrap()
        .get(&&*ms.p_mod)
        .unwrap();

    solve_fn();
}

mod h00;
