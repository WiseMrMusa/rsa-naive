use rsa_naive::{rsa::{RSATrait, RSA}, utils::{decode_string_adapter, encode_string_adapter}};

fn main() {
    let musa = RSA::generate_key();
    let message = "Hello Crypto World";
    println!("{}", message);
    let ct = musa.encrypt(encode_string_adapter(message));
    println!("{:?}", ct);
    let pt = musa.decrypt(ct);
    println!("{:?}", pt);
    println!("{}", decode_string_adapter(&pt));
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
