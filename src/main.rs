use rsa_naive::rsa::{RSA, RSATrait};

fn main() {
    let musa = RSA::generate_key(2357,2551);
    println!("{}", musa);
    println!("[5,2,3,4,6,7,3]");
    let ct = musa.encrypt([5,2,3,4,6,7,3].to_vec());
    println!("{:?}", ct);
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