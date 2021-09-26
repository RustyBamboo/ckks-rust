use polyr::{polynomial_ring, PolynomialRing};

#[derive(Debug)]
pub struct PublicKey<T>(pub PolynomialRing<T>, pub PolynomialRing<T>);
#[derive(Debug)]
pub struct CipherText<T>(PolynomialRing<T>, PolynomialRing<T>);

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

        // First part of our public key
        let mut a = PolynomialRing::rand_uniform(ring, pmod, size);
        // Flip the sign in calculation of public key
        a.coef = a.coef.iter().map(|x| -x).collect();

        // A little bit of noise
        let e = PolynomialRing::rand_normal(ring, pmod, size);

        // Second part of public key
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
    data: Vec<i32>,
) -> CipherText<i32> {
    let delta = ring / t;
    let data = data.iter().map(|x| (x * delta).rem_euclid(ring)).collect();
    let data = PolynomialRing::new(ring, pmod, data);

    let e1 = PolynomialRing::rand_normal(ring, pmod, size);
    let e2 = PolynomialRing::rand_normal(ring, pmod, size);
    let u = PolynomialRing::rand_binary(ring, pmod, size);

    let ct0 = (&pk.0 * &u) + &e1 + &data;

    let ct1 = &pk.1 * &u + &e2;

    CipherText(ct0, ct1)
}

pub fn decrypt(
    sk: &PolynomialRing<i32>,
    ring: i32,
    t: i32,
    ct: CipherText<i32>,
) -> PolynomialRing<i32> {
    let mut plain = &ct.1 * &sk + &ct.0;
    plain.coef = plain
        .coef
        .iter()
        .map(|&x| (x as f32 * t as f32 / ring as f32).round() as i32 % t)
        .collect();
    plain
}
