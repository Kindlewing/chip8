#[derive(Debug)]
pub enum Flags {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}
