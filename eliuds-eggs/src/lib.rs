pub fn egg_count(display_value: u32) -> usize {
    (0..32).filter(|n| nth_bit(display_value, *n) != 0).count()
}

fn nth_bit(value: u32, n: u32) -> u32 {
    (value >> n) & 1
}
