pub use polyr::Modulo;
pub use polyr::{polynomial, Polynomial};
pub use polyr::{polynomial_ring, PolynomialRing};

use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};

#[test]
fn mod_cyc() {
    let mut poly = polynomial![0, 77, 7, 11, 12, 1];
    poly.mod_cyc(4);
    assert_eq!(polynomial![-12, 76, 7, 11], poly)
}

#[test]
fn add() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let c = a + b;
    assert_eq!(polynomial![7, 11, 2, 1], c);
}

#[test]
fn mul() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let c = a * b;
    assert_eq!(polynomial![0, 77, 7, 11, 12, 1], c);
}

#[test]
fn mul_crt() {
    use algebra::crt::Crt;

    let log_modulus = 10;
    let modulus = 1 << log_modulus;

    let prime_size = 30;

    let log_poly_degree = 2;
    let poly_degree = 1 << log_poly_degree;

    let num_primes = (2 + log_poly_degree + 4 * log_modulus + prime_size - 1) / prime_size;
    //let num_primes = 4;
    println!("Num of primes: {}", num_primes);

    let crt = Crt::new(num_primes, prime_size, poly_degree);

    let a: Vec<BigInt> = [0, 1, 4, 5]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    let b: Vec<BigInt> = [1, 2, 4, 3]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    let a_ex = PolynomialRing::new(poly_degree as usize, a.clone());
    let b_ex = PolynomialRing::new(poly_degree as usize, b.clone());

    let a = PolynomialRing::new_with_crt(poly_degree as usize, a, &crt);
    let b = PolynomialRing::new_with_crt(poly_degree as usize, b, &crt);

    let c_ex = &a_ex * &b_ex % &modulus.to_bigint().unwrap();

    let c = (&a * &b) % &modulus.to_bigint().unwrap();

    println!("{:?}", c_ex);
    println!("{:?}", c);
}

#[test]
fn sub() {
    let a = PolynomialRing::new(4, vec![One::one(); 4]);
    let b = PolynomialRing::new(4, vec![Zero::zero(); 3]);
    let c = (a - b) % &13.to_bigint().unwrap();
    assert_eq!(PolynomialRing::new(4, vec![One::one(); 4]), c);
}

#[test]
fn mul_mod_cyc() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let mut c = a * b;
    c.mod_cyc(4);
    assert_eq!(polynomial![-12, 76, 7, 11], c)
}

#[test]
fn add_field_mod_cyc() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let mut c = a + b;
    c.mod_cyc(4);
    c.rem_euclid(5);
    assert_eq!(polynomial![2, 1, 2, 1], c)
}

#[test]
fn mul_field_mod_cyc() {
    let a = polynomial![7, 0, 1, 1];
    let b = polynomial![0, 11, 1];
    let mut c = a * b;
    c.mod_cyc(4);
    c.rem_euclid(5);
    assert_eq!(polynomial![3, 1, 2, 1], c)
}

#[test]
fn test_modulo() {
    let q = 13.to_bigint().unwrap();

    let x = [1, 6, 7, 13, 14, 26, 27, -26, -14, -13, -7, -6, -1];
    let x: Vec<BigInt> = x.iter().map(|x| x.to_bigint().unwrap()).collect();
    let y: Vec<BigInt> = x.iter().map(|a| a.mod_ring(&q)).collect();

    let expected: Vec<BigInt> = [1, 6, -6, 0, 1, 0, 1, 0, -1, 0, 6, -6, -1]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    assert_eq!(expected, y);
}

#[test]
fn rlwe() {
    let n = 2 * 2; // n = 2^k = len(a)
    let q = 13.to_bigint().unwrap(); // q = 1 mod 2n

    let a = vec![10, 11, 1, 4]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    let s = vec![11, 11, 9, 6]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    let e = vec![1, 1, -1, 0]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();

    let a = PolynomialRing::new(n, a) % &q;
    let s = PolynomialRing::new(n, s) % &q;
    let e = PolynomialRing::new(n, e) % &q;

    let c = ((&a * &s) % &q + e) % &q;

    let expected: Vec<BigInt> = [5, -5, 2, 6]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    assert_eq!(PolynomialRing::new(n, expected), c);
}

#[test]
fn overflow_add() {
    let n = 4;
    let q = 13.to_bigint().unwrap();

    let max = i32::MAX.to_bigint().unwrap();

    let a = PolynomialRing::new(n, vec![max.clone(), max]) % &q;
    let b = PolynomialRing::new(n, vec![One::one(), 2.to_bigint().unwrap()]);

    let c = (a + b) % &q;

    assert_eq!(
        PolynomialRing::new(n, vec![-2.to_bigint().unwrap(), -1.to_bigint().unwrap()]),
        c
    );
}

#[test]
fn overflow_mul() {
    let n = 4;
    let q = 13.to_bigint().unwrap();

    let max = i32::MAX.to_bigint().unwrap();

    let a = PolynomialRing::new(n, vec![max.clone(), max]) % &q;
    let b = PolynomialRing::new(n, vec![2.to_bigint().unwrap(), 2.to_bigint().unwrap()]);

    let c = (&a * &b) % &q;

    assert_eq!(
        PolynomialRing::new(
            n,
            vec![
                -6.to_bigint().unwrap(),
                1.to_bigint().unwrap(),
                -6.to_bigint().unwrap()
            ]
        ),
        c
    );
}
