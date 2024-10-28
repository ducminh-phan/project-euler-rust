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
