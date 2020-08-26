use crate::*;

pub struct Input;

impl Input {
    pub fn read_dimacs_from_stdin() -> String {
        let mut bytes = vec![];
        stdin().read_to_end(&mut bytes).unwrap();

        let mut decoder = GzDecoder::new(&*bytes);
        let mut dimacs = String::new();
        let result = decoder.read_to_string(&mut dimacs);

        if result.is_ok() {
            dimacs
        } else {
            from_utf8(&bytes).unwrap().to_string()
        }
    }

    pub fn read_flips_from_cli_argument() -> u32 {
        args().collect::<Vec<_>>()[1].parse().unwrap()
    }
}
