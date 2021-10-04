pub use polyr::Modulo;
pub use polyr::{polynomial, Polynomial};
pub use polyr::{polynomial_ring, PolynomialRing};

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
fn sub() {
    let a = polynomial_ring!(4, (1, 1, 1, 1));
    let b = polynomial_ring!(4, (0, 0, 0));
    let c = (a - b) % 13;
    assert_eq!(polynomial_ring!(4, (1, 1, 1, 1)), c);
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
    let q = 13;

    let x = [1, 6, 7, 13, 14, 26, 27, -26, -14, -13, -7, -6, -1];
    let y: Vec<i32> = x.iter().map(|&a| a.mod_ring(q)).collect();
    assert_eq!(vec![1, 6, -6, 0, 1, 0, 1, 0, -1, 0, 6, -6, -1], y);
}

#[test]
fn rlwe() {
    let n = 2 * 2; // n = 2^k = len(a)
    let q = 13; // q = 1 mod 2n

    let a = polynomial_ring!(n, (10, 11, 1, 4)) % q;
    let s = polynomial_ring!(n, (11, 11, 9, 6)) % q;
    let e = polynomial_ring!(n, (1, 1, -1, 0)) % q;

    let c = ((a * s) % q + e) % q;

    assert_eq!(polynomial_ring!(n, (5, -5, 2, 6)), c);
}

#[test]
fn overflow_add() {
    let n = 4;
    let q = 13;

    let a = polynomial_ring!(n, (i32::MAX, i32::MAX)) % q;
    let b = polynomial_ring!(n, (1, 2));

    let c = (a + b) % q;

    assert_eq!(polynomial_ring!(n, (-2, -1)), c);
}

#[test]
fn overflow_mul() {
    let n = 4;
    let q = 13;

    let a = polynomial_ring!(n, (i32::MAX, i32::MAX)) % q;
    let b = polynomial_ring!(n, (2, 2)) % q;

    let c = (a * b) % q;

    assert_eq!(polynomial_ring!(n, (-6, 1, -6)), c);
}

#[allow(unused_variables)]
#[test]
fn rand() {
    let n = 4;
    let q = 13;

    let a = PolynomialRing::rand_binary(n, 4);
    let b = PolynomialRing::rand_uniform(q, n, 4);
    let c = PolynomialRing::rand_normal(n, 4);
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}
