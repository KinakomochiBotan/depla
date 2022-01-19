#[inline]
pub const fn into_data(position: u32) -> u64 {
    1 << position
}

#[inline]
pub const fn count(data: u64) -> u32 {
    data.count_ones()
}

#[inline]
const fn shl(a: u64, b: u32) -> u64 {
    a << b
}

#[inline]
const fn shr(a: u64, b: u32) -> u64 {
    a >> b
}

macro_rules! line {
    ($r:ident, $p:expr, $m:expr, $s:ident, $n:expr) => {
        $r = $m & $s($p, $n);
        $r |= $m & $s($r, $n);
        $r |= $m & $s($r, $n);
        $r |= $m & $s($r, $n);
        $r |= $m & $s($r, $n);
        $r |= $m & $s($r, $n);
    }
}

#[inline]
pub const fn legal(board: (u64, u64)) -> u64 {

    macro_rules! calc {
        ($r:ident, $b:expr, $m:expr, $n:expr) => {
            let mask = $b.1 & $m;
            let mut buffer;
            line!(buffer, $b.0, mask, shl, $n);
            $r |= shl(buffer, $n);
            line!(buffer, $b.0, mask, shr, $n);
            $r |= shr(buffer, $n);
        };
    }

    let mut result = 0;
    calc!(result, board, 0x7e7e7e7e7e7e7e7e, 1);
    calc!(result, board, 0x007e7e7e7e7e7e00, 7);
    calc!(result, board, 0x00ffffffffffff00, 8);
    calc!(result, board, 0x007e7e7e7e7e7e00, 9);
    result & !(board.0 | board.1)
}

#[inline]
pub const fn reverse(board: (u64, u64), position: u64) -> u64 {

    macro_rules! calc {
        ($r:ident, $b:expr, $p:expr, $m:expr, $n:expr) => {
            let mask = $b.1 & $m;
            let mut buffer;
            line!(buffer, $p, mask, shl, $n);
            if $b.0 & shl(buffer, $n) != 0 { $r |= buffer; }
            line!(buffer, $p, mask, shr, $n);
            if $b.0 & shr(buffer, $n) != 0 { $r |= buffer; }
        };
    }

    let mut result = 0;
    calc!(result, board, position, 0x7e7e7e7e7e7e7e7e, 1);
    calc!(result, board, position, 0x007e7e7e7e7e7e00, 7);
    calc!(result, board, position, 0x00ffffffffffff00, 8);
    calc!(result, board, position, 0x007e7e7e7e7e7e00, 9);
    result
}

#[inline]
pub const fn make_move(board: (u64, u64), position: u64, reverse: u64) -> (u64, u64) {
    (board.0 ^ position ^ reverse, board.1 ^ reverse)
}
