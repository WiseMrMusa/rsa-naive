use crate::field_point::FieldPoint;
use primal::is_prime;
use rand::prelude::*;

pub fn gcd(a: u128, b: u128) -> u128 {
    if a == 0 && b == 0 {
        panic!("Invalid");
    } else {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
}

pub fn lambda(p: u128, q: u128) -> u128 {
    let greatest_div: u128 = gcd(p - 1, q - 1);
    (p - 1) * (q - 1) / greatest_div
}

pub fn inversion(a: i128, b: i128) -> i128 {
    let mut t = 0i128;
    let mut r = b;
    let mut t1 = 1i128;
    let mut r1 = a;
    while r1 != 0i128 {
        let q = r.div_euclid(r1);
        (t, t1) = (t1, t - q * t1);
        (r, r1) = (r1, r - q * r1);
    }
    if r != 1i128 {
        return 0i128;
    }
    if t < 0 {
        t = t + b;
    }
    t
}

pub fn decompose(n: u128) -> (u128, u128) {
    let mut d = n - 1;
    let mut r = 0u128;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    (d, r)
}

pub fn miller_rabin(a: u128, n: u128, d: u128, r: u128) -> bool {
    let n_minus_one = n - 1u128;
    let field = FieldPoint::new(a, n);
    let mut x = field.power(d);
    let mut count = 1u128;
    if x.num == 1 || x.num == n_minus_one {
        return true;
    }
    while count < 1 {
        x = x.power(2u128);
        if x.num == n_minus_one {
            return true;
        }
        count += 1u128;
    }
    true
}

pub fn generate_random_prime() -> u128 {
    let mut rng = rand::thread_rng();

    loop {
        // Generate a random number between 1000 and 10000 (adjust as needed)
        let candidate: u128 = rng.gen_range(1000..10000);
        // Ensure the number is odd
        let candidate = if candidate % 2 == 0 {
            candidate + 1
        } else {
            candidate
        };
        let candidate_usize = candidate as usize;
        // Check if the number is prime
        if is_prime(candidate_usize.try_into().unwrap()) {
            return candidate;
        }
    }
}

pub fn encode_string_adapter(m: &str) -> Vec<u8> {
    m.chars().map(|x| x.try_into().unwrap()).collect()
}

pub fn decode_string_adapter(c: &Vec<u8>) -> String {
    c.iter().map(|x| *x as char).collect()
}
