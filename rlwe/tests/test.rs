use approx::assert_relative_eq;

use num_bigint::{BigInt, ToBigInt};
use num_traits::One;

use rlwe::*;

#[test]
fn encrypt_decrypt() {
    let n = 2_i32.pow(4);
    let q = 2_i32.pow(15).to_bigint().unwrap();

    // Create a keypair 100 times
    for _ in 0..100 {
        // Create key pair
        let key = Rwle::keygen(&q, n as usize, n as usize);

        // Create our data with a single datapoint
        let x = 0.02_64;
        let mut data = vec![0f64; n as usize];
        data[0] = x;

        let plain = encode(data.as_slice(), 1usize << 19);

        // Encrypt our data using keypair
        let cipher1 = encrypt(key.public(), &q, &plain);

        // Decrypt our data
        let out = decrypt(&key.private(), cipher1);

        let decode = decode(out);

        assert_relative_eq!(x, decode[0], epsilon = 1e-4);
    }
}

#[test]
fn add() {
    let n = 4;

    let q = 1.to_bigint().unwrap() << 70;

    let key = Rwle::keygen(&q, n as usize, n as usize);

    let x = [0.05, 0.1, 1.0, 0.005];
    let y = [0.1, 0.02, 0.5, 0.3];

    let plainx = encode(&x, 1usize << 50);
    let plainy = encode(&y, 1usize << 50);

    let cipherx = encrypt(&key.public(), &q, &plainx);
    let ciphery = encrypt(&key.public(), &q, &plainy);

    let cipherz = &cipherx + &ciphery;

    let plainz = decrypt(&key.private(), cipherz);

    let z = decode(plainz);

    let expected_z: Vec<f64> = x.iter().zip(&y).map(|(a, b)| a + b).collect();

    for (&x, y) in expected_z.iter().zip(z) {
        assert_relative_eq!(x, y, epsilon = 1e-4)
    }
}

#[test]
fn mul() {
    let poly_degree = 4;
    let ciph_modulus = (1 << 30).to_bigint().unwrap();
    let big_modulus = 1 << 31;
    let scaling_factor = 1_usize << 10;

    let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize);

    let message = [0.5, 0.3, 0.78, 0.88];
    let plain = encode(&message, scaling_factor);
    println!("{:?}", plain);
    println!("{:?}", decode(plain));
    //assert_eq!(expected_z, z.coef);
}
