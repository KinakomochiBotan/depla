#[inline]
pub fn flip_vertical(n: u64) -> u64 {
    n.swap_bytes()
}

#[inline]
pub fn flip_diagonal(n: u64) -> u64 {
    [
        (28, 0x0f0f0f0f00000000),
        (14, 0x3333000033330000),
        (07, 0x5500550055005500)
    ].into_iter().fold(n, |value, (shifts, mask)| {
        let mask = mask & (value ^ (value << shifts));
        value ^ (mask ^ (mask >> shifts))
    })
}

#[inline]
pub fn reverse(n: u64) -> u64 {
    n.reverse_bits()
}
