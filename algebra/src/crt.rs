use crate::ntt::Ntt;
use crate::utils::{generate_primes, invmod};

///
/// Chinese remainder theorem
///

pub struct Crt {
    poly_degree: u64,
    primes: Vec<u64>,
    modulus: u64,
    ntts: Vec<Ntt>,
    crt_vals: Vec<u64>,
    crt_vals_inv: Vec<u64>,
}

impl Crt {
    fn new(num_primes: u64, prime_size: u64, poly_degree: u64) -> Self {
        let primes = generate_primes(num_primes, prime_size, 2 * poly_degree);

        let ntts = primes.iter().map(|p| Ntt::new(poly_degree, *p)).collect();

        let mut modulus = 1;
        for p in &primes {
            modulus *= p;
        }

        let crt_vals: Vec<u64> = primes.iter().map(|p| modulus / p).collect();

        let crt_vals_inv = (0..num_primes as usize)
            .map(|i| invmod(crt_vals[i], primes[i]))
            .collect();

        Crt {
            poly_degree,
            primes,
            modulus,
            ntts,
            crt_vals,
            crt_vals_inv,
        }
    }

    fn crt(&self, value: i128) -> Vec<i128> {
        self.primes
            .iter()
            .map(|p| value.rem_euclid(*p as i128))
            .collect()
    }

    fn reconstruct(&self, values: Vec<i128>) -> i128 {
        assert_eq!(values.len(), self.primes.len());

        let mut reg = 0;

        for i in 0..values.len() {
            let i_val =
                (values[i] * self.crt_vals_inv[i] as i128).rem_euclid(self.primes[i] as i128);
            let i_val = (i_val * self.crt_vals[i] as i128).rem_euclid(self.modulus as i128);

            reg += i_val;
            reg = reg.rem_euclid(self.modulus as i128)
        }
        reg
    }
}
