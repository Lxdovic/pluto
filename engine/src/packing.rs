#[inline]
pub const fn s(mg: i32, eg: i32) -> i32 {
    ((eg as u32) << 16) as i32 + mg
}

pub const fn extract_mg(value: i32) -> i32 {
    value as i16 as i32
}

pub const fn extract_eg(value: i32) -> i32 {
    ((value + 0x8000) >> 16) as i16 as i32
}
