use algebra::ntt::*;
use num_bigint::{BigInt, ToBigInt};

#[test]
fn ntt_test() {
    let poly_degree = 4;
    let coeff_modulus = 73;
    let ntt = Ntt::new(poly_degree, coeff_modulus);

    let input: Vec<BigInt> = vec![0, 1, 4, 5]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();

    let fwd = ntt.ntt(&input, &ntt.roots_of_unity);

    let expected: Vec<BigInt> = vec![10, 34, 71, 31]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();

    assert_eq!(expected, fwd);
}

#[test]
fn ntt_inv_test() {
    let poly_degree = 4;
    let coeff_modulus = 73;
    let ntt = Ntt::new(poly_degree, coeff_modulus);

    let input: Vec<BigInt> = vec![10, 34, 71, 31]
        .iter()
        .map(|x| (x * -18i128).rem_euclid(73).to_bigint().unwrap())
        .collect();

    let inv = ntt.ntt(&input, &ntt.roots_of_unity_inv);

    let expected: Vec<BigInt> = vec![0, 1, 4, 5]
        .iter()
        .map(|x| x.to_bigint().unwrap())
        .collect();
    assert_eq!(expected, inv);
}
