pub mod template;

pub(crate) mod continued_fractions;
pub(crate) mod misc;
pub(crate) mod numbers;
pub(crate) mod primes;
pub(crate) mod problems;
pub(crate) mod utils;

mod duration_formatter;

use log::info;
use num::BigUint;
use template::ModuleStructure;

macro_rules! impl_answer {
    ($($tt:ident)*) => {
        paste::paste! {
            #[derive(Debug)]
            enum Answer {
                $(
                    [<$tt:camel>]($tt),
                )*
            }

            $(
                impl From<$tt> for Answer {
                    fn from(value: $tt) -> Self {
                        Self::[<$tt:camel>](value)
                    }
                }

                impl From<&$tt> for Answer {
                    fn from(value: &$tt) -> Self {
                        Self::[<$tt:camel>](value.clone())
                    }
                }
            )*

            impl std::fmt::Display for Answer {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(
                            Answer::[<$tt:camel>](r) => f.write_str(&r.to_string()),
                        )*
                    }
                }
            }
        }
    };
}

impl_answer!(i32 i64 u32 u64 usize f64 String BigUint);

pub fn solve(id: u32) {
    let map = problems::get_map();
    let ms = ModuleStructure::new(id);

    let solve_fn = map
        .get(&&*ms.h_mod)
        .unwrap()
        .get(&&*ms.t_mod)
        .unwrap()
        .get(&&*ms.p_mod)
        .unwrap();

    let start_time = std::time::Instant::now();

    let result = solve_fn();

    let duration = std::time::Instant::now() - start_time;

    println!("{result}");
    info!(
        "Solve #{} in {}",
        id,
        duration_formatter::format_duration(duration)
    );
}
