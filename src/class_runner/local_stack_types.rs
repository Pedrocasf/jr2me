#[derive(Debug, Clone, Copy)]
pub enum LocalStackTypes {
    TBoolean(bool),
    TChar(u8),
    TShort(u16),
    TInt(u32),
    TFloat(f32),
    TReference(u32),
    TRetrunAddress(u32),
    TLong(u64),
    TDouble(f64)
}
