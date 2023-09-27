use rsa_naive::rsa::{RSATrait, RSA};

fn main() {
    let musa = RSA::generate_key();
    println!("key_generate {}", musa);
    println!("[5,2,3,4,6,7,3]");
    let ct = musa.encrypt([5, 2, 3, 4, 6, 7, 3].to_vec());
    println!("cypher text {:?}", ct);
    let mt = musa.decrypt(ct);
    println!("message text {:?}", mt);
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
