use itertools::{iproduct, EitherOrBoth::*, Itertools};

#[derive(PartialEq, Debug)]
pub struct Polynomial<T> {
    coef: Vec<T>,
}

#[macro_export]
macro_rules! polynomial {
    ($($x:expr),*) => {
        Polynomial::new(vec![$($x), *])
    }
}

impl Polynomial<i32> {
    pub fn new(coef: Vec<i32>) -> Self {
        Self { coef }
    }

    pub fn len(&self) -> usize {
        self.coef.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Take the function modulo of self with (X^n + 1)
    pub fn mod_cyc(&mut self, n: usize) {
        if self.len() >= n {
            let diff = self.len() - n;
            for i in n..n + diff {
                self.coef[i - n] -= self.coef[i];
                self.coef[i] = 0;
            }
            self.coef.drain(n..);
        }
    }

    pub fn rem_euclid(&mut self, q: i32) {
        self.coef = self.coef.iter().map(|x| x.rem_euclid(q)).collect()
    }
}

impl std::ops::Add for Polynomial<i32> {
    type Output = Self;
    fn add(self, other: Polynomial<i32>) -> Self {
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
        Polynomial::new(out)
    }
}

// TODO: Use FFT to do this in O(nlogn) instead of O(n^2)
//
// See; https://math.stackexchange.com/questions/764727/concrete-fft-polynomial-multiplication-example
impl std::ops::Mul for Polynomial<i32> {
    type Output = Self;
    fn mul(self, other: Polynomial<i32>) -> Self {
        let mut res = vec![0; other.len() + self.len() - 1];
        for ((i1, v1), (i2, v2)) in
            iproduct!(other.coef.iter().enumerate(), self.coef.iter().enumerate())
        {
            res[i1 + i2] += v1 * v2;
        }
        Polynomial::new(res)
    }
}

impl std::fmt::Display for Polynomial<i32> {
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
