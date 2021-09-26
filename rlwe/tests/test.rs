use rlwe::*;

#[test]
fn encrypt_decrypt() {
    let n = 2_i32.pow(4);
    let q = 2_i32.pow(15);
    let t = 2_i32.pow(8);

    // Create a keypair 100 times
    for _ in 0..100 {
        // Create key pair
        let key = Rwle::keygen(q, n as usize, n as usize);

        // Create our data with a single datapoint
        let x = 73_i32;
        let mut data = vec![0; n as usize];
        data[0] = x.rem_euclid(t);

        // Encrypt our data using keypair
        let cipher1 = encrypt(&key.public(), n as usize, q, t, n as usize, data);

        // Decrypt our data
        let out = decrypt(&key.private(), q, t, cipher1);

        assert_eq!(x, out.coef[0]);
    }
}
