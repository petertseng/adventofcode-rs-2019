pub mod intcode;
pub mod search;

use std::env;
use std::fs;

#[macro_export]
macro_rules! tests {
    (
        $($test_func:ident {
            $( $test_name:ident( $( $param:expr ),* ); )+
        })+
    ) => {
        $(
            $(
                #[test]
                fn $test_name() {
                    $test_func($( $param ),* )
                }
            )+
        )+
    }
}

pub fn numbers<T>(s: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split(|c: char| c != '-' && !c.is_digit(10))
        .filter_map(|d| {
            if d.is_empty() {
                None
            } else {
                Some(d.parse::<T>().expect("can't parse integer"))
            }
        })
        .collect()
}

pub fn read_input_file_or_intcode() -> Vec<i64> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "/dev/stdin".to_string());

    let comma_separated = if filename.contains(',') {
        filename
    } else {
        fs::read_to_string(filename).expect("couldn't read file")
    };

    comma_separated
        .trim()
        .split(',')
        .map(|i| i.parse().expect("can't parse integer"))
        .collect()
}

pub fn read_input_lines<T, F>(f: F) -> Vec<T>
where
    F: FnMut(&str) -> T,
{
    read_input_file().lines().map(f).collect()
}

pub fn read_input_file() -> String {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "/dev/stdin".to_string());
    fs::read_to_string(filename).expect("couldn't read file")
}
