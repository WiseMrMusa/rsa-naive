use crate::rsa::RSATrait;
pub mod rsa;

fn main() {
    let musa = rsa::RSA::generate_key(2357,2551);
    println!("{:?}", musa);
    println!("[5,2,3,4,6,7,3]");
    let ct = musa.encrypt([5,2,3,4,6,7,3].to_vec());
    println!("{:?}", ct);
    let musa = rsa::RSA::generate_key(2357,2551);
    let mt = musa.decrypt(ct);
    println!("{:?}", mt);
}


// #[cfg(test)]

// #[test]
// fn test_gcd() {
//     assert!(gcd(12, 8) == 4);
// }

// #[test]
// fn test_inverse_3mod5() {
//     assert!(inversion(3, 5) == 2);
// }