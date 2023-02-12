use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    assert_eq!(
        27u64.pow(5) + 84u64.pow(5) + 110u64.pow(5) + 133u64.pow(5),
        144u64.pow(5),
    );

    let mut a = 1u64;
    let mut b5_plus_c5 = HashMap::new();

    loop {
        let a5 = a.pow(5);

        let b = a;
        let b5 = b.pow(5);
        for c in 1..=b {
            let c5 = c.pow(5);
            b5_plus_c5.insert(b5 + c5, (b, c));
        }

        (1..a).into_par_iter().find_any(|&d| {
            let d5 = d.pow(5);
            for e in 1..=d {
                let e5 = e.pow(5);
                let a5_minus_d5_plus_e5 = a5 - (d5 + e5);
                if let Some((b, c)) = b5_plus_c5.get(&a5_minus_d5_plus_e5) {
                    println!("{a}⁵ = {b}⁵ + {c}⁵ + {d}⁵ + {e}⁵");
                    return true;
                }
            }
            false
        });

        a += 1;
    }
}
