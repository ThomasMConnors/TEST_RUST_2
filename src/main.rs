// https://code.visualstudio.com/docs/languages/rust

fn main() {
    println!("Hello, world!");
    println!("{}", gcd(100, 200));
    println!("{}", gcd(480, 240));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t: u64;

            t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd_1s() {
    assert_eq!(gcd(1, 1), 1);
}

#[test]
fn test_gcd_samples() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

#[test]
#[should_panic]
fn test_gcd_panic() {
    gcd(0, 0);
    // let result: Result<u64, Box<dyn std::any::Any + Send>> = std::panic::catch_unwind(|| gcd(0, 0));
    // assert!(result.is_err()); //probe further for specific error type here, if desired
}
