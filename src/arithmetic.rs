use crate::*;

pub fn ripple_carry_adder(a: &[i32], b: &[i32]) -> (Vec<i32>, i32) {
    let mut out = vec![];
    let mut c_out = SOLVER.false_literal();

    for (a, b) in a.iter().zip(b) {
        let (sum, carry) = full_adder(*a, *b, c_out);

        out.push(sum);
        c_out = carry;
    }

    (out, c_out)
}

// Implement the propagation complete full-adder encoding from this paper:
// http://www.picat-lang.org/papers/cp17.pdf
pub fn full_adder(a: i32, b: i32, c_in: i32) -> (i32, i32) {
    let out = SOLVER.new_literal();
    let c_out = SOLVER.new_literal();

    SOLVER.add_clause(&[a, -b, c_in, out]);
    SOLVER.add_clause(&[a, b, -c_in, out]);
    SOLVER.add_clause(&[-a, -b, c_in, -out]);
    SOLVER.add_clause(&[-a, b, -c_in, -out]);
    SOLVER.add_clause(&[-a, c_out, out]);
    SOLVER.add_clause(&[a, -c_out, -out]);
    SOLVER.add_clause(&[-b, -c_in, c_out]);
    SOLVER.add_clause(&[b, c_in, -c_out]);
    SOLVER.add_clause(&[-a, -b, -c_in, out]);
    SOLVER.add_clause(&[a, b, c_in, -out]);

    (out, c_out)
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_assigned(literals: &[i32], expected: &[bool]) {
        SOLVER.run();

        for (a, b) in literals.iter().zip(expected) {
            assert_eq!(SOLVER.assignment(*a), *b);
        }
    }

    #[test]
    fn it_adds_two_bits() {
        let t = SOLVER.true_literal();
        let f = SOLVER.false_literal();

        let (out, c_out) = full_adder(f, f, f);
        assert_assigned(&[out, c_out], &[false, false]);

        let (out, c_out) = full_adder(f, f, t);
        assert_assigned(&[out, c_out], &[true, false]);

        let (out, c_out) = full_adder(f, t, f);
        assert_assigned(&[out, c_out], &[true, false]);

        let (out, c_out) = full_adder(f, t, t);
        assert_assigned(&[out, c_out], &[false, true]);

        let (out, c_out) = full_adder(t, f, f);
        assert_assigned(&[out, c_out], &[true, false]);

        let (out, c_out) = full_adder(t, f, t);
        assert_assigned(&[out, c_out], &[false, true]);

        let (out, c_out) = full_adder(t, t, f);
        assert_assigned(&[out, c_out], &[false, true]);

        let (out, c_out) = full_adder(t, t, t);
        assert_assigned(&[out, c_out], &[true, true]);
    }

    #[test]
    fn it_adds_n_bits() {
        let t = SOLVER.true_literal();
        let f = SOLVER.false_literal();

        let (out, c_out) = ripple_carry_adder(&[t, t, f, f], &[t, f, f, t]); // 3 + 9
        assert_assigned(&out, &[false, false, true, true]); // 12
        assert_assigned(&[c_out], &[false]);

        let (out, c_out) = ripple_carry_adder(&[f, t, f, t], &[f, t, t, f]); // 10 + 6
        assert_assigned(&out, &[false, false, false, false]); // 0
        assert_assigned(&[c_out], &[true]); // overflowed
    }
}
