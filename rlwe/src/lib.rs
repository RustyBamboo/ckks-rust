#![feature(int_log)]
pub mod encoder;

use encoder::CKKSEncoder;

use polyr::{Modulo, PolynomialRing};

use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use num_complex::Complex64;
use num_rational::Ratio;
use num_traits::cast::ToPrimitive;
use num_traits::Zero;

use algebra::crt::Crt;

use arrayvec::ArrayVec;

// b & a from equation a * s + e = b where a,s,e are randomly generated
#[derive(Debug)]
pub struct PublicKey<'n, T>(pub PolynomialRing<'n, T>, pub PolynomialRing<'n, T>);
// encrypted data
#[derive(Debug)]
pub struct CipherText<'n, T, const N: usize> {
    c: ArrayVec<PolynomialRing<'n, T>, N>,
    scaling_factor: BigUint,
    modulus: BigInt,
}

impl<T, const N: usize> CipherText<'_, T, N> {
    pub fn dim(&self) -> usize {
        self.c.len()
    }
}

impl<'n> CipherText<'n, BigInt, 3> {
    ///
    /// This takes a 3-dimensional ciphertext and reduces it back into 2-dimensions
    ///
    pub fn relin(
        &self,
        relin_key: &'n PublicKey<BigInt>,
        big_modulus: &'n BigInt,
    ) -> CipherText<BigInt, 2> {
        let modulus = &self.modulus;

        let mut new_c0 = (&relin_key.0 * &self.c[2]) % &(modulus * big_modulus);
        new_c0.coef = new_c0.coef.iter().map(|x| x / big_modulus).collect();
        new_c0 = (new_c0 + &self.c[0]) % &modulus;

        let mut new_c1 = (&relin_key.1 * &self.c[2]) % &(modulus * big_modulus);
        new_c1.coef = new_c1.coef.iter().map(|x| x / big_modulus).collect();
        new_c1 = (new_c1 + &self.c[1]) % &modulus;

        CipherText {
            c: [new_c0, new_c1].into(),
            modulus: modulus.clone(),
            scaling_factor: self.scaling_factor.clone(),
        }
    }
}

#[derive(Debug)]
pub struct PlainText<'n, T> {
    pub poly: PolynomialRing<'n, T>,
    scaling_factor: BigUint,
}

type PrivateKey<'n, T> = PolynomialRing<'n, T>;

impl<'n, const N: usize> std::ops::Add for &CipherText<'n, BigInt, N> {
    type Output = CipherText<'n, BigInt, N>;
    fn add(self, other: &CipherText<'n, BigInt, N>) -> Self::Output {
        let modulus = self.modulus.clone();
        let scaling_factor = self.scaling_factor.clone();
        let c = self.c.iter().zip(&other.c).map(|(x, y)| x + y).collect();
        CipherText {
            c,
            modulus,
            scaling_factor,
        }
    }
}

impl<'n, const N: usize> std::ops::Sub for &CipherText<'n, BigInt, N> {
    type Output = CipherText<'n, BigInt, N>;
    fn sub(self, other: &CipherText<'n, BigInt, N>) -> Self::Output {
        let modulus = self.modulus.clone();
        let scaling_factor = self.scaling_factor.clone();
        let c = self.c.iter().zip(&other.c).map(|(x, y)| x - y).collect();
        CipherText {
            c,
            modulus,
            scaling_factor,
        }
    }
}

///
/// Multiply two ciphertexts together.
///
/// This will increase the dimensionality of the ciphertext. In the current implementation it
/// goes from dim 2 -> dim 3
///
impl<'a, 'b, 'n> std::ops::Mul<&'b CipherText<'n, BigInt, 2>> for &'a CipherText<'n, BigInt, 2> {
    type Output = CipherText<'n, BigInt, 3>;
    fn mul(self, other: &'b CipherText<'n, BigInt, 2>) -> Self::Output {
        let modulus = self.modulus.clone();

        let c0 = &self.c[0] * &other.c[0];
        let c0 = c0 % &modulus;

        let c1 = &self.c[0] * &other.c[1] + &other.c[0] * &self.c[1];
        let c1 = c1 % &modulus;

        let c2 = &self.c[1] * &other.c[1];
        let c2 = c2 % &modulus;

        CipherText {
            c: [c0, c1, c2].into(),
            modulus,
            scaling_factor: &self.scaling_factor * &other.scaling_factor,
        }
    }
}

#[derive(Debug)]
pub struct Rwle<'n, T> {
    sk: PrivateKey<'n, T>,
    pk: PublicKey<'n, T>,
}

impl<'a> Rwle<'a, BigInt> {
    pub fn add_crt(mut self, crt: &'a Crt) -> Self {
        let sk = self.sk.add_crt(crt);
        let pk = PublicKey(self.pk.0.add_crt(crt), self.pk.1.add_crt(crt));
        self.sk = sk;
        self.pk = pk;
        self
    }

    pub fn keygen(modulus: &BigInt, poly_degree: usize, size: usize) -> Self {
        // Our secret key
        let sk = PrivateKey::rand_binary(poly_degree, size);

        // First part of our public key, a
        let mut a = PolynomialRing::rand_uniform(&modulus, poly_degree, size);
        // Flip the sign in calculation of public key
        a.coef = a.coef.iter().map(|x| -x).collect();

        // A little bit of noise
        let e = PolynomialRing::rand_normal(poly_degree, size);

        // Second part of public key, b
        let b = &a * &sk + &e;

        // Return back sign
        a.coef = a.coef.iter().map(|x| -x).collect();

        Rwle {
            sk,
            pk: PublicKey(b, a),
        }
    }

    pub fn switch_key(
        &self,
        big_modulo: &BigInt,
        new_key: &PolynomialRing<BigInt>,
    ) -> PublicKey<BigInt> {
        let mod_squared = big_modulo * big_modulo;

        let swk = PolynomialRing::rand_uniform(&mod_squared, self.sk.poly_degree, self.sk.len());
        let swk_e = PolynomialRing::rand_normal(self.sk.poly_degree, self.sk.len());

        let mut sw0 = (&swk * &self.sk) % &mod_squared;

        sw0.coef = sw0.coef.iter().map(|x| -x).collect();
        sw0 = (&sw0 + &swk_e) % &mod_squared;

        let temp = PolynomialRing::new(
            self.sk.poly_degree,
            new_key
                .coef
                .iter()
                .map(|x| (x * big_modulo).mod_ring(&mod_squared))
                .collect(),
        );

        sw0 = (&sw0 + &temp) % &mod_squared;

        PublicKey(sw0, swk)
    }

    pub fn relin_key(&self, big_modulo: &BigInt) -> PublicKey<BigInt> {
        let sk_squared = (&self.sk * &self.sk) % &big_modulo;
        self.switch_key(&big_modulo, &sk_squared)
    }

    pub fn public(&self) -> &PublicKey<BigInt> {
        &self.pk
    }

    pub fn private(&self) -> &PrivateKey<BigInt> {
        &self.sk
    }
}

pub fn encrypt<'n>(
    pk: &'n PublicKey<BigInt>,
    modulus: &BigInt,
    plain: &'n PlainText<BigInt>,
) -> CipherText<'n, BigInt, 2> {
    let poly_degree = plain.poly.poly_degree;
    let size = poly_degree;

    let e1 = PolynomialRing::rand_normal(poly_degree, size);
    let e2 = PolynomialRing::rand_normal(poly_degree, size);
    let u = PolynomialRing::rand_binary(poly_degree, size);

    // Encrypt the data with b and add error.
    let c0 = (((&pk.0 * &u) % &modulus + &e1) % &modulus + &plain.poly) % &modulus;

    // Apply u to pk1 to preserve integrity and add error.
    let c1 = (&pk.1 * &u + &e2) % &modulus;

    CipherText {
        c: [c0, c1].into(),
        modulus: modulus.clone(),
        scaling_factor: plain.scaling_factor.clone(),
    }
}

pub fn decrypt<'n, const N: usize>(
    sk: &'n PolynomialRing<BigInt>,
    ct: CipherText<'n, BigInt, N>,
) -> PlainText<'n, BigInt> {
    let modulus = ct.modulus;

    let mut poly = ct.c[0].clone();
    let mut sk_pow = sk.clone();

    for i in 1..N {
        poly = (poly + &sk_pow * &ct.c[i]) % &modulus;
        // TODO: This does one extra computation at last element. Fix this.
        sk_pow = (&sk_pow * &sk) % &modulus;
    }

    let poly = poly % &modulus;

    PlainText {
        poly,
        scaling_factor: ct.scaling_factor,
    }
}

pub fn encode<'n>(
    message: &[f64],
    scaling_factor: usize,
    encoder: &CKKSEncoder,
) -> PlainText<'n, BigInt> {
    let num_values = message.len();
    let plain_len = num_values << 1;

    let message = message.iter().map(|&x| Complex64::new(x, 0.)).collect();

    let to_scale = encoder.embedding_inv(&message);

    let mut coef = vec![Zero::zero(); plain_len];

    for i in 0..num_values {
        coef[i] = (to_scale[i].re * scaling_factor as f64 + 0.5)
            .to_bigint()
            .unwrap();
        coef[i + num_values] = (to_scale[i].im * scaling_factor as f64 + 0.5)
            .to_bigint()
            .unwrap();
    }

    PlainText {
        poly: PolynomialRing::new(plain_len, coef),
        scaling_factor: scaling_factor.to_biguint().unwrap(),
    }
}

pub fn decode(plain: PlainText<BigInt>, encoder: &CKKSEncoder) -> Vec<Complex64> {
    let scaling_factor = plain.scaling_factor.to_bigint().unwrap();
    let plain_len = plain.poly.len();
    let num_values = plain_len >> 1;

    let mut coef = vec![Complex64::zero(); num_values];

    for i in 0..num_values {
        let r1 = Ratio::new(plain.poly.coef[i].clone(), scaling_factor.clone());
        let r2 = Ratio::new(
            plain.poly.coef[i + num_values].clone(),
            scaling_factor.clone(),
        );

        coef[i] = Complex64::new(r1.to_f64().unwrap(), r2.to_f64().unwrap());
    }

    encoder.embedding(&coef)
}
