#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate core;

use min_max::*;

fn output_gcd(n: u64, m: u64) {
    let d = gcd(n, m);
    println!("gcd({n}, {m}) = {d}")
}

fn main() {
    output_gcd(25, 15)
}

// Functional version of gcd.
fn gcd(n: u64, m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    fn aux((n, m): (u64, u64)) -> u64 {
        match n {
            0 => m,
            _ => aux(min_max!(n, m % n))
        }
    }
    aux((m, n))
}

// Unit testing built into the language.
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15 ), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    assert_eq!(gcd(11 * 13 * 13 * 17 * 19 * 23, 2 * 5 * 13 * 17), 13 * 17);
}

// Property based testing with quickcheck.
#[cfg(test)]
mod tests {
    use quickcheck::TestResult;
    use crate::gcd;

    // Loop-based implementation of gcd for comparison.
    fn gcd_loop(mut n: u64, mut m: u64) -> u64 {
        assert!(m != 0 && n != 0);
        while m != 0 {
            if m < n {
                (m, n) = (n, m);
            }
            m = m % n;
        }
        n
    }

    quickcheck! {
        fn prop(x: u64, y: u64) -> TestResult {
            // Doesn't make sense to calculate gcd(0, x).
            match (x, y) {
                (0, _) | (_, 0) => TestResult::discard(),
                _ => TestResult::from_bool(gcd(x, y) == gcd_loop(x, y))
            }
        }
    }
}
