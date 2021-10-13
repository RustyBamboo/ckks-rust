use crate::utils::{bit_reverse_vec, invmod, root_of_unity};

///
/// Parameters for Number/Fermat Theoretic Transform
///
#[derive(Debug)]
pub struct Ntt {
    coeff_modulus: i128,
    degree: usize,
    pub roots_of_unity: Vec<i128>,
    pub roots_of_unity_inv: Vec<i128>,
}

impl Ntt {
    pub fn new(degree: usize, coeff_modulus: i128) -> Self {
        assert!(
            degree & (degree - 1) == 0,
            "Polynomial degree needs to be a power of 2"
        );

        let root_of_unity = root_of_unity(2 * degree as i128, coeff_modulus);
        let root_of_unity_inv = invmod(root_of_unity, coeff_modulus);

        let mut roots_of_unity = vec![1_i128; degree];
        let mut roots_of_unity_inv = vec![1_i128; degree];

        for i in 1..degree {
            roots_of_unity[i] = (roots_of_unity[i - 1] * root_of_unity).rem_euclid(coeff_modulus);
        }

        for i in 1..degree {
            roots_of_unity_inv[i] =
                (roots_of_unity_inv[i - 1] * root_of_unity_inv).rem_euclid(coeff_modulus);
        }

        Ntt {
            coeff_modulus,
            degree,
            roots_of_unity,
            roots_of_unity_inv,
        }
    }

    pub fn ntt(&self, coeffs: &Vec<i128>, rou: &Vec<i128>) -> Vec<i128> {
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
                        (rou[rou_idx as usize] * result[index_odd]).rem_euclid(self.coeff_modulus);

                    let butterfly_plus =
                        (result[index_even] + omega_factor).rem_euclid(self.coeff_modulus);
                    let butterfly_minus =
                        (result[index_even] - omega_factor).rem_euclid(self.coeff_modulus);

                    result[index_even] = butterfly_plus;
                    result[index_odd] = butterfly_minus;
                }
            }
        }
        result
    }

    pub fn fft_fwd(&self, coeffs: &Vec<i128>) -> Vec<i128> {
        let num_coeffs = coeffs.len();

        assert!(
            num_coeffs == self.degree,
            "fft_fwd: input length does not match degree"
        );

        let fft_input: Vec<i128> = (0..num_coeffs)
            .map(|i| (coeffs[i] * self.roots_of_unity[i]).rem_euclid(self.coeff_modulus))
            .collect();

        self.ntt(&fft_input, &self.roots_of_unity)
    }

    pub fn fft_inv(&self, coeffs: &Vec<i128>) -> Vec<i128> {
        let num_coeffs = coeffs.len();
        assert!(
            num_coeffs == self.degree,
            "fft_inv: input length does not match degree"
        );

        let to_scale_down = self.ntt(coeffs, &self.roots_of_unity_inv);
        let poly_degree_inv = invmod(self.degree as i128, self.coeff_modulus);

        (0..num_coeffs)
            .map(|i| {
                (to_scale_down[i] * self.roots_of_unity_inv[i] * poly_degree_inv)
                    % self.coeff_modulus
            })
            .collect()
    }
}
