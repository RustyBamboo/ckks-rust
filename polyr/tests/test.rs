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
fn rlwe() {
    let n = 2 * 2; // n = 2^k = len(a)
    let q = 13; // q = 1 mod 2n

    let a = polynomial_ring!(q, n, (10, 11, 1, 4));
    let s = polynomial_ring!(q, n, (11, 11, 9, 6));
    let e = polynomial_ring!(q, n, (1, 1, -1, 0));

    let c = a * s + e;

    assert_eq!(polynomial_ring!(q, n, (5, 8, 2, 6)), c);
}

#[test]
fn overflow_add() {
    let n = 4;
    let q = 13;

    let a = polynomial_ring!(q, n, (i32::MAX, i32::MAX));
    let b = polynomial_ring!(q, n, (1, 2));

    let c = a + b;

    assert_eq!(polynomial_ring!(q, n, (11, 12)), c);
}

#[test]
fn overflow_mul() {
    let n = 4;
    let q = 13;

    let a = polynomial_ring!(q, n, (i32::MAX, i32::MAX));
    let b = polynomial_ring!(q, n, (2, 2));

    let c = a * b;

    assert_eq!(polynomial_ring!(q, n, (7, 1, 7)), c);
}

#[allow(unused_variables)]
#[test]
fn rand() {
    let n = 4;
    let q = 13;

    let a = PolynomialRing::rand_binary(q, n, 4);
    let b = PolynomialRing::rand_uniform(q, n, 4);
    let c = PolynomialRing::rand_normal(q, n, 4);
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}
