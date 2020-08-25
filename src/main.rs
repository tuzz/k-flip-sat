mod formula;
mod input;
mod solver;

use crate::formula::*;
use crate::input::*;
use crate::solver::*;
use ipasir_sys::*;
use lazy_static::*;
use std::cell::RefCell;
use std::env::args;
use std::io::{stdin, Read};

fn main() {
    let dimacs = Input::read_dimacs_from_stdin();
    let flips = Input::read_flips_from_cli_argument();
    let formula = Formula::parse(&dimacs);
    let solver = Solver::new();

    solver.add(1);
    solver.add(0);
    solver.run();

    println!("{:?}", solver.assignment(1));
}
