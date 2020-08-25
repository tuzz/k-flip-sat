use crate::*;

pub fn counter(bits: &[i32]) -> Vec<i32> {
    let max_bits = log2_ceil(bits.len());

    let sums = bits.chunks(2).map(|pair| {
        let (sum, carry) = half_adder(pair[0], pair[1]);
        vec![sum, carry]
    }).collect::<Vec<_>>();

    recursive_adder(&sums, max_bits)
}

// Implement the parallel-counter encoding described in this paper:
// https://arxiv.org/pdf/2005.06274.pdf
// TODO: This currently uses propagation complete adders but I'm not sure it should.
pub fn recursive_adder(numbers: &[Vec<i32>], max_bits: usize) -> Vec<i32> {
    match numbers.len() {
        1 => numbers[0].clone(),
        2 => ripple_carry_adder(&numbers[0], &numbers[1], max_bits),
        n => {
            let half = round_up_to_power_of_2(n / 2);

            let left = recursive_adder(&numbers[0..half], max_bits);
            let right = recursive_adder(&numbers[half..], max_bits);

            ripple_carry_adder(&left, &right, max_bits)
        }
    }
}

pub fn ripple_carry_adder(a: &[i32], b: &[i32], max_bits: usize) -> Vec<i32> {
    let a = &pad_with_zeros(a, b.len());
    let b = &pad_with_zeros(b, a.len());

    let mut out = vec![];
    let mut c_out = SOLVER.false_literal();

    for (a, b) in a.iter().zip(b) {
        let (sum, carry) = full_adder(*a, *b, c_out);

        out.push(sum);
        c_out = carry;
    }

    if out.len() < max_bits {
        out.push(c_out);
    } else {
        SOLVER.add_clause(&[-c_out]);
    }

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

fn round_up_to_power_of_2(n: usize) -> usize {
    2_usize.pow(log2_ceil(n) as u32)
}

fn log2_ceil(n: usize) -> usize {
    for i in (0..) {
        if 2_usize.pow(i) >= n {
            return i as usize;
        }
    }

    unreachable!();
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
        let max = usize::MAX;

        let out = ripple_carry_adder(&[t, t, f, f], &[t, f, f, t], max); // 3 + 9
        assert_assigned(&out, &[false, false, true, true, false]); // 12

        let out = ripple_carry_adder(&[f, t, f, t], &[f, t, t, f], max); // 10 + 6
        assert_assigned(&out, &[false, false, false, false, true]); // 16
    }

    #[test]
    fn it_can_enforce_the_maximum_number_of_bits_returned() {
        let t = SOLVER.true_literal();
        let f = SOLVER.false_literal();

        let out = ripple_carry_adder(&[t, t, f, f], &[t, f, f, t], 4); // 3 + 9
        assert_assigned(&out, &[false, false, true, true]); // 12
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
        ], usize::MAX);

        assert_assigned(&out, &[false, false, true, true, true, false]); // 28
    }

    #[test]
    fn it_counts_the_number_of_true_bits() {
        let t = SOLVER.true_literal();
        let f = SOLVER.false_literal();

        let out = counter(&[f, f, f, f, f, f, f, f, f, f]);
        assert_assigned(&out, &[false, false, false, false]); // 0

        let out = counter(&[t, f, t, f, t, t, f, t, f, f]);
        assert_assigned(&out, &[true, false, true, false]); // 5

        let out = counter(&[t, t, t, t, t, t, t, t, t, t]);
        assert_assigned(&out, &[false, true, false, true]); // 10
    }
}
