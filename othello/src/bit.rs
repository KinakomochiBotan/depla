#[inline]
pub const fn into_data(position: u32) -> u64 {
    1 << position
}

#[inline]
pub const fn flip_bit(data: u64, position: u32) -> u64 {
    data ^ into_data(position)
}

#[inline]
pub const fn is_set(data: u64, position: u32) -> bool {
    data & into_data(position) != 0
}

#[inline]
pub const fn count_bit(data: u64) -> u32 {
    data.count_ones()
}

#[inline]
pub const fn check_board(board: (u64, u64)) -> bool {
    board.0 & board.1 == 0
}

#[inline]
pub const fn calc_legal(board: (u64, u64)) -> u64 {
    let mut result = 0;

    macro_rules! calc {
        ($n:literal, $m:literal) => {
            let mask = $m & board.1;
            let mut buffer = mask & (board.0 << $n);
            buffer |= mask & (buffer << $n);
            buffer |= mask & (buffer << $n);
            buffer |= mask & (buffer << $n);
            buffer |= mask & (buffer << $n);
            buffer |= mask & (buffer << $n);
            result |= buffer << $n;
            let mut buffer = mask & (board.0 >> $n);
            buffer |= mask & (buffer >> $n);
            buffer |= mask & (buffer >> $n);
            buffer |= mask & (buffer >> $n);
            buffer |= mask & (buffer >> $n);
            buffer |= mask & (buffer >> $n);
            result |= buffer >> $n;
        };
    }

    calc!(1, 0x7e7e7e7e7e7e7e7e);
    calc!(7, 0x007e7e7e7e7e7e00);
    calc!(8, 0x00ffffffffffff00);
    calc!(9, 0x007e7e7e7e7e7e00);
    result & !(board.0 | board.1)
}

#[inline]
pub const fn make_move(board: (u64, u64), position: u32) -> (u64, u64) {
    let position = into_data(position);
    let mut result = 0;

    macro_rules! calc {
        ($n:literal, $m:literal) => {
            let mask = $m & board.1;
            let mut buffer = mask & (position << $n);
            buffer |= mask & (buffer << $n);
            buffer |= mask & (buffer << $n);
            buffer |= mask & (buffer << $n);
            buffer |= mask & (buffer << $n);
            buffer |= mask & (buffer << $n);
            if board.0 & (buffer << $n) != 0 { result |= buffer; }
            let mut buffer = mask & (position >> $n);
            buffer |= mask & (buffer >> $n);
            buffer |= mask & (buffer >> $n);
            buffer |= mask & (buffer >> $n);
            buffer |= mask & (buffer >> $n);
            buffer |= mask & (buffer >> $n);
            if board.0 & (buffer >> $n) != 0 { result |= buffer; }
        };
    }

    calc!(1, 0x7e7e7e7e7e7e7e7e);
    calc!(7, 0x007e7e7e7e7e7e00);
    calc!(8, 0x00ffffffffffff00);
    calc!(9, 0x007e7e7e7e7e7e00);
    (board.0 ^ position ^ result, board.1 ^ result)
}
