use dimacs::*;

#[derive(Debug)]
pub struct Formula {
    num_vars: u64,
    clauses: Vec<Vec<i64>>,
}

impl Formula {
    pub fn parse(dimacs: &str) -> Self {
        let instance = parse_dimacs(dimacs).unwrap();

        if let Instance::Cnf { num_vars, clauses } = instance {
            Self::parse_cnf(num_vars, &*clauses)
        } else {
            panic!(".sat files are not supported")
        }
    }

    pub fn parse_cnf(num_vars: u64, clauses: &[Clause]) -> Self {
        let vec = clauses.iter().map(|c| {
            c.lits().iter().map(|literal| {
                let variable = literal.var().to_u64() as i64;

                match literal.sign() {
                    Sign::Pos => variable,
                    Sign::Neg => -variable,
                }
            }).collect()
        }).collect();

        Self { num_vars, clauses: vec }
    }
}
