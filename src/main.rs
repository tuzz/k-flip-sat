mod arithmetic;
mod binary;
mod formula;
mod input;
mod logic;
mod reduction;
mod solver;

use crate::arithmetic::*;
use crate::binary::*;
use crate::formula::*;
use crate::input::*;
use crate::logic::*;
use crate::solver::*;
use crate::reduction::*;
use ipasir_sys::*;
use lazy_static::*;
use rand::prelude::*;
use std::{cell::RefCell, rc::Rc};
use std::{env::args, ops::Deref};
use std::io::{stdin, Read};

fn main() {
    let dimacs = Input::read_dimacs_from_stdin();
    let k_flips = Input::read_flips_from_cli_argument();
    let formula = Formula::parse(&dimacs);

    // Keep our solver's literals in-sync with those in the formula.
    SOLVER.sync_with_formula(&formula);

    // We need ground true/false for some of the circuit reductions.
    SOLVER.set_ground_literals();

    let mut reduction = Reduction::new(formula);
    let unsats = reduction.add_clauses_with_unsat_literals();
    let num_unsat = reduction.count_unsatisfied_clauses(&unsats);
    let unsat_limit = Binary::of_length(num_unsat.len());
    reduction.must_be_less_than(&num_unsat, &unsat_limit);

    let assignments = reduction.generate_one_literal_per_literal();
    let flips = reduction.add_clauses_that_capture_flips(&assignments);
    let num_flips = reduction.count_flipped_literals(&flips);
    let flips_limit = Binary::of_length(num_flips.len());
    reduction.must_be_less_than(&num_flips, &flips_limit);

    // The number of allowed flips doesn't change per iteration.
    Binary::assign(&flips_limit, k_flips + 1);

    // Randomly assign true/false to each assigned literal.
    reduction.start_by_assuming_random_assignments(&assignments);

    while SOLVER.run() { // Until there are no more solutions.
        let threshold = Binary::decode(&num_unsat);

        reduction.set_assignments_to_those_of_the_solution(&assignments);

        // The number of unsat clauses strictly decreases per iteration.
        Binary::assume(&unsat_limit, threshold);
    }
}
