#![feature(bigint_helper_methods)]

use std::io::prelude::*;
use std::io::stdout;

fn is_counter_example(n: u64) -> bool {
    for p in [
        7, 11, 13, 17, 31, 37, 41, 43, 59, 61, 67, 73, 79, 83, 89, 103, 107, 109, 113, 127, 131,
        137,
    ] {
        if n % p == 0 {
            return false;
        }
    }
    let two_to_64_mod_n = (((1u64 << 63) % n) * 2) % n;
    let mut r = 1;
    let mut p = n;
    let mut acc = 2;
    while p > 0 {
        if p % 2 != 0 {
            r *= acc;
            r %= n;
        }
        // acc = (acc * acc) % n
        //
        // (acc * acc) % n = (high * 2**64 + low) % n
        //                 = ((high  % n) * (2**64 % n) + (low % n)) % n
        let (low, high) = acc.widening_mul(acc);
        acc = ((high % n) * two_to_64_mod_n + (low % n)) % n;

        p /= 2;
    }
    r == 3
}

fn main() {
    let solution = 4_700_063_497;
    assert!(!is_counter_example(5));
    assert!(is_counter_example(solution));

    let mut n = 1;
    while !is_counter_example(n) {
        if n % 1048576 == 1 {
            print!("\r{:6.2} %", (n as f32) / (solution as f32) * 100.);
            stdout().flush().expect("Could not flush");
        }
        n += 4;
    }
    println!();
    println!("{n} is a counter example");
    assert!(is_counter_example(n));
}
