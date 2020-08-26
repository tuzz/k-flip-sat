use crate::*;

lazy_static! {
    pub static ref SOLVER: Solver = Solver::new();
}

pub struct Solver {
    inner: Rc<RefCell<Inner>>,
}

pub struct Inner {
    pointer: *mut std::ffi::c_void,
    literals: u32,
    clauses: u32,
    true_literal: i32,
    false_literal: i32,
}

const LOG_DIMACS: bool = false;

impl Solver {
    pub fn new() -> Self {
        let inner = Inner {
            pointer: unsafe { ipasir_init() },
            literals: 0,
            clauses: 0,
            true_literal: 0,
            false_literal: 0,
        };

        Self { inner: Rc::new(RefCell::new(inner)) }
    }

    pub fn sync_with_formula(&self, formula: &Formula) {
        let mut inner = self.inner.borrow_mut();

        inner.literals += formula.num_vars;
        inner.clauses += formula.clauses.len() as u32;
    }

    pub fn set_ground_literals(&self) {
        let ground_truth = self.new_literal();
        let ground_false = self.new_literal();

        self.add_clause(&[ground_truth]);
        self.add_clause(&[-ground_false]);

        let mut inner = self.inner.borrow_mut();

        inner.true_literal = ground_truth;
        inner.false_literal = ground_false;
    }

    pub fn new_literal(&self) -> i32 {
        let mut inner = self.inner.borrow_mut();

        inner.literals += 1;
        inner.literals as i32
    }

    pub fn true_literal(&self) -> i32 {
        self.true_literal
    }

    pub fn false_literal(&self) -> i32 {
        self.false_literal
    }

    pub fn add(&self, literal: i32) {
        unsafe { ipasir_add(self.pointer, literal); }

        if literal == 0 {
            self.inner.borrow_mut().clauses += 1;
        }

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
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { ipasir_release(self.pointer); }
    }
}

impl Deref for Solver {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.try_borrow_unguarded().unwrap() }
    }
}

unsafe impl Send for Solver { }
unsafe impl Sync for Solver { }
