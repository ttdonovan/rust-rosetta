// http://rosettacode.org/wiki/Modular_inverse

#[cfg(not(test))]
fn main() {
    println!("{:?}", mul_inv(42, 2017));
}

fn mul_inv(a: int, b: int) -> Option<int> {
    let (gcd, mut x, _) = egcd(a, b);
    if gcd != 1 { // No multiplicative inverse exists
        return None;
    }
    if x < 0 {
        x += b;
    }
    Some(x % b)
}

fn egcd(a: int, b: int) -> (int, int, int) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (g, y, x) = egcd(b % a, a);
    (g, x - (b / a) * y, y)
}

#[test]
fn test() {
    assert_eq!(mul_inv(42, 2017), Some(1969));
}
