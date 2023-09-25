use rand::{self, Rng};

// use rsa::naive;

#[derive(Debug)]
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
    phi: u128,
}

pub trait RSATrait {
    fn generate_key(p: u128, q: u128) -> RSA;
    fn encrypt(self: Self, m: Vec<u8>) -> Vec<u128>;
    fn decrypt(self: Self, c: Vec<u128>) -> Vec<u8>;
}

impl RSATrait for RSA {
    fn generate_key(p: u128, q: u128) -> RSA {
        let p: u128 = p;
        let q: u128 = q;
        let n = p * q;
        let phi = (p - 1) * (q - 1);
        // let e = rand::thread_rng().gen_range(0..phi);
        let e = 3674911;
        assert!(gcd(e, phi) == 1);
        let d = inversion(e.try_into().unwrap(), phi.try_into().unwrap());
        RSA { public_key: PublicKey { n, e }, private_key: PrivateKey { d: d.try_into().unwrap(), phi } }
    }
    fn encrypt(self: Self, m: Vec<u8>) -> Vec<u128> {
        let c: Vec<u128> = m.iter().map(|x| {
            let x = FieldPoint::new(*x as u128 , self.public_key.n);
            let m = x.power(self.public_key.e);
            m.num
        }).collect();
        c
    }
    fn decrypt(self: Self, c: Vec<u128>) -> Vec<u8> {
        let m = c.iter().map(|x| {
            let x = FieldPoint::new(*x, self.public_key.n);
            let m = x.power(self.private_key.d);
            m.num as u8
        }).collect();
        m
    }
}


use std::ops::{ Add, Sub, Mul};

pub struct FieldPoint {
    num: u128,
    prime: u128,
}

impl FieldPoint {
    pub fn new(num: u128, prime: u128) -> FieldPoint {
        if num > prime {
            panic!("Not a valid input");
        } else {
            FieldPoint { num, prime }
        }
    }

    pub fn power(self: &Self, index: u128) -> Self {
        if index == 0 {
            FieldPoint { num: 1u128, prime: self.prime }
        } else {
            let mut aux = index.rem_euclid(self.prime - 1);
            let mut acc = 1u128;
            let mut base = self.num;
            while aux > 0 {
                if aux % 2 == 0 {
                    base = (base * base).rem_euclid(self.prime);
                    aux = aux / 2u128;
                } else {
                    acc = (acc * base).rem_euclid(self.prime);
                    aux = aux - 1u128;
                }
            }
            FieldPoint { num: acc, prime: self.prime }
        }
    }
}

impl Add for FieldPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.prime == other.prime {
            FieldPoint {
                num: (self.num + other.num).rem_euclid(self.prime),
                prime: self.prime,
            }
        } else {
            panic!("Cannot add these field points, different prime values");
        }
    }
}

impl Sub for FieldPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.prime == other.prime {
            let num = if self.num > other.num {
                self.num - other.num
            } else {
                self.num - other.num + self.prime
            };
            FieldPoint {
                num,
                prime: self.prime,
            }
        } else {
            panic!("Cannot add these field points, different prime values");
        }
    }
}

impl Mul for FieldPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.prime == other.prime {
            FieldPoint {
                num: (self.num * other.num).rem_euclid(self.prime),
                prime: self.prime,
            }
        } else {
            panic!("Cannot add these field points, different prime values");
        }
    }
}

pub fn gcd(a: u128, b: u128) -> u128 {
    if a == 0 && b == 0 {
        panic!("Invalid");
    } else {
        if b == 0 {
            a
        } else {
            gcd(b, a%b)
        }
    }
}

pub fn lambda(p: u128, q: u128) -> u128 {
    let greatest_div: u128 = gcd(p-1, q-1);
    (p - 1)*(q - 1) / greatest_div
}

pub fn inversion(a: i128, b: i128) -> i128 {
    let mut t = 0i128;
    let mut r = b;
    let mut t1 = 1i128;
    let mut r1 = a;
    while r1 != 0i128 {
        let q = r.div_euclid(r1);
        (t,t1) = (t1, t-q*t1);
        (r, r1) = (r1, r - q*r1);
    }
    if r != 1i128 {
        return 0i128;
    } if t < 0 {
        t = t + b;
    }
    t
}

pub fn decompose(n: u128) -> (u128, u128) {
    let mut d = n - 1;
    let mut r = 0u128;
    while d%2 == 0 {
        d /= 2;
        r += 1;
    }
    (d,r)
}

pub fn miller_rabin(a: u128, n: u128, d: u128, r: u128) -> bool {
    let n_minus_one = n - 1u128;
    let field = FieldPoint::new(a,n);
    let mut x = field.power(d);
    let mut count = 1u128;
    if x.num == 1 || x.num == n_minus_one {
        return true;
    } while count < 1 {
        x = x.power(2u128);
        if x.num == n_minus_one {
            return  true;
        }
        count += 1u128;
    }
    true
}