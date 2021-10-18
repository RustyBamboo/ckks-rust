use approx::assert_relative_eq;

use num_bigint::ToBigInt;
use rlwe::*;

use algebra::crt::Crt;

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

    let prod = &cipherx * &ciphery;
    let cipherz_relin = prod.relin(&relin_key, &big_modulus);
    let plainz_relin = decrypt(&key.private(), cipherz_relin);
    let z_relin = decode(plainz_relin, &encoder);

    let expected_z: Vec<f64> = x.iter().zip(&y).map(|(a, b)| a * b).collect();
    for (&x, y) in expected_z.iter().zip(z_relin) {
        assert_relative_eq!(x, y.re, epsilon = 1e-4)
    }
}

#[test]
fn reverse_bits() {
    use algebra::utils::reverse_bits;
    let x = 6;
    assert_eq!(reverse_bits(x, 5), 12);
}

#[test]
fn mnist() {
    let four_img: [u8; 28 * 28] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 232, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 81, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 180, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 163, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 153, 210, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 163, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 254, 162, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 163, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 254, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 245, 163,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 198, 254, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 254,
        163, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 231, 254, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159,
        254, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 163, 254, 216, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        159, 254, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 86, 178, 248, 254, 91, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 159, 254, 85, 0, 0, 0, 47, 49, 116, 144, 150, 241, 243, 234, 179, 241, 252, 40, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 150, 253, 237, 207, 207, 207, 253, 254, 250, 240, 198, 143, 91, 28,
        5, 233, 250, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 177, 177, 177, 177, 177, 98, 56, 0,
        0, 0, 0, 0, 102, 254, 220, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 169, 254, 137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 169, 254, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 169, 254, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 169, 255, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 169, 254, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        169, 254, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        169, 255, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        96, 254, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let mut four_img_f64 = [0f64; 28 * 28];

    for i in 0..28 * 28 {
        four_img_f64[i] = four_img[i] as f64 / 255f64;
    }

    let poly_degree = 1 << 11;
    let ciph_modulus = 1.to_bigint().unwrap() << 600;
    let scaling_factor = 1_usize << 30;

    let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize);
    let encoder = encoder::CKKSEncoder::new(poly_degree as usize * 2);

    let padded_message: [f64; 1024] = {
        let mut whole: [f64; 1024] = [0.; 1024];
        let (one, two) = whole.split_at_mut(28 * 28);
        one.copy_from_slice(&four_img_f64);
        two.copy_from_slice(&[0.; 240]);
        whole
    };
    let plain = encode(&padded_message, scaling_factor, &encoder);
    let cipher = encrypt(&key.public(), &ciph_modulus, &plain);
    let out = decrypt(&key.private(), cipher);
    let img = decode(out, &encoder);

    let mut img: Vec<u8> = img.iter().map(|x| (x.re * 255.) as u8).collect();
    img.truncate(28 * 28);

    // Check if the decoded image is within +-1 pixel value of original image
    for i in 0..28 * 28 {
        assert!((four_img[i] as i32 - img[i] as i32).abs() <= 1);
    }
}

#[test]
fn mnist_crt() {
    let four_img: [u8; 28 * 28] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 232, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 81, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 180, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 163, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 153, 210, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 163, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 254, 162, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 163, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 254, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 245, 163,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 198, 254, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 120, 254,
        163, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 231, 254, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159,
        254, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 163, 254, 216, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        159, 254, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 86, 178, 248, 254, 91, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 159, 254, 85, 0, 0, 0, 47, 49, 116, 144, 150, 241, 243, 234, 179, 241, 252, 40, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 150, 253, 237, 207, 207, 207, 253, 254, 250, 240, 198, 143, 91, 28,
        5, 233, 250, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 177, 177, 177, 177, 177, 98, 56, 0,
        0, 0, 0, 0, 102, 254, 220, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 169, 254, 137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 169, 254, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 169, 254, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 169, 255, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 169, 254, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        169, 254, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        169, 255, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        96, 254, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let mut four_img_f64 = [0f64; 28 * 28];

    for i in 0..28 * 28 {
        four_img_f64[i] = four_img[i] as f64 / 255f64;
    }

    let log_poly_degree = 11;
    let poly_degree = 1 << log_poly_degree;

    let log_modulus = 600;
    let ciph_modulus = 1.to_bigint().unwrap() << log_modulus;

    let prime_size = 30;

    let num_primes = (2 + log_poly_degree + 4 * log_modulus + prime_size - 1) / prime_size;
    let crt = Crt::new(num_primes, prime_size, poly_degree);

    let scaling_factor = 1_usize << 30;

    let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize).add_crt(&crt);
    let encoder = encoder::CKKSEncoder::new(poly_degree as usize * 2);

    let padded_message: [f64; 1024] = {
        let mut whole: [f64; 1024] = [0.; 1024];
        let (one, two) = whole.split_at_mut(28 * 28);
        one.copy_from_slice(&four_img_f64);
        two.copy_from_slice(&[0.; 240]);
        whole
    };
    let plain = encode(&padded_message, scaling_factor, &encoder);
    let cipher = encrypt(&key.public(), &ciph_modulus, &plain);
    let out = decrypt(&key.private(), cipher);
    let img = decode(out, &encoder);

    let mut img: Vec<u8> = img.iter().map(|x| (x.re * 255.) as u8).collect();
    img.truncate(28 * 28);

    // Check if the decoded image is within +-1 pixel value of original image
    for i in 0..28 * 28 {
        assert!((four_img[i] as i32 - img[i] as i32).abs() <= 1);
    }
}
