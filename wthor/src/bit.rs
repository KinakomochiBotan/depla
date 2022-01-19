#[inline]
pub fn flip_vertical(data: u64) -> u64 {
    data.swap_bytes()
}

#[inline]
pub fn rotate180(data: u64) -> u64 {
    data.reverse_bits()
}

#[inline]
pub fn flip_diagonal(data: u64) -> u64 {

    macro_rules! calc {
        ($r:ident, $m:expr, $n:expr) => {
            let mask = $m & ($r ^ ($r << $n));
            $r ^= mask ^ (mask >> $n);
        };
    }

    let mut result = data;
    calc!(result, 0x0f0f0f0f00000000, 28);
    calc!(result, 0x3333000033330000, 14);
    calc!(result, 0x5500550055005500, 07);
    result
}
