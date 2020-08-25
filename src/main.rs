mod formula;
mod input;

use crate::formula::Formula;
use crate::input::Input;

fn main() {
    let dimacs = Input::read_dimacs_from_stdin();
    let flips = Input::read_flips_from_cli_argument();
    let formula = Formula::parse(&dimacs);

    println!("{:?}, {}", formula, flips);
}
