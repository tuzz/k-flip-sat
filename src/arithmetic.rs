use crate::*;

pub fn counter(bits: &[i32]) -> Vec<i32> {
    let sums = bits.chunks(2).map(|pair| {
        let (sum, carry) = half_adder(pair[0], pair[1]);
        vec![sum, carry]
    }).collect::<Vec<_>>();

    recursive_adder(&sums)
}

pub fn recursive_adder(numbers: &[Vec<i32>]) -> Vec<i32> {
    match numbers.len() {
        1 => numbers[0].clone(),
        2 => ripple_carry_adder(&numbers[0], &numbers[1]),
        n => {
            let half = round_up_to_power_of_2(n as f32 / 2.);

            let left = recursive_adder(&numbers[0..half]);
            let right = recursive_adder(&numbers[half..]);

            let left = pad_with_zeros(&left, right.len());
            let right = pad_with_zeros(&right, left.len());

            ripple_carry_adder(&left, &right)
        }
    }
}

// TODO: max bits
pub fn ripple_carry_adder(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut out = vec![];
    let mut c_out = SOLVER.false_literal();

    for (a, b) in a.iter().zip(b) {
        let (sum, carry) = full_adder(*a, *b, c_out);

        out.push(sum);
        c_out = carry;
    }

    out.push(c_out);
    out
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

// The half add is the same as above but sets c_in to 0 and simplifies.
// This has 6 clauses, not 7 as is stated in the paper referenced above.
pub fn half_adder(a: i32, b: i32) -> (i32, i32) {
    let out = SOLVER.new_literal();
    let c_out = SOLVER.new_literal();

    SOLVER.add_clause(&[a, -b, out]);
    SOLVER.add_clause(&[-a, -b, -out]);
    SOLVER.add_clause(&[-a, c_out, out]);
    SOLVER.add_clause(&[a, -c_out, -out]);
    SOLVER.add_clause(&[b, -c_out]);
    SOLVER.add_clause(&[a, b, -out]);

    (out, c_out)
}

fn round_up_to_power_of_2(n: f32) -> usize {
    let mut power = 1.;
    while power < n { power *= 2.; }
    power as usize
}

fn pad_with_zeros(bits: &[i32], len: usize) -> Vec<i32> {
    let padding = len.saturating_sub(bits.len());
    let zeros = (0..padding).map(|_| SOLVER.false_literal());

    bits.iter().cloned().chain(zeros).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_assigned(literals: &[i32], expected: &[bool]) {
        assert_eq!(literals.len(), expected.len());

        SOLVER.run();

        for (a, b) in literals.iter().zip(expected) {
            assert_eq!(SOLVER.assignment(*a), *b);
        }
    }

    #[test]
    fn it_adds_two_bits() {
        let t = SOLVER.true_literal();
        let f = SOLVER.false_literal();

        let (out, c_out) = half_adder(f, f);
        assert_assigned(&[out, c_out], &[false, false]);

        let (out, c_out) = half_adder(f, t);
        assert_assigned(&[out, c_out], &[true, false]);

        let (out, c_out) = half_adder(t, f);
        assert_assigned(&[out, c_out], &[true, false]);

        let (out, c_out) = half_adder(t, t);
        assert_assigned(&[out, c_out], &[false, true]);
    }

    #[test]
    fn it_adds_three_bits() {
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

        let out = ripple_carry_adder(&[t, t, f, f], &[t, f, f, t]); // 3 + 9
        assert_assigned(&out, &[false, false, true, true, false]); // 12

        let out = ripple_carry_adder(&[f, t, f, t], &[f, t, t, f]); // 10 + 6
        assert_assigned(&out, &[false, false, false, false, true]); // 16
    }

    #[test]
    fn it_adds_n_numbers() {
        let t = SOLVER.true_literal();
        let f = SOLVER.false_literal();

        let out = recursive_adder(&[
            vec![t, t, f, f], // 3
            vec![t, f, f, t], // 9
            vec![f, t, f, t], // 10
            vec![f, t, t, f], // 6
        ]);

        assert_assigned(&out, &[false, false, true, true, true, false]); // 28
    }

    #[test]
    fn it_counts_the_number_of_true_bits() {
        let t = SOLVER.true_literal();
        let f = SOLVER.false_literal();

        let out = counter(&[f, f, f, f, f, f, f, f, f, f]);
        assert_assigned(&out, &[false, false, false, false, false]); // 0

        let out = counter(&[t, f, t, f, t, t, f, t, f, f]);
        assert_assigned(&out, &[true, false, true, false, false]); // 5

        let out = counter(&[t, t, t, t, t, t, t, t, t, t]);
        assert_assigned(&out, &[false, true, false, true, false]); // 10
    }
}
