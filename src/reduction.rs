use crate::*;

pub struct Reduction<'a> {
    formula: &'a Formula,
    random: SmallRng,
}

impl<'a> Reduction<'a> {
    pub fn new(formula: &'a Formula) -> Self {
        let random = SmallRng::from_entropy();

        Self { formula, random }
    }

    pub fn add_clauses_with_unsat_literals(&self) -> Vec<i32> {
        self.formula.clauses.iter().map(|clause| {
            let mut clause = clause.clone();
            let unsat_literal = SOLVER.new_literal();

            clause.push(unsat_literal);
            SOLVER.add_clause(&clause);

            unsat_literal
        }).collect::<Vec<_>>()
    }

    pub fn count_unsatisfied_clauses(&self, unsats: &[i32]) -> Vec<i32> {
        counter(unsats)
    }

    pub fn must_be_less_than(&self, number: &[i32], threshold: &[i32]) {
        SOLVER.add_clause(&[less_than(number, &threshold)]);
    }

    pub fn generate_one_literal_per_literal(&self) -> Vec<i32> {
        (1..=self.formula.num_vars).map(|_| {
            SOLVER.new_literal()
        }).collect()
    }

    pub fn add_clauses_that_capture_flips(&self, assignments: &[i32]) -> Vec<i32> {
        (1..).zip(assignments).map(|(formula_literal, assignment)| {
            let is_flipped = SOLVER.new_literal();

            // If the variable is assumed to be true, then either the literal
            // must be true or it has been flipped (and vice versa).
            SOLVER.add_clause(&[-*assignment, formula_literal, is_flipped]);
            SOLVER.add_clause(&[*assignment, -formula_literal, is_flipped]);

            is_flipped
        }).collect()
    }

    pub fn count_flipped_literals(&self, flips: &[i32]) -> Vec<i32> {
        counter(flips)
    }

    pub fn start_by_assuming_random_assignments(&mut self, assignments: &[i32]) {
        for var in assignments {
            let boolean = *[true, false].choose(&mut self.random).unwrap();
            let literal = if boolean { *var } else { -*var };

            SOLVER.assume(literal);
        }
    }

    pub fn set_assignments_to_those_of_the_solution(&self, assignments: &[i32]) {
        let from_solution = assignments.iter().zip(1..).map(|(var, literal)| {
            let boolean = SOLVER.assignment(literal);
            let literal = if boolean { *var } else { -*var };

            literal
        }).collect::<Vec<_>>();

        for literal in from_solution {
            SOLVER.assume(literal);
        }
    }
}
