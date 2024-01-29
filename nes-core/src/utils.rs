pub fn concat(low: u8, high: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn split(val: u16) -> (u8, u8) {
    (val as u8, (val >> 8) as u8)
}

pub fn increment(val: u8) -> u8 {
    val.wrapping_add(1)
}

pub fn decrement(val: u8) -> u8 {
    val.wrapping_sub(1)
}

pub fn add_with_carry(a: u8, b: u8, carry: bool) -> u8 {
    a.wrapping_add(b).wrapping_add(if carry { 1 } else { 0 })
}

pub fn sub_with_carry(a: u8, b: u8, carry: bool) -> u8 {
    a.wrapping_sub(b).wrapping_sub(if carry { 1 } else { 0 })
}
