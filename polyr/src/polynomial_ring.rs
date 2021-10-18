use itertools::{iproduct, EitherOrBoth::*, Itertools};
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::{One, ToPrimitive, Zero};

use algebra::crt::Crt;
use algebra::ntt::Ntt;

use rayon::prelude::*;

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

#[derive(Debug, Clone)]
pub struct PolynomialRing<'n, T> {
    pub coef: Vec<T>,
    pub poly_degree: usize,
    crt: Option<&'n Crt>,
}

impl<T> PartialEq for PolynomialRing<'_, T>
where
    Vec<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.coef == other.coef && self.poly_degree == other.poly_degree
    }
}

impl<'a> PolynomialRing<'a, BigInt> {
    ///
    /// Create a new polynomial in R/[x^N]
    ///
    /// Where poly_degree is N, and coef are the coefficients
    ///
    pub fn new(poly_degree: usize, coef: Vec<BigInt>) -> Self {
        // Take the mod to make sure elements are in the ring
        Self {
            coef,
            poly_degree,
            crt: None,
        }
    }

    ///
    /// Create a new polynomial but with a context to the Chinese Remainder Theorem
    ///
    pub fn new_with_crt(poly_degree: usize, coef: Vec<BigInt>, crt: &'a Crt) -> Self {
        Self {
            coef,
            poly_degree,
            crt: Some(crt),
        }
    }

    ///
    /// Add a Chinese Remainder Theorem context
    ///
    pub fn add_crt(mut self, crt: &'a Crt) -> Self {
        self.crt = Some(crt);
        self
    }

    ///
    /// Add a Chinese Remainder Theorem context as an Option
    ///
    pub fn add_option_crt(mut self, crt: Option<&'a Crt>) -> Self {
        self.crt = crt;
        self
    }

    pub fn len(&self) -> usize {
        self.coef.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    ///
    /// Remove trailing zeros of polynomial
    ///
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

    ///
    /// Take the function modulo of self with (X^n + 1)
    ///
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

    ///
    /// Create a random polynomial with samples either 0 or 1.
    ///
    pub fn rand_binary(poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let u = Uniform::from(0..2);
        let coef = (0..size)
            .map(|_| u.sample(&mut rng).to_bigint().unwrap())
            .collect();
        Self {
            coef,
            poly_degree,
            crt: None,
        }
    }

    ///
    /// Create a random polynomial from a uniform distribution of [0, q]
    ///
    /// Where q is modulo of the field elements
    ///
    pub fn rand_uniform(ring: &BigInt, poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let coef = (0..size)
            .map(|_| {
                rng.gen_bigint_range(&Zero::zero(), &ring)
                    .to_bigint()
                    .unwrap()
            })
            .collect();
        Self {
            coef,
            poly_degree,
            crt: None,
        }
    }

    ///
    /// Create a random polynomial with samples from a Gaussian distribution in [0, 2]
    ///
    pub fn rand_normal(poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let n = Normal::new(0., 2.).expect("Error creating distribution");
        let coef = (0..size)
            .map(|_| n.sample(&mut rng).to_bigint().unwrap())
            .collect();
        Self {
            coef,
            poly_degree,
            crt: None,
        }
    }
}

impl<'a> std::ops::Rem<&BigInt> for PolynomialRing<'a, BigInt> {
    type Output = Self;
    fn rem(self, other: &BigInt) -> Self::Output {
        &self % other
    }
}

impl<'a> std::ops::Rem<&BigInt> for &PolynomialRing<'a, BigInt> {
    type Output = PolynomialRing<'a, BigInt>;
    fn rem(self, other: &BigInt) -> Self::Output {
        let coef = self.coef.iter().map(|x| x.mod_ring(&other)).collect();
        PolynomialRing::new(self.poly_degree, coef).add_option_crt(self.crt)
    }
}

impl<'a> std::ops::Add for PolynomialRing<'a, BigInt> {
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
        PolynomialRing::new(self.poly_degree, out)
            .mod_cyc()
            .add_option_crt(self.crt)
    }
}

impl<'a> std::ops::Add<&PolynomialRing<'a, BigInt>> for &PolynomialRing<'a, BigInt> {
    type Output = PolynomialRing<'a, BigInt>;
    fn add(self, other: &PolynomialRing<BigInt>) -> Self::Output {
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
        PolynomialRing::new(self.poly_degree, out)
            .mod_cyc()
            .add_option_crt(self.crt)
    }
}

impl<'a> std::ops::Add<&PolynomialRing<'a, BigInt>> for PolynomialRing<'a, BigInt> {
    type Output = PolynomialRing<'a, BigInt>;
    fn add(self, other: &PolynomialRing<BigInt>) -> Self::Output {
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
        PolynomialRing::new(self.poly_degree, out)
            .mod_cyc()
            .add_option_crt(self.crt)
    }
}

impl<'a> std::ops::Sub for PolynomialRing<'a, BigInt> {
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
        PolynomialRing::new(self.poly_degree, out)
            .mod_cyc()
            .add_option_crt(self.crt)
    }
}

impl<'a> std::ops::Sub<&PolynomialRing<'a, BigInt>> for &PolynomialRing<'a, BigInt> {
    type Output = PolynomialRing<'a, BigInt>;
    fn sub(self, other: &PolynomialRing<BigInt>) -> Self::Output {
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
        PolynomialRing::new(self.poly_degree, out)
            .mod_cyc()
            .add_option_crt(self.crt)
    }
}

impl<'a> std::ops::Sub<&PolynomialRing<'a, BigInt>> for PolynomialRing<'a, BigInt> {
    type Output = PolynomialRing<'a, BigInt>;
    fn sub(self, other: &PolynomialRing<BigInt>) -> Self::Output {
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
        PolynomialRing::new(self.poly_degree, out)
            .mod_cyc()
            .add_option_crt(self.crt)
    }
}

// TODO: Use FFT to do this in O(nlogn) instead of O(n^2)
//
// See; https://math.stackexchange.com/questions/764727/concrete-fft-polynomial-multiplication-example
impl<'a> std::ops::Mul<&PolynomialRing<'a, BigInt>> for &PolynomialRing<'a, BigInt> {
    type Output = PolynomialRing<'a, BigInt>;
    fn mul(self, other: &PolynomialRing<BigInt>) -> Self::Output {
        if let Some(crt) = self.crt {
            let a = &self.coef;
            let b = &other.coef;

            let mul_ntt = |ntt: &Ntt| {
                let a = ntt.fft_fwd(&a);
                let b = ntt.fft_fwd(&b);
                let c = (0..self.poly_degree).map(|i| &a[i] * &b[i]).collect();
                let res = ntt
                    .fft_inv(&c)
                    .iter()
                    .map(|x| x.to_bigint().unwrap())
                    .collect();
                return PolynomialRing::new(self.poly_degree, res)
                    .mod_cyc()
                    .add_option_crt(self.crt);
            };

            let prod: Vec<PolynomialRing<_>> = crt.ntts.iter().map(|n| mul_ntt(n)).collect();

            let final_coeffs = (0..self.poly_degree)
                .into_par_iter()
                .map(|i| {
                    let vals = prod.iter().map(|p| p.coef[i].to_i128().unwrap()).collect();
                    crt.reconstruct(vals)
                        .mod_ring(&crt.modulus.to_bigint().unwrap())
                })
                .collect();

            return PolynomialRing::new(self.poly_degree, final_coeffs)
                .mod_cyc()
                .add_option_crt(self.crt);
        }
        let mut res = vec![Zero::zero(); other.len() + self.len() - 1];
        for ((i1, v1), (i2, v2)) in
            iproduct!(other.coef.iter().enumerate(), self.coef.iter().enumerate())
        {
            res[i1 + i2] += v1 * v2;
        }
        PolynomialRing::new(self.poly_degree, res)
            .mod_cyc()
            .add_option_crt(self.crt)
    }
}

impl std::fmt::Display for PolynomialRing<'_, BigInt> {
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
