use itertools::{iproduct, EitherOrBoth::*, Itertools};
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

#[derive(PartialEq, Debug)]
pub struct PolynomialRing<T> {
    coef: Vec<T>,
    ring: T,
    pmod: usize,
}

#[macro_export]
macro_rules! polynomial_ring {
    ($r:expr, $p: expr, ($($x:expr),*)) => {
        PolynomialRing::new($r,$p, vec![$($x), *])
    }
}

impl PolynomialRing<i32> {
    pub fn new(ring: i32, pmod: usize, coef: Vec<i32>) -> Self {
        // Take the mod to make sure elements are in the ring
        let coef = coef.iter().map(|x| x.rem_euclid(ring)).collect();
        Self { coef, ring, pmod }
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
                if *c == 0 {
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
        let n = self.pmod;
        if self.len() >= n {
            let diff = self.len() - n;
            for i in n..n + diff {
                self.coef[i - n] -= self.coef[i];
                self.coef[i] = 0;
            }
            self.coef.drain(n..);
        }
        self
    }

    pub fn check_same_basis(&self, other: &Self) -> bool {
        self.ring == other.ring && self.pmod == other.pmod
    }

    pub fn rand_binary(ring: i32, pmod: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let u = Uniform::from(0..2);
        let coef = (0..size).map(|_| u.sample(&mut rng)).collect();
        Self { coef, ring, pmod }.clean()
    }

    pub fn rand_uniform(ring: i32, pmod: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let u = Uniform::from(0..ring);
        let coef = (0..size).map(|_| u.sample(&mut rng)).collect();
        Self { coef, ring, pmod }.clean()
    }

    pub fn rand_normal(ring: i32, pmod: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let n = Normal::new(0., 2.).expect("Error creating distribution");
        let coef = (0..size).map(|_| n.sample(&mut rng) as i32).collect();
        Self { coef, ring, pmod }.clean()
    }
}

impl std::ops::Add for PolynomialRing<i32> {
    type Output = Self;
    fn add(self, other: PolynomialRing<i32>) -> Self {
        assert!(self.check_same_basis(&other));
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => (a + b).rem_euclid(self.ring),
                Left(a) => *a,
                Right(a) => *a,
            })
            .collect();
        PolynomialRing::new(self.ring, self.pmod, out).mod_cyc()
    }
}

// TODO: Use FFT to do this in O(nlogn) instead of O(n^2)
//
// See; https://math.stackexchange.com/questions/764727/concrete-fft-polynomial-multiplication-example
impl std::ops::Mul for PolynomialRing<i32> {
    type Output = Self;
    fn mul(self, other: PolynomialRing<i32>) -> Self {
        assert!(self.check_same_basis(&other));
        let mut res = vec![0; other.len() + self.len() - 1];
        for ((i1, v1), (i2, v2)) in
            iproduct!(other.coef.iter().enumerate(), self.coef.iter().enumerate())
        {
            res[i1 + i2] += (v1 * v2).rem_euclid(self.ring);
        }
        PolynomialRing::new(self.ring, self.pmod, res).mod_cyc()
    }
}

impl std::fmt::Display for PolynomialRing<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let coef = &self.coef;
        if coef.is_empty() {
            return write!(f, "0");
        }
        let mut is_first = true;
        for (i, c) in coef.iter().enumerate().rev() {
            if *c == 0 {
                continue;
            }
            if is_first {
                is_first = false;
            } else {
                write!(f, "+")?
            }
            if *c == 1 {
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
