use crate::utils::{bit_reverse_vec, invmod, root_of_unity, RemEuclid};
use num_bigint::BigInt;
use num_traits::One;

///
/// Parameters for Number/Fermat Theoretic Transform
///
#[derive(Debug)]
pub struct Ntt {
    coeff_modulus: u64,
    degree: u64,
    pub roots_of_unity: Vec<BigInt>,
    pub roots_of_unity_inv: Vec<BigInt>,
}

impl Ntt {
    pub fn new(degree: u64, coeff_modulus: u64) -> Self {
        assert!(
            degree & (degree - 1) == 0,
            "Polynomial degree needs to be a power of 2"
        );

        let root_of_unity = root_of_unity(2 * degree as i128, coeff_modulus as i128);
        let root_of_unity_inv = invmod(root_of_unity, coeff_modulus as i128);

        let mut roots_of_unity = vec![One::one(); degree as usize];
        let mut roots_of_unity_inv = vec![One::one(); degree as usize];

        for i in 1..degree as usize {
            roots_of_unity[i] = (&roots_of_unity[i - 1] * &root_of_unity) % coeff_modulus;
        }

        for i in 1..degree as usize {
            roots_of_unity_inv[i] =
                (&roots_of_unity_inv[i - 1] * &root_of_unity_inv) % coeff_modulus;
        }

        Ntt {
            coeff_modulus,
            degree,
            roots_of_unity,
            roots_of_unity_inv,
        }
    }

    pub fn ntt(&self, coeffs: &Vec<BigInt>, rou: &Vec<BigInt>) -> Vec<BigInt> {
        assert!(
            coeffs.len() == coeffs.len(),
            "Length of roots of unity is too small"
        );

        let num_coeffs = coeffs.len();
        let mut result = bit_reverse_vec(coeffs);

        let log_num_coeffs = num_coeffs.log2();

        for logm in 1..log_num_coeffs + 1 {
            for j in (0..num_coeffs).step_by(1 << logm) {
                for i in 0..(1 << (logm - 1)) {
                    let index_even = j + i;
                    let index_odd = j + i + (1 << (logm - 1));

                    let rou_idx = i << (1 + log_num_coeffs - logm);
                    let omega_factor =
                        (&rou[rou_idx as usize] * &result[index_odd]) % self.coeff_modulus;

                    let butterfly_plus =
                        (&result[index_even] + &omega_factor).rem_euclid(&self.coeff_modulus);
                    let butterfly_minus =
                        (&result[index_even] - &omega_factor).rem_euclid(&self.coeff_modulus);

                    result[index_even] = butterfly_plus;
                    result[index_odd] = butterfly_minus;
                }
            }
        }
        result
    }

    pub fn fft_fwd(&self, coeffs: &Vec<BigInt>) -> Vec<BigInt> {
        let num_coeffs = coeffs.len();

        assert!(
            num_coeffs == self.degree as usize,
            "fft_fwd: input length does not match degree"
        );

        let fft_input: Vec<BigInt> = (0..num_coeffs)
            .map(|i| (&coeffs[i] * &self.roots_of_unity[i]) % self.coeff_modulus)
            .collect();

        self.ntt(&fft_input, &self.roots_of_unity)
    }

    pub fn fft_inv(&self, coeffs: &Vec<BigInt>) -> Vec<BigInt> {
        let num_coeffs = coeffs.len();
        assert!(
            num_coeffs == self.degree as usize,
            "fft_inv: input length does not match degree"
        );

        let to_scale_down = self.ntt(coeffs, &self.roots_of_unity_inv);
        let poly_degree_inv = invmod(self.degree as i128, self.coeff_modulus as i128);

        (0..num_coeffs)
            .map(|i| {
                (&to_scale_down[i] * &self.roots_of_unity_inv[i] * poly_degree_inv)
                    % self.coeff_modulus as i128
            })
            .collect()
    }
}
