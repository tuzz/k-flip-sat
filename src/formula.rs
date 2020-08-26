use dimacs::{Instance, Sign, Clause, parse_dimacs};

#[derive(Debug)]
pub struct Formula {
    pub num_vars: u32,
    pub clauses: Vec<Vec<i32>>,
}

impl Formula {
    pub fn parse(dimacs: &str) -> Self {
        let instance = parse_dimacs(dimacs).unwrap();

        if let Instance::Cnf { num_vars, clauses } = instance {
            Self::parse_cnf(num_vars as u32, &*clauses)
        } else {
            panic!(".sat files are not supported")
        }
    }

    pub fn parse_cnf(num_vars: u32, clauses: &[Clause]) -> Self {
        let vec = clauses.iter().map(|c| {
            c.lits().iter().map(|literal| {
                let variable = literal.var().to_u64() as i32;

                match literal.sign() {
                    Sign::Pos => variable,
                    Sign::Neg => -variable,
                }
            }).collect()
        }).collect();

        Self { num_vars, clauses: vec }
    }
}
