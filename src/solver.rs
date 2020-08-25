use crate::*;

lazy_static! {
    pub static ref SOLVER: Solver = Solver::new();
}

pub struct Solver {
    pointer: *mut std::ffi::c_void,
    literals: RefCell<u32>,
    clauses: RefCell<u32>,
    true_literal: i32,
    false_literal: i32,
}

const LOG_DIMACS: bool = false;

impl Solver {
    pub fn new() -> Self {
        let mut solver = Self {
            pointer: unsafe { ipasir_init() },
            literals: RefCell::new(0),
            clauses: RefCell::new(0),
            true_literal: 0,
            false_literal: 0,
        };

        solver.true_literal = solver.new_literal();
        solver.false_literal = solver.new_literal();

        solver.add(solver.true_literal);
        solver.add(0);

        solver.add(-solver.false_literal);
        solver.add(0);

        solver
    }

    pub fn new_literal(&self) -> i32 {
        *self.literals.borrow_mut() += 1;
        *self.literals.borrow() as i32
    }

    pub fn true_literal(&self) -> i32 {
        self.true_literal
    }

    pub fn false_literal(&self) -> i32 {
        self.false_literal
    }

    pub fn add(&self, literal: i32) {
        unsafe { ipasir_add(self.pointer, literal); }
        if literal == 0 { *self.clauses.borrow_mut() += 1; }

        if LOG_DIMACS {
            print!("{} ", literal);
            if literal == 0 { println!(); }
        }
    }

    pub fn add_clause(&self, literals: &[i32]) {
        for literal in literals {
            self.add(*literal);
        }

        self.add(0);
    }

    pub fn assume(&self, literal: i32) {
        unsafe { ipasir_assume(self.pointer, literal); }
    }

    pub fn run(&self) -> bool {
        unsafe { ipasir_solve(self.pointer) == 10 }
    }

    pub fn assignment(&self, literal: i32) -> bool {
        unsafe { ipasir_val(self.pointer, literal) == literal }
    }

    pub fn literals(&self) -> u32 {
        *self.literals.borrow()
    }

    pub fn clauses(&self) -> u32 {
        *self.clauses.borrow()
    }
}

unsafe impl Send for Solver { }
unsafe impl Sync for Solver { }
