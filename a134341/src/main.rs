fn main() {
    assert_eq!(
        27u64.pow(5) + 84u64.pow(5) + 110u64.pow(5) + 133u64.pow(5),
        144u64.pow(5),
    );

    let mut a = 1u64;
    'outer: loop {
        for b in 1..=a {
            for c in 1..=b {
                for d in 1..=c {
                    for e in 1..=d {
                        if a.pow(5) == b.pow(5) + c.pow(5) + d.pow(5) + e.pow(5) {
                            println!("{a}⁵ = {b}⁵ + {c}⁵ + {d}⁵ + {e}⁵");
                            break 'outer;
                        }
                    }
                }
            }
        }
        a += 1;
        println!("{a}");
    }
}
