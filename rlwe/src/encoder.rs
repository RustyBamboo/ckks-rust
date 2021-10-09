use num_complex::Complex64;
use std::f64::consts::PI;

///
/// Reverse bits of an integer with specified width.
///
/// E.g. Reverse 6 = 0b110 with a width of 5 would result in reversing 0b00110, which becomes
/// 0b01100 = 12
///
pub fn reverse_bits(value: usize, width: u32) -> usize {
    value.reverse_bits() >> (usize::BITS - width)
}

///
/// Reverse an array by reversing bits of indicides
///
pub fn bit_reverse_vec<T>(mut values: Vec<T>) -> Vec<T> {
    let len = values.len();
    for i in 0..len / 2 {
        values.swap(i, reverse_bits(i, len.log2()));
    }
    values
}

///
/// Algorithms from "Improved Bootstrapping for Approximate Homomorphic Encryption"
///
pub struct CKKSEncoder {
    roots_of_unity: Vec<Complex64>,
    roots_of_unity_inv: Vec<Complex64>,
    rot_group: Vec<i64>,
    fft_length: usize,
}

impl CKKSEncoder {
    pub fn new(fft_length: usize) -> CKKSEncoder {
        let angles: Vec<f64> = (0..fft_length)
            .map(|i| 2. * PI * i as f64 / fft_length as f64)
            .collect();

        // Precompute roots of unity and their inverse
        let roots_of_unity = angles
            .iter()
            .map(|angle| Complex64::new(angle.cos(), angle.sin()))
            .collect();
        let roots_of_unity_inv = angles
            .iter()
            .map(|angle| Complex64::new((-angle).cos(), (-angle).sin()))
            .collect();

        let num_slots = fft_length / 4;
        //let width = num_slots.log2() as u32;

        // Precompute reversed bits
        // let reverse_bits = (0..num_slots).map(|i| reverse_bits(i, width) % num_slots).collect();

        // Precompute group with power of 5
        let mut rot_group = vec![1i64; num_slots];
        for i in 1..num_slots {
            rot_group[i] = (5 * rot_group[i - 1]).rem_euclid(fft_length as i64);
        }

        CKKSEncoder {
            roots_of_unity,
            roots_of_unity_inv,
            rot_group,
            fft_length,
        }
    }

    pub fn embedding(&self, coeffs: Vec<Complex64>) -> Vec<Complex64> {
        let num_coeffs = coeffs.len();
        let mut result = bit_reverse_vec(coeffs);

        let log_num_coeffs = num_coeffs.log2();

        for logm in 1..log_num_coeffs + 1 {
            let idx_mod = 1 << (logm + 2);
            let gap = self.fft_length / idx_mod;

            for j in (0..num_coeffs).step_by(1 << logm) {
                for i in 0..(1 << (logm - 1)) {
                    let index_even = j + i;
                    let index_odd = j + i + (1 << (logm - 1));

                    let rou_idx = (self.rot_group[i].rem_euclid(idx_mod as i64)) * gap as i64;
                    let omega_factor = self.roots_of_unity[rou_idx as usize] * result[index_odd];

                    let butterfly_plus = result[index_even] + omega_factor;
                    let butterfly_minus = result[index_even] - omega_factor;

                    result[index_even] = butterfly_plus;
                    result[index_odd] = butterfly_minus;
                }
            }
        }
        result
    }

    pub fn embedding_inv(&self, coeffs: Vec<Complex64>) -> Vec<Complex64> {
        let num_coeffs = coeffs.len();
        let mut result = coeffs;

        let log_num_coeffs = num_coeffs.log2();

        for logm in (1..log_num_coeffs + 1).rev() {
            let idx_mod = 1 << (logm + 2);
            let gap = self.fft_length / idx_mod;

            for j in (0..num_coeffs).step_by(1 << logm) {
                for i in 0..(1 << (logm - 1)) {
                    let index_even = j + i;
                    let index_odd = j + i + (1 << (logm - 1));

                    let rou_idx = (self.rot_group[i].rem_euclid(idx_mod as i64)) * gap as i64;

                    let butterfly_plus = result[index_even] + result[index_odd];
                    let mut butterfly_minus = result[index_even] - result[index_odd];
                    butterfly_minus *= self.roots_of_unity_inv[rou_idx as usize];

                    result[index_even] = butterfly_plus;
                    result[index_odd] = butterfly_minus;
                }
            }
        }
        let to_scale_down = bit_reverse_vec(result);

        to_scale_down
            .iter()
            .map(|x| x / num_coeffs as f64)
            .collect()
    }
}
