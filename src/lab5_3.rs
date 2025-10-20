use rand::{thread_rng,Rng};

pub fn cipher_rsa_for_num(){
    let p = generate_prime() as u64;/*3557;*/
    let q = generate_prime() as u64;/*2579;*/
    let n = p * q;
    let euler_f = (p - 1) * (q - 1);

    let mut e: u64 = 3;
    while gcd(e, euler_f) != 1 {
        e += 2;
    }

    let m: i128 = 122;

    let d_i128 = mod_inv(e as i128, euler_f as i128).expect("");
    let d: u64 = ((d_i128 % euler_f as i128 + euler_f as i128) % euler_f as i128) as u64;

    let open_key = [e, n];
    let secret_key = [d, n];

    let c = mod_pow(m, e as i128, n as i128);
    let enc = mod_pow(c, d as i128, n as i128);

    println!("p: {p} \t q: {q} \t n: {n}");
    println!("Euler's fn: {euler_f}\t d: {d}");
    println!("\nopen_key: {:?}\n secret_key: {:?}", open_key, secret_key);
    println!("crypt: {c} \t encrypt: {enc}")
}

pub fn generate_prime() -> u32 {
    let mut rng = thread_rng();
    let mut candidate:u32;
    loop {
        candidate = rng.gen_range(0..=u32::max_value()) | 1;
        if is_prime(candidate) {
            return candidate;
        }
    }
}

fn is_prime(candidate:u32) -> bool {
    if candidate <= 3 {
        return true;
    } else if candidate % 3 == 0 {
        return false;
    }else {
        let mut i: u32 = 5;
        while i * i <= candidate{
            if candidate % i == 0 || candidate % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
    }
    return true;
}

pub fn mod_inv(e: i128, d:i128) -> Option<i128>{
    let (mut a, mut b) = (e, d);
    let (mut x0, mut x1) = (1, 0);
    while b != 0 {
        let q = a / b;
        let (a_old, b_old) = (a, b);
        a = b_old;
        b = a_old - q * b_old;

        let (x0_old, x1_old) = (x0, x1);
        x0 = x1_old;
        x1 = x0_old - q * x1_old;
    }
    if a != 1{
        None
    }else {
        Some((x0 % d + d) % d)
    }
}

pub fn gcd(mut a: u64, mut b: u64) -> u64{
    while b != 0 {
        let t = b;
        b = a % b;
        a = t; 
    }
    a
}

pub fn mod_pow(mut base: i128, mut exp: i128, modu: i128) -> i128 {
    let mut result = 1;
    base = base % modu;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modu;
        }
        exp /= 2;
        base = (base * base) % modu;
    }
    result
}