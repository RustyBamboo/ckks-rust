use polyr::{Modulo, PolynomialRing};

use arrayvec::ArrayVec;

// b & a from equation a * s + e = b where a,s,e are randomly generated
#[derive(Debug)]
pub struct PublicKey<T>(pub PolynomialRing<T>, pub PolynomialRing<T>);
// encrypted data
#[derive(Debug)]
pub struct CipherText<T, const N: usize> {
    c: ArrayVec<PolynomialRing<T>, N>,
    scaling_factor: i32,
    modulus: i32,
}

#[derive(Debug)]
pub struct PlainText<T> {
    poly: PolynomialRing<T>,
    scaling_factor: u32,
}

type PrivateKey<T> = PolynomialRing<T>;

impl<const N: usize> std::ops::Add for &CipherText<i32, N> {
    type Output = CipherText<i32, N>;
    fn add(self, other: &CipherText<i32, N>) -> Self::Output {
        let modulus = self.modulus;
        let scaling_factor = self.scaling_factor;
        let c = self.c.iter().zip(&other.c).map(|(x, y)| x + y).collect();
        CipherText {
            c,
            modulus,
            scaling_factor,
        }
    }
}

impl<const N: usize> std::ops::Sub for &CipherText<i32, N> {
    type Output = CipherText<i32, N>;
    fn sub(self, other: &CipherText<i32, N>) -> Self::Output {
        let modulus = self.modulus;
        let scaling_factor = self.scaling_factor;
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
impl std::ops::Mul for &CipherText<i32, 2> {
    type Output = CipherText<i32, 3>;
    fn mul(self, other: &CipherText<i32, 2>) -> Self::Output {
        let modulus = self.modulus;
        let scaling_factor = self.scaling_factor;
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
pub fn relin(_relin_key: PublicKey<i32>, _c: CipherText<PolynomialRing<i32>, 3>) {
    todo!()
}

#[derive(Debug)]
pub struct Rwle<T> {
    sk: PrivateKey<T>,
    public: PublicKey<T>,
}

impl Rwle<i32> {
    pub fn keygen(modulus: i32, poly_degree: usize, size: usize) -> Self {
        // Our secret key
        let sk = PrivateKey::rand_binary(poly_degree, size);

        // First part of our public key, a
        let mut a = PolynomialRing::rand_uniform(modulus, poly_degree, size);
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
            public: PublicKey(b, a),
        }
    }

    pub fn switch_key(&self, big_modulo: i32, new_key: &PolynomialRing<i32>) -> PublicKey<i32> {
        let mod_squared = big_modulo * big_modulo;

        let swk = PolynomialRing::rand_uniform(mod_squared, self.sk.poly_degree, self.sk.len());
        let swk_e = PolynomialRing::rand_normal(self.sk.poly_degree, self.sk.len());

        let mut sw0 = (&swk * &self.sk) % mod_squared;

        sw0.coef = sw0.coef.iter().map(|x| -x).collect();
        sw0 = (&sw0 + &swk_e) % mod_squared;

        let temp = PolynomialRing::new(
            self.sk.poly_degree,
            new_key
                .coef
                .iter()
                .map(|x| (x * big_modulo).mod_ring(mod_squared))
                .collect(),
        );

        sw0 = (&sw0 + &temp) % mod_squared;

        PublicKey(sw0, swk)
    }

    pub fn relin_key(&self, big_modulo: i32) -> PublicKey<i32> {
        let sk_squared = (&self.sk * &self.sk) % big_modulo;
        self.switch_key(big_modulo, &sk_squared)
    }

    pub fn public(&self) -> &PublicKey<i32> {
        &self.public
    }

    pub fn private(&self) -> &PolynomialRing<i32> {
        &self.sk
    }

    pub fn encrypt() -> Result<(), ()> {
        todo!()
    }
}

pub fn encrypt(
    pk: &PublicKey<i32>,
    size: usize,
    modulus: i32,
    t: i32,
    poly_degree: usize,
    data: &Vec<i32>,
) -> CipherText<i32, 2> {
    let delta = modulus / t;
    let data = data
        .iter()
        .map(|x| (x * delta).rem_euclid(modulus))
        .collect();
    let data = PolynomialRing::new(poly_degree, data);

    let e1 = PolynomialRing::rand_normal(poly_degree, size);
    let e2 = PolynomialRing::rand_normal(poly_degree, size);
    let u = PolynomialRing::rand_binary(poly_degree, size);
    // Encrypt the data with b and add error.
    let c0 = (((&pk.0 * &u) % modulus + &e1) % modulus + &data) % modulus;
    // Apply u to pk1 to preserve integrity and add error.
    let c1 = (&pk.1 * &u + &e2) % modulus;

    CipherText {
        c: [c0, c1].into(),
        modulus,
        scaling_factor: delta,
    }
}

pub fn decrypt<const N: usize>(
    sk: &PolynomialRing<i32>,
    modulus: i32,
    t: i32,
    ct: CipherText<i32, N>,
) -> PolynomialRing<i32> {
    //let mut plain = (&ct.c[1] * &sk + &ct.c[0]) % modulus;
    let mut plain = ct.c[0].clone();
    let mut sk_pow = sk.clone();

    for i in 1..N {
        plain = (plain + &sk_pow * &ct.c[i]) % modulus;
        // TODO: This does one extra computation at last element. Fix this.
        sk_pow = (&sk_pow * sk) % modulus;
    }

    plain.coef = plain
        .coef
        .iter()
        .map(|&x| (x as f32 * t as f32 / modulus as f32).round() as i32 % t)
        .collect();
    plain
}

pub fn encode(message: &[f32], scaling_factor: u32) -> PlainText<i32> {
    let coef = message
        .iter()
        .map(|x| (x * scaling_factor as f32) as i32)
        .collect();
    PlainText {
        poly: PolynomialRing::new(message.len(), coef),
        scaling_factor,
    }
}

pub fn decode(plain: PlainText<i32>) -> Vec<f32> {
    plain
        .poly
        .coef
        .iter()
        .map(|&x| x as f32 / plain.scaling_factor as f32)
        .collect()
}
