pub fn square(s: u32) -> u64 {
    if s == 1 { 1 } else { square(s - 1) * 2 }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
