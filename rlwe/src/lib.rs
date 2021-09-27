use polyr::{polynomial_ring, PolynomialRing};

// b & a from equation a * s + e = b where a,s,e are randomly generated 
#[derive(Debug)]
pub struct PublicKey<T>(pub PolynomialRing<T>, pub PolynomialRing<T>);
// encrypted data
#[derive(Debug)]
pub struct CipherText<T>(PolynomialRing<T>, PolynomialRing<T>);

impl std::ops::Add for &CipherText<i32> {
    type Output = CipherText<i32>;
    fn add(self, other: &CipherText<i32>) -> CipherText<i32> {
        CipherText {
            0: &self.0 + &other.0,
            1: &self.1 + &other.1,
        }
    }
}
impl std::ops::Mul for &CipherText<i32> {
    type Output = CipherText<i32>;
    fn mul(self, other: &CipherText<i32>) -> CipherText<i32> {
        CipherText {
            0: &self.0 * &other.0,
            1: &self.1 * &other.1,
        }
    }
}

#[derive(Debug)]
pub struct Rwle<T> {
    sk: PolynomialRing<T>,
    public: PublicKey<T>,
    e: PolynomialRing<T>,
}

impl Rwle<i32> {
    pub fn keygen(ring: i32, pmod: usize, size: usize) -> Self {
        // Our secret key
        let sk = PolynomialRing::rand_binary(ring, pmod, size);

        // First part of our public key, a
        let mut a = PolynomialRing::rand_uniform(ring, pmod, size);
        // Flip the sign in calculation of public key
        a.coef = a.coef.iter().map(|x| -x).collect();

        // A little bit of noise
        let e = PolynomialRing::rand_normal(ring, pmod, size);

        // Second part of public key, b
        let b = &a * &sk + &e;

        // Return back sign
        a.coef = a.coef.iter().map(|x| -x).collect();

        Rwle {
            sk,
            e,
            public: PublicKey(b, a),
        }
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
    ring: i32,
    t: i32,
    pmod: usize,
    data: &Vec<i32>,
) -> CipherText<i32> {
    let delta = ring / t;
    let data = data.iter().map(|x| (x * delta).rem_euclid(ring)).collect();
    let data = PolynomialRing::new(ring, pmod, data);

    let e1 = PolynomialRing::rand_normal(ring, pmod, size);
    let e2 = PolynomialRing::rand_normal(ring, pmod, size);
    let u = PolynomialRing::rand_binary(ring, pmod, size);
    // Encrypt the data with b and add error. 
    let ct0 = (&pk.0 * &u) + &e1 + &data;
    // Apply u to pk1 to preserve integrity and add error.
    let ct1 = &pk.1 * &u + &e2;

    CipherText(ct0, ct1)
}

pub fn decrypt(
    sk: &PolynomialRing<i32>,
    ring: i32,
    t: i32,
    ct: CipherText<i32>,
) -> PolynomialRing<i32> {
    // ((a) * u) * s) + ((-a * s) * u) + d 
    // ~b * (~-b + d) = d cause error don't matter if you round!
    let mut plain = &ct.1 * &sk + &ct.0;
    plain.coef = plain
        .coef
        .iter()
        .map(|&x| (x as f32 * t as f32 / ring as f32).round() as i32 % t)
        .collect();
    plain
}
