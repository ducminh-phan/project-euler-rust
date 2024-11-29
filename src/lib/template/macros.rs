#[macro_export]
macro_rules! declare_problems {
    ($($x:ident),* $(,)?) => {
        use std::collections::HashMap;
        use $crate::Answer;

        $(
            mod $x;
        )*

        pub fn get_map<'a>() -> HashMap<&'a str, fn() -> Answer> {
            HashMap::from([
                $(
                    (stringify!($x), $x::solve as fn() -> Answer),
                )*
            ])
        }
    };
}

#[macro_export]
macro_rules! declare_t_collections {
    ($($x:ident),* $(,)?) => {
        use std::collections::HashMap;
        use $crate::Answer;

        $(
            mod $x;
        )*

        pub fn get_map<'a>() -> HashMap<&'a str, HashMap<&'a str, fn() -> Answer>> {
            HashMap::from([
                $(
                    (stringify!($x).into(), $x::get_map()),
                )*
            ])
        }
    };
}

#[macro_export]
macro_rules! declare_h_collections {
    ($($x:ident),* $(,)?) => {
        use std::collections::HashMap;
        use $crate::Answer;

        $(
            mod $x;
        )*

        pub fn get_map<'a>() -> HashMap<&'a str, HashMap<&'a str, HashMap<&'a str, fn() -> Answer>>> {
            HashMap::from([
                $(
                    (stringify!($x).into(), $x::get_map()),
                )*
            ])
        }
    };
}
