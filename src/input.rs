use crate::*;

pub struct Input;

impl Input {
    pub fn read_dimacs_from_stdin() -> String {
        let mut bytes = vec![];
        stdin().read_to_end(&mut bytes).unwrap();

        if let Some(dimacs) = decode_gzipped_input(&bytes) {
            return dimacs;
        }

        if let Some(dimacs) = decode_xz_input(&bytes) {
            return dimacs;
        }

        from_utf8(&bytes).unwrap().to_string()
    }

    pub fn read_flips_from_cli_argument() -> u32 {
        args().collect::<Vec<_>>()[1].parse().unwrap()
    }
}

fn decode_gzipped_input(bytes: &[u8]) -> Option<String> {
    let mut decoder = GzDecoder::new(bytes);
    let mut string = String::new();

    match decoder.read_to_string(&mut string) {
        Ok(_) => Some(string),
        Err(_) => None,
    }
}

fn decode_xz_input(bytes: &[u8]) -> Option<String> {
    let mut decoder = XzDecoder::new(bytes);
    let mut string = String::new();

    match decoder.read_to_string(&mut string) {
        Ok(_) => Some(string),
        Err(_) => None,
    }
}
