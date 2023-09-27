use std::ops::{ Add, Sub, Mul};

pub struct FieldPoint {
    pub num: u128,
    pub prime: u128,
}

impl FieldPoint {
    pub fn new(num: u128, prime: u128) -> FieldPoint {
        if num >= prime {
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

// #[cfg(test)]

// #[test]
// fn test_power(){
//     let x = FieldPoint::new(2, 7);
//     assert!(x.power(2) == FieldPoint::new(4, 7) )
// }
