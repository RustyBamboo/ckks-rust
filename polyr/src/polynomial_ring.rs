use itertools::{iproduct, EitherOrBoth::*, Itertools};
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

///
/// Takes a number and maps it into the space (q/2, q/2] for some number q.
///
pub fn mod_ring(num: i32, q: i32) -> i32 {
    let num = num.rem_euclid(q);

    return if num > q / 2 { num - q } else { num };
}

pub trait Modulo {
    fn mod_ring(self, q: Self) -> Self;
}
impl Modulo for i32 {
    fn mod_ring(self, q: Self) -> Self {
        return mod_ring(self, q);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PolynomialRing<T> {
    pub coef: Vec<T>,
    pub poly_degree: usize,
}

#[macro_export]
macro_rules! polynomial_ring {
    ($p: expr, ($($x:expr),*)) => {
        PolynomialRing::new($p, vec![$($x), *])
    }
}

impl PolynomialRing<i32> {
    pub fn new(poly_degree: usize, coef: Vec<i32>) -> Self {
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
        let n = self.poly_degree;
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

    pub fn rand_binary(poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let u = Uniform::from(0..2);
        let coef = (0..size).map(|_| u.sample(&mut rng)).collect();
        Self { coef, poly_degree }.clean()
    }

    pub fn rand_uniform(ring: i32, poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let u = Uniform::from(0..ring);
        let coef = (0..size).map(|_| u.sample(&mut rng)).collect();
        Self { coef, poly_degree }.clean()
    }

    pub fn rand_normal(poly_degree: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let n = Normal::new(0., 2.).expect("Error creating distribution");
        let coef = (0..size).map(|_| n.sample(&mut rng) as i32).collect();
        Self { coef, poly_degree }.clean()
    }
}

impl std::ops::Rem<i32> for PolynomialRing<i32> {
    type Output = Self;
    fn rem(self, other: i32) -> Self::Output {
        &self % other
    }
}

impl std::ops::Rem<i32> for &PolynomialRing<i32> {
    type Output = PolynomialRing<i32>;
    fn rem(self, other: i32) -> Self::Output {
        let coef = self.coef.iter().map(|x| x.mod_ring(other)).collect();
        PolynomialRing::new(self.poly_degree, coef)
    }
}

impl std::ops::Add for PolynomialRing<i32> {
    type Output = Self;
    fn add(self, other: PolynomialRing<i32>) -> Self {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => a + b,
                Left(a) => *a,
                Right(a) => *a,
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Add<&PolynomialRing<i32>> for &PolynomialRing<i32> {
    type Output = PolynomialRing<i32>;
    fn add(self, other: &PolynomialRing<i32>) -> PolynomialRing<i32> {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => a + b,
                Left(a) => *a,
                Right(a) => *a,
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Add<&PolynomialRing<i32>> for PolynomialRing<i32> {
    type Output = PolynomialRing<i32>;
    fn add(self, other: &PolynomialRing<i32>) -> PolynomialRing<i32> {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => a + b,
                Left(a) => *a,
                Right(a) => *a,
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Sub for PolynomialRing<i32> {
    type Output = Self;
    fn sub(self, other: PolynomialRing<i32>) -> Self {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => (b - a),
                Left(a) => *a,
                Right(a) => *a,
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Sub<&PolynomialRing<i32>> for &PolynomialRing<i32> {
    type Output = PolynomialRing<i32>;
    fn sub(self, other: &PolynomialRing<i32>) -> PolynomialRing<i32> {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => (b - a),
                Left(a) => *a,
                Right(a) => *a,
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

impl std::ops::Sub<&PolynomialRing<i32>> for PolynomialRing<i32> {
    type Output = PolynomialRing<i32>;
    fn sub(self, other: &PolynomialRing<i32>) -> PolynomialRing<i32> {
        let out = other
            .coef
            .iter()
            .zip_longest(self.coef.iter())
            .map(|x| match x {
                Both(a, b) => (b - a),
                Left(a) => *a,
                Right(a) => *a,
            })
            .collect();
        PolynomialRing::new(self.poly_degree, out).mod_cyc()
    }
}

// TODO: Use FFT to do this in O(nlogn) instead of O(n^2)
//
// See; https://math.stackexchange.com/questions/764727/concrete-fft-polynomial-multiplication-example
impl std::ops::Mul for PolynomialRing<i32> {
    type Output = Self;
    fn mul(self, other: PolynomialRing<i32>) -> Self {
        let mut res = vec![0; other.len() + self.len() - 1];
        for ((i1, v1), (i2, v2)) in
            iproduct!(other.coef.iter().enumerate(), self.coef.iter().enumerate())
        {
            res[i1 + i2] += v1 * v2;
        }
        PolynomialRing::new(self.poly_degree, res).mod_cyc()
    }
}

impl std::ops::Mul<&PolynomialRing<i32>> for &PolynomialRing<i32> {
    type Output = PolynomialRing<i32>;
    fn mul(self, other: &PolynomialRing<i32>) -> PolynomialRing<i32> {
        let mut res = vec![0; other.len() + self.len() - 1];
        for ((i1, v1), (i2, v2)) in
            iproduct!(other.coef.iter().enumerate(), self.coef.iter().enumerate())
        {
            res[i1 + i2] += v1 * v2;
        }
        PolynomialRing::new(self.poly_degree, res).mod_cyc()
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
