use itertools::{iproduct, EitherOrBoth::*, Itertools};
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{Zero, One};

///
/// Takes a number and maps it into the space (q/2, q/2] for some number q.
///
pub fn mod_ring(num: &BigInt, q: &BigInt) -> BigInt {
    // This takes the modulus rather than remainder
    let num = ((num % q) + q) % q;

    return if num > q / 2 { num - q } else { num };
}

pub trait Modulo {
    fn mod_ring(&self, q: &Self) -> Self;
}
impl Modulo for BigInt {
    fn mod_ring(&self, q: &Self) -> Self {
        return mod_ring(&self, &q);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PolynomialRing<T> {
    pub coef: Vec<T>,
    pub poly_degree: usize,
}

impl PolynomialRing<BigInt> {
    pub fn new(poly_degree: usize, coef: Vec<BigInt>) -> Self {
        // Take the mod to make sure elements are in the ring
        Self { coef, poly_degree }
    }

    pub fn len(&self) -> usize {
        self.coef.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Remove trailing zeros
    pub fn clean(mut self) -> Self {
        let mut count = 0;
        {
            let coef = self.coef.iter().rev();
            for c in coef {
                if *c == Zero::zero() {
                    count += 1;
                } else {
                    break;
                }
            }
        }
        self.coef.drain(self.coef.len() - count..);
        self
    }

    // Take the function modulo of self with (X^n + 1)
    fn mod_cyc(mut self) -> Self {
        let n = self.poly_degree;
        if self.len() >= n {
            let diff = self.len() - n;
            for i in n..n + diff {
                self.coef[i - n] = self.coef[i - n].clone() - self.coef[i].clone();
                self.coef[i] = Zero::zero();
            }
            self.coef.drain(n..);
        }
        self
    }

    pub fn rand_binary(poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let u = Uniform::from(0..2);
        let coef = (0..size)
            .map(|_| u.sample(&mut rng).to_bigint().unwrap())
            .collect();
        Self { coef, poly_degree }.clean()
    }

    pub fn rand_uniform(ring: &BigInt, poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let coef = (0..size)
            .map(|_| {
                rng.gen_bigint_range(&Zero::zero(), &ring)
                    .to_bigint()
                    .unwrap()
            })
            .collect();
        Self { coef, poly_degree }.clean()
    }

    pub fn rand_normal(poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let n = Normal::new(0., 2.).expect("Error creating distribution");
        let coef = (0..size)
            .map(|_| n.sample(&mut rng).to_bigint().unwrap())
            .collect();
        Self { coef, poly_degree }.clean()
    }
}

impl std::ops::Rem<&BigInt> for PolynomialRing<BigInt> {
    type Output = Self;
    fn rem(self, other: &BigInt) -> Self::Output {
        &self % other
    }
}

impl std::ops::Rem<&BigInt> for &PolynomialRing<BigInt> {
    type Output = PolynomialRing<BigInt>;
    fn rem(self, other: &BigInt) -> Self::Output {
        let coef = self.coef.iter().map(|x| x.mod_ring(&other)).collect();
        PolynomialRing::new(self.poly_degree, coef)
    }
}

impl std::ops::Add for PolynomialRing<BigInt> {
    type Output = Self;
    fn add(self, other: PolynomialRing<BigInt>) -> Self {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => a + b,
                Left(a) => a.clone(),
                Right(a) => a.clone(),
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Add<&PolynomialRing<BigInt>> for &PolynomialRing<BigInt> {
    type Output = PolynomialRing<BigInt>;
    fn add(self, other: &PolynomialRing<BigInt>) -> PolynomialRing<BigInt> {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => a + b,
                Left(a) => a.clone(),
                Right(a) => a.clone(),
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Add<&PolynomialRing<BigInt>> for PolynomialRing<BigInt> {
    type Output = PolynomialRing<BigInt>;
    fn add(self, other: &PolynomialRing<BigInt>) -> PolynomialRing<BigInt> {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => a + b,
                Left(a) => a.clone(),
                Right(a) => a.clone(),
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Sub for PolynomialRing<BigInt> {
    type Output = Self;
    fn sub(self, other: PolynomialRing<BigInt>) -> Self {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => (b - a),
                Left(a) => a.clone(),
                Right(a) => a.clone(),
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Sub<&PolynomialRing<BigInt>> for &PolynomialRing<BigInt> {
    type Output = PolynomialRing<BigInt>;
    fn sub(self, other: &PolynomialRing<BigInt>) -> PolynomialRing<BigInt> {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => (b - a),
                Left(a) => a.clone(),
                Right(a) => a.clone(),
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Sub<&PolynomialRing<BigInt>> for PolynomialRing<BigInt> {
    type Output = PolynomialRing<BigInt>;
    fn sub(self, other: &PolynomialRing<BigInt>) -> PolynomialRing<BigInt> {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => (b - a),
                Left(a) => a.clone(),
                Right(a) => a.clone(),
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

// TODO: Use FFT to do this in O(nlogn) instead of O(n^2)
//
// See; https://math.stackexchange.com/questions/764727/concrete-fft-polynomial-multiplication-example
impl std::ops::Mul for PolynomialRing<BigInt> {
    type Output = Self;
    fn mul(self, other: PolynomialRing<BigInt>) -> Self {
        let mut res = vec![Zero::zero(); other.len() + self.len() - 1];
        for ((i1, v1), (i2, v2)) in
            iproduct!(other.coef.iter().enumerate(), self.coef.iter().enumerate())
        {
            res[i1 + i2] += v1 * v2;
        }
        PolynomialRing::new(self.poly_degree, res).mod_cyc()
    }
}

impl std::ops::Mul<&PolynomialRing<BigInt>> for &PolynomialRing<BigInt> {
    type Output = PolynomialRing<BigInt>;
    fn mul(self, other: &PolynomialRing<BigInt>) -> PolynomialRing<BigInt> {
        let mut res = vec![Zero::zero(); other.len() + self.len() - 1];
        for ((i1, v1), (i2, v2)) in
            iproduct!(other.coef.iter().enumerate(), self.coef.iter().enumerate())
        {
            res[i1 + i2] += v1 * v2;
        }
        PolynomialRing::new(self.poly_degree, res).mod_cyc()
    }
}

impl std::fmt::Display for PolynomialRing<BigInt> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let coef = &self.coef;
        if coef.is_empty() {
            return write!(f, "0");
        }
        let mut is_first = true;
        for (i, c) in coef.iter().enumerate().rev() {
            if *c == Zero::zero() {
                continue;
            }
            if is_first {
                is_first = false;
            } else {
                write!(f, "+")?
            }
            if *c == One::one() {
                match i {
                    0 => write!(f, "1")?,
                    1 => write!(f, "x")?,
                    _ => write!(f, "x^{}", i)?,
                }
            } else {
                match i {
                    0 => write!(f, "{}", c)?,
                    1 => write!(f, "{}*x", c)?,
                    _ => write!(f, "{}*x^{}", c, i)?,
                }
            }
        }
        Ok(())
    }
}
