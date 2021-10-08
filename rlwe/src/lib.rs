use polyr::{Modulo, PolynomialRing};

use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};

use arrayvec::ArrayVec;

// b & a from equation a * s + e = b where a,s,e are randomly generated
#[derive(Debug)]
pub struct PublicKey<T>(pub PolynomialRing<T>, pub PolynomialRing<T>);
// encrypted data
#[derive(Debug)]
pub struct CipherText<T, const N: usize> {
    c: ArrayVec<PolynomialRing<T>, N>,
    scaling_factor: BigUint,
    modulus: BigInt,
}

#[derive(Debug)]
pub struct PlainText<T> {
    pub poly: PolynomialRing<T>,
    scaling_factor: BigUint,
}

type PrivateKey<T> = PolynomialRing<T>;

impl<const N: usize> std::ops::Add for &CipherText<BigInt, N> {
    type Output = CipherText<BigInt, N>;
    fn add(self, other: &CipherText<BigInt, N>) -> Self::Output {
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

impl<const N: usize> std::ops::Sub for &CipherText<BigInt, N> {
    type Output = CipherText<BigInt, N>;
    fn sub(self, other: &CipherText<BigInt, N>) -> Self::Output {
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
impl std::ops::Mul for &CipherText<BigInt, 2> {
    type Output = CipherText<BigInt, 3>;
    fn mul(self, other: &CipherText<BigInt, 2>) -> Self::Output {
        let modulus = self.modulus.clone();
        let scaling_factor = self.scaling_factor.clone();
        let c0 = &self.c[0] * &other.c[0];
        let c1 = &self.c[1] * &other.c[1] + &self.c[1] * &other.c[0];
        let c2 = &self.c[1] * &other.c[1];

        CipherText {
            c: [c0, c1, c2].into(),
            modulus,
            scaling_factor,
        }
    }
}

///
/// This takes a 3-dimensional ciphertext and reduces it back into 2-dimensions
///
pub fn relin(_relin_key: PublicKey<BigInt>, _c: CipherText<PolynomialRing<BigInt>, 3>) {
    todo!()
}

#[derive(Debug)]
pub struct Rwle<T> {
    sk: PrivateKey<T>,
    pk: PublicKey<T>,
}

impl Rwle<BigInt> {
    pub fn keygen(modulus: BigInt, poly_degree: usize, size: usize) -> Self {
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

    pub fn switch_key(&self, big_modulo: &BigInt, new_key: &PolynomialRing<BigInt>) -> PublicKey<BigInt> {
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

pub fn encrypt(pk: &PublicKey<BigInt>, modulus: BigInt, plain: &PlainText<BigInt>) -> CipherText<BigInt, 2> {
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
        modulus,
        scaling_factor: plain.scaling_factor.clone(),
    }
}

pub fn decrypt<const N: usize>(sk: &PolynomialRing<BigInt>, ct: CipherText<BigInt, N>) -> PlainText<BigInt> {
    let modulus = ct.modulus;

    let mut poly = ct.c[0].clone();
    let mut sk_pow = sk.clone();

    for i in 1..N {
        poly = (poly + &sk_pow * &ct.c[i]) % &modulus;
        // TODO: This does one extra computation at last element. Fix this.
        sk_pow = (&sk_pow * sk) % &modulus;
    }

    PlainText {
        poly,
        scaling_factor: ct.scaling_factor,
    }
}

pub fn encode(message: &[f64], scaling_factor: usize) -> PlainText<BigInt> {
    let coef = message
        .iter()
        .map(|x| ((x * scaling_factor as f64) as i128).to_bigint().unwrap())
        .collect();
    PlainText {
        poly: PolynomialRing::new(message.len(), coef),
        scaling_factor: scaling_factor.to_biguint().unwrap(),
    }
}

pub fn decode(plain: PlainText<BigInt>) -> Vec<BigInt> {
    plain
        .poly
        .coef
        .iter()
        .map(|x| x / plain.scaling_factor.to_bigint().unwrap())
        .collect()
}
