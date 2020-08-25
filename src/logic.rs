use crate::*;

// The Tseitin transformation for an AND gate.
pub fn and(a: i32, b: i32) -> i32 {
    let out = SOLVER.new_literal();

    SOLVER.add_clause(&[-a, -b, out]);
    SOLVER.add_clause(&[a, -out]);
    SOLVER.add_clause(&[b, -out]);

    out
}

// The Tseitin transformation for an OR gate.
pub fn or(a: i32, b: i32) -> i32 {
    let out = SOLVER.new_literal();

    SOLVER.add_clause(&[a, b, -out]);
    SOLVER.add_clause(&[-a, out]);
    SOLVER.add_clause(&[-b, out]);

    out
}

// The Tseitin transformation for an XNOR gate.
pub fn equal(a: i32, b: i32) -> i32 {
    let out = SOLVER.new_literal();

    SOLVER.add_clause(&[-a, -b, out]);
    SOLVER.add_clause(&[a, b, out]);
    SOLVER.add_clause(&[a, -b, -out]);
    SOLVER.add_clause(&[-a, b, -out]);

    out
}
