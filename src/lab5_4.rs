use std::io;

use crate::lab5_3::{gcd, generate_prime, mod_inv, mod_pow};

pub fn cipher_rsa_for_string(){
    // let mut input_line: String = get_string_from_line();
    let input_line = "Цікаво воно спарює з і та е.".to_string();
    //let input_line = "Cryptography is fun and educational! Learning RSA encryption and decryption with Python helps understand public key cryptosystems".to_string();
    let input_byte_array: &[u8] = input_line.as_bytes();

    let p = generate_prime();/*3557;*/
    let q = generate_prime();/*2579;*/
    let n: u64 = p as u64 * q as u64;
    let euler_f = (p - 1) as u64 * (q - 1) as u64;

    let mut e: u64 = 3;
    while gcd(e, euler_f) != 1 {
        e += 2;
    }
    let d_i64 = mod_inv(e as i128, euler_f as i128).expect("");
    let d: u64 = ((d_i64 % euler_f as i128 + euler_f as i128) % euler_f as i128) as u64;
    let mut c: Vec<i128> = vec![];
    
    // let open_key = [e, n];
    // let secret_key = [d, n];

    for iter in input_byte_array {
        let crypt_byte: i128 = mod_pow(*iter as i128, e as i128, n as i128);
        c.push(crypt_byte);   
    }
    println!("{:?}",c);

    let mut uncrypt_array: Vec<u8> = vec![];
    for iter in c {
        let uncrypt_byte: u8 = mod_pow(iter as i128, d as i128, n as i128) as u8;
        uncrypt_array.push(uncrypt_byte);
    }
    println!("{}",String::from_utf8(uncrypt_array).unwrap());
}

fn get_string_from_line() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    return input.trim().parse().expect("");
}