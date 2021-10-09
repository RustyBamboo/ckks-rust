use approx::assert_relative_eq;

use num_bigint::ToBigInt;
use rlwe::*;

#[test]
fn encoder() {
    let n = 8;

    let encoder = encoder::CKKSEncoder::new(n as usize * 2);

    let x = [0.5, 0.3, 0.78, 0.88];
    let plainx = encode(&x, 1usize << 30, &encoder);

    println!("{:?}", plainx);
}

#[test]
fn encrypt_decrypt() {
    let n = 8;
    let q = 1.to_bigint().unwrap() << 600;

    let encoder = encoder::CKKSEncoder::new(n as usize * 2);

    // Create a keypair 100 times
    for _ in 0..100 {
        // Create key pair
        let key = Rwle::keygen(&q, n as usize, n as usize);

        // Create our data with a single datapoint
        let x = 0.02_64;
        let data = vec![x; n / 2 as usize];

        let plain = encode(data.as_slice(), 1usize << 30, &encoder);

        // Encrypt our data using keypair
        let cipher1 = encrypt(key.public(), &q, &plain);

        // Decrypt our data
        let out = decrypt(&key.private(), cipher1);

        let decode = decode(out, &encoder);

        assert_relative_eq!(x, decode[0].re, epsilon = 1e-4);
    }
}

#[test]
fn add() {
    let n = 8;

    let q = 1.to_bigint().unwrap() << 600;

    let key = Rwle::keygen(&q, n as usize, n as usize);
    let encoder = encoder::CKKSEncoder::new(n as usize * 2);

    let x = [0.05, 0.1, 1.0, 0.005];
    let y = [0.1, 0.02, 0.5, 0.3];

    let plainx = encode(&x, 1usize << 30, &encoder);
    let plainy = encode(&y, 1usize << 30, &encoder);

    let cipherx = encrypt(&key.public(), &q, &plainx);
    let ciphery = encrypt(&key.public(), &q, &plainy);

    let cipherz = &cipherx + &ciphery;

    let plainz = decrypt(&key.private(), cipherz);

    let z = decode(plainz, &encoder);

    let expected_z: Vec<f64> = x.iter().zip(&y).map(|(a, b)| a + b).collect();

    for (&x, y) in expected_z.iter().zip(z) {
        assert_relative_eq!(x, y.re, epsilon = 1e-4)
    }
}

#[test]
fn mul() {
    let poly_degree = 4 * 2;
    let ciph_modulus = 1.to_bigint().unwrap() << 600;
    let scaling_factor = 1_usize << 30;

    let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize);
    let encoder = encoder::CKKSEncoder::new(poly_degree as usize * 2);

    let x = [0.05, 0.1, 1.0, 0.005];
    let y = [0.1, 0.02, 0.5, 0.3];
    let plainx = encode(&x, scaling_factor, &encoder);
    let plainy = encode(&y, scaling_factor, &encoder);

    let cipherx = encrypt(&key.public(), &ciph_modulus, &plainx);
    let ciphery = encrypt(&key.public(), &ciph_modulus, &plainy);

    let cipherz = &cipherx * &ciphery;

    let plainz = decrypt(&key.private(), cipherz);

    let z = decode(plainz, &encoder);
    let expected_z: Vec<f64> = x.iter().zip(&y).map(|(a, b)| a * b).collect();

    for (&x, y) in expected_z.iter().zip(z) {
        assert_relative_eq!(x, y.re, epsilon = 1e-4)
    }
}

#[test]
fn mul_relin() {
    let poly_degree = 4 * 2;
    let ciph_modulus = 1.to_bigint().unwrap() << 600;
    let big_modulus = 1.to_bigint().unwrap() << 1200;
    let scaling_factor = 1_usize << 30;

    let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize);
    let relin_key = key.relin_key(&big_modulus);
    let encoder = encoder::CKKSEncoder::new(poly_degree as usize * 2);

    let x = [0.05, 0.1, 1.0, 0.005];
    let y = [0.1, 0.02, 0.5, 0.3];
    let plainx = encode(&x, scaling_factor, &encoder);
    let plainy = encode(&y, scaling_factor, &encoder);

    let cipherx = encrypt(&key.public(), &ciph_modulus, &plainx);
    let ciphery = encrypt(&key.public(), &ciph_modulus, &plainy);

    let cipherz_relin = (&cipherx * &ciphery).relin(&relin_key, &big_modulus);
    let plainz_relin = decrypt(&key.private(), cipherz_relin);
    let z_relin = decode(plainz_relin, &encoder);

    let expected_z: Vec<f64> = x.iter().zip(&y).map(|(a, b)| a * b).collect();
    for (&x, y) in expected_z.iter().zip(z_relin) {
        assert_relative_eq!(x, y.re, epsilon = 1e-4)
    }
}

#[test]
fn reverse_bits() {
    use encoder::reverse_bits;
    let x = 6;
    assert_eq!(reverse_bits(x, 5), 12);
}
