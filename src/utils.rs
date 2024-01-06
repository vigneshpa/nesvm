pub fn concat(high:u8, low:u8) -> u16 {((high as u16) << 8) & (low as u16)}

pub fn split(val:u16) -> (u8, u8) {((val >> 8) as u8, val as u8)}

pub fn signed_add(a: u16, b: u8) -> u16 {
    let (high, mut low) = crate::utils::split(a);
    low = low.wrapping_add(b);
    concat(high, low)
}