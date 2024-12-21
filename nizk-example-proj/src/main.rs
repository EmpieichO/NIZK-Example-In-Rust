use std::cmp::Ordering;

mod power_arith;
mod hashing_func;
mod nizk_proofsys;

use std::arch::asm;

/// Function to get the current CPU cycle count using the RDTSC instruction
fn get_cpu_cycles() -> u64 {
    let high: u32;
    let low: u32;
    unsafe {
        // Read the time-stamp counter into two 32-bit registers
        asm!(
            "rdtsc",
            out("eax") low,   // Lower 32 bits go into `low`
            out("edx") high   // Higher 32 bits go into `high`
        );
    }
    // Combine the high and low parts to get the full 64-bit counter
    ((high as u64) << 32) | (low as u64)
}

fn main() {
    // Measure CPU cycles taken for a sample code block
    let start = get_cpu_cycles();

    use std::time::Instant;
    let now = Instant::now();
    let b: u32 = 7;
    let p: u32 = 71;
    println!("Using prime: {}", p);
    println!("Using primitive element b: {}", b);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);

    let proof_k = nizk_proofsys::proving_k(b, p);

    let ver_s = proof_k.0;
    let ver_r = proof_k.1;
    let ver_x = proof_k.2;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);

    println!("1st proof element s : {}", ver_s);
    println!("2nd proof element r : {}", ver_r);
    println!("3rd proof element x : {}", ver_x);

    let fin_prod = nizk_proofsys::verifying_proof(b, ver_x, ver_s, ver_r, p);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);

    match ver_s.cmp(&fin_prod) {
        Ordering::Less => println!("Verification Failed!!!"),
        Ordering::Greater => println!("Verification Failed!!!!"),
        Ordering::Equal => println!("*** .. NIZK .. Successfully Verified *** :-) \n"),
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);



    let end = get_cpu_cycles();
    // println!("The sum is: {}", sum);
    println!("CPU cycles taken: {}\n", end - start);
}
