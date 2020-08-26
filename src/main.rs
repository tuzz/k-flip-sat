mod arithmetic;
mod binary;
mod formula;
mod input;
mod logic;
mod solver;

use crate::arithmetic::*;
use crate::binary::*;
use crate::formula::*;
use crate::input::*;
use crate::logic::*;
use crate::solver::*;
use ipasir_sys::*;
use lazy_static::*;
use std::{cell::RefCell, rc::Rc};
use std::{env::args, ops::Deref};
use std::io::{stdin, Read};

fn main() {
    let dimacs = Input::read_dimacs_from_stdin();
    let _flips = Input::read_flips_from_cli_argument();
    let formula = Formula::parse(&dimacs);

    // Keep our solver's literals in-sync with those in the formula.
    SOLVER.sync_with_formula(&formula);

    // We need ground true/false for some of the circuit reductions.
    SOLVER.set_ground_literals();
}
