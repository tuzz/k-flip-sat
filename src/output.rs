use crate::*;

// Use the competition output format for Max-SAT solvers as specified here:
// https://maxsat-evaluations.github.io/2020/rules.html

pub struct Output<'a> {
    formula: &'a Formula,
    previous: u32,
    assignments: Vec<i32>,
}

// Whether to output variables in the 2020 compact format specified here:
// https://maxsat-evaluations.github.io/2020/vline.html
const COMPACT_FORMAT: bool = true;

impl<'a> Output<'a> {
    pub fn new(formula: &'a Formula) -> Self {
        Self { formula, previous: 0, assignments: vec![] }
    }

    pub fn print_number_of_flips(&self, num_flips_binary: &[i32], k_flips: u32) {
        if self.previous == 0 { return; }

        let num_flips = Binary::decode(num_flips_binary);
        let num_vars = self.formula.num_vars;
        let pluralized = if num_flips == 1 { "variable" } else { "variables" };

        println!("c flipped {}/{} {} (k={})", num_flips, num_vars, pluralized, k_flips);
    }

    pub fn print_clause_progress(&mut self, num_unsatisfied: u32) {
        if self.previous == 0 {
            let num_clauses = self.formula.clauses.len() as u32;
            let num_satisfied = num_clauses - num_unsatisfied;

            println!("c random assignment satisfied {}/{} clauses", num_satisfied, num_clauses);
        } else {
            let improvement = self.previous - num_unsatisfied;
            let pluralized = if improvement == 1 { "clause" } else { "clauses" };

            println!("c solution improved by {} {}", improvement, pluralized);
        }

        self.previous = num_unsatisfied;
    }

    pub fn print_solution_cost(&self) {
        println!("o {}", self.previous);
    }

    pub fn remember_assignments(&mut self) {
        self.assignments.clear();

        for var in 1..=self.formula.num_vars as i32 {
            let boolean = SOLVER.assignment(var);
            let literal = if boolean { var } else { -var };

            self.assignments.push(literal);
        }
    }

    pub fn print_assigned_variables(&self) {
        print!("v");

        if COMPACT_FORMAT {
            print!(" ");

            for literal in &self.assignments {
                print!("{}", if *literal > 0 { 1 } else { 0 });
            }
        } else {
            for literal in &self.assignments {
                print!(" {}", literal);
            }
        }

        println!();
    }

    pub fn print_whether_solved(&self, k_flips: u32) {
        let num_clauses = self.formula.clauses.len() as u32;
        let num_satisfied = num_clauses - self.previous;

        if self.previous == 0 {
            println!("c completely solved all clauses with k={}", k_flips);
            println!("s OPTIMUM FOUND");
        } else {
            println!("c stuck at {}/{} clauses with k={}", num_satisfied, num_clauses, k_flips);
            println!("s UNKNOWN");
        }
    }
}
