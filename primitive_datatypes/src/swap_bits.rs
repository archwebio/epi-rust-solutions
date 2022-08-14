/// Swap bits of a given number
/// Operates at O(1)
/// # Examples
///
/// ```rs
/// // 12: 1 1 0 0
/// // swap  ^ ^   bits: 1 and 2
/// // 10: 1 0 1 0
/// let swapped = 10
/// let answer = swap_bits::swap(12, 1, 2);
///
/// assert_eq!(answer, swapped);
/// ```
pub fn swap(n: i32, i: i32, j: i32) -> i32 {
    if (&n >> i) & 1 != (&n >> j) & 1 {
        let mask = (1 << i) | (1 << j);
        return n ^ mask;
    }

    return n;
}

#[test]
fn swap_test() {
    assert_eq!(swap(12, 1, 0), 12);
    assert_eq!(swap(12, 1, 2), 10);
    assert_eq!(swap(12, 3, 2), 12);
}
