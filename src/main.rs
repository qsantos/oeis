use std::io::prelude::*;
use std::io::stdout;

use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;

fn is_counter_example(n: u128) -> bool {
    for p in [7, 11, 13, 17, 31, 37, 41, 43, 59, 61, 67, 73, 79] {
        if n % p == 0 {
            return false;
        }
    }
    let mut r = 1;
    let mut p = n;
    let mut acc = 2;
    while p > 0 {
        if p % 2 != 0 {
            r *= acc;
            r %= n;
        }
        acc = (acc * acc) % n;
        p /= 2;
    }
    r == 3
}

fn main() {
    let solution = 4_700_063_497;
    assert!(!is_counter_example(5));
    assert!(is_counter_example(solution));

    let mut chunk_start = 0;
    let chunk_size = 1_000_000;
    let n = loop {
        let chunk_end = chunk_start + chunk_size;
        if let Some(n) = (chunk_start..chunk_end)
            .into_par_iter()
            .find_first(|&n| is_counter_example(1 + 4 * n))
        {
            break 1 + 4 * n;
        }
        print!(
            "\r{:6.2} %",
            ((1 + 4 * chunk_end) as f32) / (solution as f32) * 100.
        );
        stdout().flush().expect("Could not flush");
        chunk_start = chunk_end;
    };
    println!();
    println!("{n} is a counter example");
    assert!(is_counter_example(n));
}
