use crate::*;

pub struct Input;

impl Input {
    pub fn read_dimacs_from_stdin() -> String {
        let mut dimacs = String::new();
        stdin().read_to_string(&mut dimacs).unwrap();

        dimacs
    }

    pub fn read_flips_from_cli_argument() -> u32 {
        args().collect::<Vec<_>>()[1].parse().unwrap()
    }
}
