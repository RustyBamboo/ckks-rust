use crate::ntt::Ntt;
use crate::utils::{generate_primes, invmod};

use num_bigint::BigInt;
use num_traits::{One, Zero};

use rayon::prelude::*;

///
/// Chinese remainder theorem
///

#[derive(Debug)]
pub struct Crt {
    primes: Vec<u64>,
    pub modulus: BigInt,
    pub ntts: Vec<Ntt>,
    crt_vals: Vec<BigInt>,
    crt_vals_inv: Vec<BigInt>,
}

impl Crt {
    pub fn new(num_primes: u64, prime_size: u64, poly_degree: u64) -> Self {
        let primes = generate_primes(num_primes, prime_size, 2 * poly_degree);

        let ntts = primes
            .par_iter()
            .map(|p| Ntt::new(poly_degree, *p))
            .collect();

        let mut modulus: BigInt = One::one();
        for p in &primes {
            modulus = modulus * p;
        }

        let crt_vals: Vec<BigInt> = primes.par_iter().map(|p| &modulus / p).collect();

        let crt_vals_inv: Vec<BigInt> = (0..num_primes as usize)
            .into_par_iter()
            .map(|i| invmod(&crt_vals[i], primes[i]))
            .collect();

        Crt {
            primes,
            modulus,
            ntts,
            crt_vals,
            crt_vals_inv,
        }
    }

    ///
    /// Take value X and return a_i mod m_i
    ///
    pub fn crt(&self, value: i128) -> Vec<i128> {
        self.primes
            .iter()
            .map(|p| value.rem_euclid(*p as i128))
            .collect()
    }

    ///
    /// Take an array of a_i (mod m_i) to get value X (mod m_0 * m_1 ...)
    ///
    pub fn reconstruct(&self, values: Vec<i128>) -> BigInt {
        assert_eq!(values.len(), self.primes.len());

        let mut reg = BigInt::zero();

        for i in 0..values.len() {
            let i_val = (values[i] * &self.crt_vals_inv[i]) % self.primes[i];
            let i_val = (i_val * &self.crt_vals[i]) % &self.modulus;
            reg += i_val;
            reg = reg % &self.modulus;
        }
        reg
    }
}
