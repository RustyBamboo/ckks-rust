use algebra::crt::Crt;
use num_traits::cast::ToPrimitive;

fn main() {
    let num_primes = 3;
    let prime_size = 1 << 1;
    let poly_degree = 4;

    let crt = Crt::new(num_primes, prime_size, poly_degree);

    println!("{:?}", crt);

    let val = crt.reconstruct(vec![9, 17, 1]);
    println!("{}", val);

    let a_i = crt.crt(val.to_i128().unwrap());
    println!("a_i = {:?}", a_i);
}
