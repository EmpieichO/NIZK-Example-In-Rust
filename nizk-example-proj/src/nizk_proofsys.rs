use rand::Rng;
extern crate rand;


pub fn proving_k(b: u32, p: u32) -> (u32, u32, u32) {
    // select a secret_key as a random integer between 2 and 12
    let k: u32 = rand::thread_rng().gen_range(2..=23);
    println!("The secret_key k is: {}", k);
    
    let x = crate::power_arith::power_function(b, k, p);
    println!("The public_key x is: {}", x);
    let v: u32 = rand::thread_rng().gen_range(1..=11);
    println!("The randomly selected v is: {}", v);

    let s = crate::power_arith::power_function(b, v, p);
    println!("The s = b**v is: {}", s);

    let red_c = crate::hashing_func::hash_const(b, x, s, p);
    // println!("The reduced hash constant ver_c : {}", red_c);

    let skc = (k*red_c) % (p-1);
    println!("skc reduced mod p-1 : {:?}", skc);

    let r;

    if v < skc {
        r = p + v - skc -1;
        // println!("The r is : {}", r);
    } else {
        r = v - skc;
    }
    println!("The r is : {}", r);

    let proof_k = (s, r, x);
    println!("The proof of knowing k : {}, {}, {}", proof_k.0, proof_k.1, proof_k.2);

    return proof_k;
}


pub fn verifying_proof(b: u32, ver_x: u32, ver_s: u32, ver_r: u32, p: u32) -> u32 {

    let b_to_r = crate::power_arith::power_function(b, ver_r, p);
    println!(".. b_to_r is {}", b_to_r);

    let red_c = crate::hashing_func::hash_const(b, ver_x, ver_s, p);
    
    let x_to_c = crate::power_arith::power_function(ver_x, red_c, p);
    println!(".. x_to_c is {}", x_to_c);

    let fin_prod = (b_to_r * x_to_c) % p; 

    // let fin_prod = (u128::from(b_to_r) * u128::from(x_to_c)) % u128::from(p); 
    println!(".. fin_prod is {}", fin_prod);

    return fin_prod;
}
