use num_traits::{One, PrimInt, Zero};
use std::ops::{AddAssign, BitAnd, DivAssign, ShrAssign, SubAssign};

///
/// Reverse bits of an integer with specified width.
///
/// E.g. Reverse 6 = 0b110 with a width of 5 would result in reversing 0b00110, which becomes
/// 0b01100 = 12
///
pub fn reverse_bits(value: usize, width: u32) -> usize {
    value.reverse_bits() >> (usize::BITS - width)
}

///
/// Reverse an array by reversing bits of indicides
///
pub fn bit_reverse_vec<T: Clone>(values: &Vec<T>) -> Vec<T> {
    let mut result = (*values).clone();
    let len = result.len();
    for i in 0..len / 2 {
        result.swap(i, reverse_bits(i, len.log2()));
    }
    result
}

///
/// Find a^b % p
///
pub fn powmod<T>(mut a: T, mut b: T, p: T) -> T
where
    T: PrimInt + One + Zero + SubAssign + ShrAssign + BitAnd<T, Output = T> + PartialEq,
{
    let _0 = T::zero();
    let _1 = T::one();
    let mut res = _1;
    while b != _0 {
        if (b & _1) != _0 {
            res = res * a % p;
            b -= _1;
        } else {
            a = a * a % p;
            b >>= _1;
        }
    }
    res
}

#[test]
fn powmod_test() {
    assert_eq!(powmod(3, 1, 7), 3);
    assert_eq!(powmod(3, 2, 7), 2);
    assert_eq!(powmod(3, 3, 7), 6);
    assert_eq!(powmod(3, 4, 7), 4);
    assert_eq!(powmod(3, 5, 7), 5);
    assert_eq!(powmod(3, 6, 7), 1);
    assert_eq!(powmod(3, 7, 7), 3);
}

///
/// Find the inverse in a given modulus
///
pub fn invmod<T>(a: T, p: T) -> T
where
    T: PrimInt + One + Zero + SubAssign + ShrAssign + BitAnd<T, Output = T>,
{
    let _1 = T::one();
    let _2 = _1 + _1;
    powmod(a, p - _2, p)
}

///
/// Find the generator of a cyclic group
///
/// Upon failure will return None
///
pub fn generator<T>(p: T) -> Option<T>
where
    T: PrimInt
        + Zero
        + One
        + PartialOrd
        + AddAssign
        + SubAssign
        + DivAssign
        + ShrAssign
        + BitAnd<T, Output = T>,
{
    let _0 = T::zero();
    let _1 = T::one();
    let _2 = _1 + _1;

    let mut fact = Vec::new();

    let phi = p - _1;
    let mut n = phi;

    // for(int i = 2; i * i <= n; ++i)
    let mut i = _2;
    while i * i <= n {
        if (n % i) == _0 {
            fact.push(i);
            while (n % i) == _0 {
                n /= i;
            }
        }

        i += _1;
    }

    if n > _1 {
        fact.push(n);
    }

    // for(int res = 2; res <=p; ++res)
    let mut res = _2;
    while res <= p {
        let mut ok = true;

        for f in fact.iter() {
            if !ok {
                break;
            }
            let check = powmod(res, phi / *f, p) != _1;
            ok &= check;

            if ok {
                return Some(res);
            }
        }

        res += _1;
    }
    None
}

#[test]
fn generator_test() {
    let primes = [5, 7, 11, 13, 17, 59, 73];

    for p in primes {
        let res = generator(p).unwrap();
        // If we got a generator, and given that p is prime, then the generator to the power of
        // (prime - 1) should give 1.
        assert_eq!(powmod(res, p - 1, p), 1);
    }
}

///
/// Finds a root of unity in a given modulus. The modulus must be PRIME.
///
pub fn root_of_unity<T>(order: T, modulus: T) -> T
where
    T: PrimInt
        + Zero
        + One
        + PartialOrd
        + AddAssign
        + SubAssign
        + DivAssign
        + ShrAssign
        + BitAnd<T, Output = T>,
{
    let _1 = T::one();
    let g = generator(modulus).unwrap();

    powmod(g, (modulus - _1) / order, modulus)
}
