use std::ops::{Mul, MulAssign};

use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct BigRational {
    pub p: BigUint,
    pub q: BigUint,
}

impl BigRational {
    pub fn new(p: BigUint, q: BigUint) -> Self {
        Self { p, q }
    }
}

impl From<BigUint> for BigRational {
    fn from(value: BigUint) -> BigRational {
        BigRational::new(value, BigUint::from(1u8))
    }
}

impl From<BigRational> for BigUint {
    fn from(value: BigRational) -> BigUint {
        value.p / value.q
    }
}

impl Mul<BigRational> for BigRational {
    type Output = BigRational;

    fn mul(self, rhs: BigRational) -> Self::Output {
        BigRational::new(self.p * rhs.p, self.q * rhs.q)
    }
}

impl MulAssign<BigRational> for BigRational {
    fn mul_assign(&mut self, rhs: BigRational) {
        let res = self.clone() * rhs;
        self.p = res.p;
        self.q = res.q;
    }
}
