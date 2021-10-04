use polyr::{polynomial_ring, Modulo, PolynomialRing};

// b & a from equation a * s + e = b where a,s,e are randomly generated
#[derive(Debug)]
pub struct PublicKey<T>(pub PolynomialRing<T>, pub PolynomialRing<T>);
// encrypted data
#[derive(Debug)]
pub struct CipherText<T> {
    c0: PolynomialRing<T>,
    c1: PolynomialRing<T>,
    scaling_factor: i32,
    modulus: i32,
}

#[derive(Debug)]
pub struct PlainText<T> {
    poly: PolynomialRing<T>,
    scaling_factor: u32,
}

type PrivateKey<T> = PolynomialRing<T>;

impl std::ops::Add for &CipherText<i32> {
    type Output = CipherText<i32>;
    fn add(self, other: &CipherText<i32>) -> CipherText<i32> {
        let modulus = self.modulus;
        let scaling_factor = self.scaling_factor;
        CipherText {
            c0: (&self.c0 + &other.c0) % modulus,
            c1: (&self.c1 + &other.c1) % modulus,
            modulus,
            scaling_factor,
        }
    }
}

impl std::ops::Sub for &CipherText<i32> {
    type Output = CipherText<i32>;
    fn sub(self, other: &CipherText<i32>) -> CipherText<i32> {
        let modulus = self.modulus;
        let scaling_factor = self.scaling_factor;
        CipherText {
            c0: (&self.c0 - &other.c0) % modulus,
            c1: (&self.c1 - &other.c1) % modulus,
            modulus,
            scaling_factor,
        }
    }
}

impl std::ops::Mul for &CipherText<i32> {
    type Output = CipherText<i32>;
    fn mul(self, other: &CipherText<i32>) -> CipherText<i32> {
        let c0 = &self.c0 * &other.c0;
        let c1 = &self.c0 * &other.c1 + &self.c1 * &other.c0;
        let c2 = &self.c1 * &other.c1;
        //CipherText {
        //    c0: &self.c0 * &other.c0,
        //    c1: &self.c1 * &other.c1,
        //}
        todo!()
    }
}

///
/// This takes a 3-dimensional ciphertext and reduces it back into 2-dimensions
///
pub fn relin(
    relin_key: PublicKey<i32>,
    c0: PolynomialRing<i32>,
    c1: PolynomialRing<i32>,
    c2: PolynomialRing<i32>,
) {
    // modulo * big modulus
    let new_c0 = relin_key.0 * c2;
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
) -> CipherText<i32> {
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
        c0,
        c1,
        modulus,
        scaling_factor: delta,
    }
}

pub fn decrypt(
    sk: &PolynomialRing<i32>,
    modulus: i32,
    t: i32,
    ct: CipherText<i32>,
) -> PolynomialRing<i32> {
    // ((a) * u) * s) + ((-a * s) * u) + d
    // ~b * (~-b + d) = d cause error don't matter if you round!
    let mut plain = (&ct.c1 * &sk + &ct.c0) % modulus;
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
