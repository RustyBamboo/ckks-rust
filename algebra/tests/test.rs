use algebra::ntt::*;

#[test]
fn ntt_test() {
    let poly_degree = 4;
    let coeff_modulus = 73;
    let ntt = Ntt::new(poly_degree, coeff_modulus);

    let input: Vec<i128> = vec![0, 1, 4, 5];

    let fwd = ntt.ntt(&input, &ntt.roots_of_unity);

    assert_eq!(vec![10, 34, 71, 31], fwd);
}

#[test]
fn ntt_inv_test() {
    let poly_degree = 4;
    let coeff_modulus = 73;
    let ntt = Ntt::new(poly_degree, coeff_modulus);

    let input: Vec<i128> = vec![10, 34, 71, 31]
        .iter()
        .map(|x| (x * -18i128).rem_euclid(73))
        .collect();

    let inv = ntt.ntt(&input, &ntt.roots_of_unity_inv);

    assert_eq!(vec![0, 1, 4, 5], inv);
}
