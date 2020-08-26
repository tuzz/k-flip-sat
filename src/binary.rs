use crate::*;

pub struct Binary;

impl Binary {
    pub fn of_length(n: usize) -> Vec<i32> {
        (0..n).map(|_| SOLVER.new_literal()).collect::<Vec<_>>()
    }

    pub fn decode(bits: &[i32]) -> u32 {
        let mut decimal = 0;

        for bit in bits.iter().rev() {
            decimal *= 2;

            if SOLVER.assignment(*bit) {
                decimal += 1;
            }
        }

        decimal
    }

    pub fn assign(bits: &[i32], value: u32) {
        assign_with(bits, value, |bit| SOLVER.add_clause(&[bit]));
    }

    pub fn assume(bits: &[i32], value: u32) {
        assign_with(bits, value, |bit| SOLVER.assume(bit));
    }
}

fn assign_with<F: Fn(i32)>(bits: &[i32], value: u32, f: F) {
    let string = format!("{:0width$b}", value, width = bits.len());

    assert_eq!(string.len(), bits.len());

    for (character, bit) in string.chars().rev().zip(bits) {
        match character { '0' => f(-*bit), '1' => f(*bit), _ => panic!() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_decode_a_binary_number_from_assigned_literals() {
        let t = SOLVER.true_literal();
        let f = SOLVER.false_literal();

        SOLVER.run();

        assert_eq!(Binary::decode(&[f, f, f]), 0);
        assert_eq!(Binary::decode(&[t, f, f]), 1);
        assert_eq!(Binary::decode(&[f, t, f]), 2);
        assert_eq!(Binary::decode(&[t, t, f]), 3);
        assert_eq!(Binary::decode(&[f, f, t]), 4);
        assert_eq!(Binary::decode(&[t, f, t]), 5);
        assert_eq!(Binary::decode(&[f, t, t]), 6);
        assert_eq!(Binary::decode(&[t, t, t]), 7);
    }

    #[test]
    fn it_can_encode_a_decimal_number_to_constrained_literals() {
        let b0 = SOLVER.new_literal();
        let b1 = SOLVER.new_literal();
        let b2 = SOLVER.new_literal();

        Binary::assign(&[b0, b1, b2], 6);

        SOLVER.run();

        assert_eq!(SOLVER.assignment(b0), false);
        assert_eq!(SOLVER.assignment(b1), true);
        assert_eq!(SOLVER.assignment(b2), true);
    }
}
