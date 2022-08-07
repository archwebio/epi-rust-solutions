/// Calculates the parity for a number by iterating each bit
/// Operates at O(n)
/// # Examples
///
/// ```rs
/// let parity = 0
/// let answer = primitive_datatypes::parity1(3);
///
/// assert_eq!(answer, parity);
/// ```
pub fn parity1(n: i32) -> i32 {
    let mut parity = 0;
    let mut digit = n;

    while digit > 0 {
        parity ^= digit & 1;
        digit >>= 1;
    }

    parity
}

/// Calculates the parity for a number by iterating each bit
/// Operates at O(k), where k is the number of `1's` in a given number
/// # Examples
///
/// ```rs
/// let parity = 1
/// let answer = primitive_datatypes::parity2(4);
///
/// assert_eq!(answer, parity);
/// ```
pub fn parity2(n: i32) -> i32 {
    let mut parity = 0;
    let mut num = n;

    while num > 0 {
        parity ^= 1;
        num &= num - 1;
    }

    parity
}

#[test]
fn parity_1_test() {
    assert_eq!(parity1(1), 1);
    assert_eq!(parity1(2), 1);
    assert_eq!(parity1(3), 0);
    assert_eq!(parity1(4), 1);
    assert_eq!(parity1(5), 0);
    assert_eq!(parity1(7), 1);
    assert_eq!(parity1(15), 0);
}

#[test]
fn parity_2_test() {
    assert_eq!(parity2(1), 1);
    assert_eq!(parity2(2), 1);
    assert_eq!(parity2(3), 0);
    assert_eq!(parity2(4), 1);
    assert_eq!(parity2(5), 0);
    assert_eq!(parity2(7), 1);
    assert_eq!(parity2(15), 0);
}
