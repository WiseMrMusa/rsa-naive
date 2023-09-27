use std::fmt::Display;

use crate::field_point::FieldPoint;
use crate::utils::{gcd, generate_random_prime, inversion};

pub struct RSA {
    pub public_key: PublicKey,
    private_key: PrivateKey,
}

#[derive(Debug)]
pub struct PublicKey {
    pub n: u128,
    pub e: u128,
}

#[derive(Debug)]
struct PrivateKey {
    d: u128,
    _phi: u128,
}

pub trait RSATrait {
    fn generate_key() -> RSA;
    fn encrypt(self: &Self, m: Vec<u8>) -> Vec<u128>;
    fn decrypt(self: &Self, c: Vec<u128>) -> Vec<u8>;
}

impl RSATrait for RSA {
    fn generate_key() -> RSA {
        let p: u128 = generate_random_prime();
        let q: u128 = generate_random_prime();
        let n = p * q;

        // using miller -rabin algorithm determine if result and result2 are prime number.

        // totient
        let phi = (p - 1) * (q - 1);

        // let e = rand::thread_rng().gen_range(0..phi);
        // what does this e stand for?
        // let e = 3674911;
        let e = generate_random_prime();
        assert!(gcd(e, phi) == 1);
        let d = inversion(e.try_into().unwrap(), phi.try_into().unwrap());
        println!("The public key is {} and N is {}", e, n);
        RSA {
            public_key: PublicKey { n, e },
            private_key: PrivateKey {
                d: d.try_into().unwrap(),
                _phi: phi,
            },
        }
    }

    fn encrypt(self: &Self, m: Vec<u8>) -> Vec<u128> {
        let c: Vec<u128> = m
            .iter()
            .map(|x| {
                let x = FieldPoint::new(*x as u128, self.public_key.n);
                let m = x.power(self.public_key.e);
                m.num
            })
            .collect();
        c
    }
    fn decrypt(self: &Self, c: Vec<u128>) -> Vec<u8> {
        let m = c
            .iter()
            .map(|x| {
                let x = FieldPoint::new(*x, self.public_key.n);
                let m = x.power(self.private_key.d);
                m.num as u8
            })
            .collect();
        m
    }
}

impl Display for RSA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "The public key is {} in Field {}",
            self.public_key.e, self.public_key.n
        )
    }
}
