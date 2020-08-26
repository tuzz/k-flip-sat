use crate::*;

// Use the standardised output format for SAT solvers as specified here:
// http://beyondnp.org/static/media/uploads/docs/satformat.pdf

pub struct Output<'a> {
    formula: &'a Formula,
    previous: u32,
    assignments: Vec<i32>,
}

impl<'a> Output<'a> {
    pub fn new(formula: &'a Formula) -> Self {
        Self { formula, previous: 0, assignments: vec![] }
    }

    pub fn print_number_of_flips(&self, num_flips_binary: &[i32], k_flips: u32) {
        let num_flips = Binary::decode(num_flips_binary);
        let num_vars = self.formula.num_vars;
        let pluralized = if num_flips == 1 { "variable" } else { "variables" };

        println!("c flipped {}/{} {} (k={})", num_flips, num_vars, pluralized, k_flips);
    }

    pub fn print_clause_progress(&mut self, num_unsatisfied: u32) {
        if self.previous != 0 {
            let improvement = self.previous - num_unsatisfied;
            let pluralized = if improvement == 1 { "clause" } else { "clauses" };

            println!("c solution improved by {} {}", improvement, pluralized);
        }

        self.previous = num_unsatisfied;
    }

    pub fn print_solution_line(&self, num_unsatisfied: u32) {
        let num_clauses = self.formula.clauses.len() as u32;
        let num_satisfied = num_clauses - num_unsatisfied;

        println!("s max {} {} {}", num_satisfied, self.formula.num_vars, num_clauses);
    }

    pub fn remember_assignments(&mut self) {
        self.assignments.clear();

        for var in 1..self.formula.num_vars as i32 {
            let boolean = SOLVER.assignment(var);
            let literal = if boolean { var } else { -var };

            self.assignments.push(literal);
        }
    }

    pub fn print_assigned_variables(&self) {
        print!("v");

        for literal in &self.assignments {
            print!(" {}", literal);
        }

        println!();
    }

    pub fn print_whether_solved(&self, k_flips: u32) {
        let num_clauses = self.formula.clauses.len() as u32;
        let num_satisfied = num_clauses - self.previous;

        if self.previous == 0 {
            println!("c completely solved all clauses with k={}", k_flips);
        } else {
            println!("c stuck at {}/{} clauses with k={}", num_satisfied, num_clauses, k_flips);
        }
    }
}
