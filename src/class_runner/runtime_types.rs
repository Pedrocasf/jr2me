#[derive(Debug, Clone, Copy)]
pub enum RuntimeTypesOfTypes {
    TBoolean,
    TChar,
    TShort,
    TInt,
    TFLoat,
    TReference,
    TReturnAddress,
    TLong,
    TDouble,
}
#[derive(Debug, Clone, Copy)]
pub enum RuntimeTypes {
    RBoolean(bool),
    RChar(u8),
    RShort(u16),
    RInt(u32),
    RFloat(f32),
    RReference(u32),
    RTRetrunAddress(u32),
    RLong(u64),
    RDouble(f64),
}
impl RuntimeTypes {
    pub fn new(type_of_type: RuntimeTypesOfTypes, data: Option<&[u8]>) -> (RuntimeTypes, bool) {
        use RuntimeTypes::*;
        use RuntimeTypesOfTypes::*;
        let d;
        if let Some(data) = data {
            d = data;
        } else {
            d = &[0; 8];
        }
        match type_of_type {
            TBoolean => (RBoolean(d[0] != 0), false),
            TChar => (RChar(d[0]), false),
            TShort => (
                RShort(u16::from_be_bytes(
                    d[0..2].try_into().expect("Not enough bytes"),
                )),
                false,
            ),
            TInt => (
                RInt(u32::from_be_bytes(
                    d[0..4].try_into().expect("Not enough bytes"),
                )),
                false,
            ),
            TFLoat => (
                RFloat(f32::from_be_bytes(
                    d[0..4].try_into().expect("Not enough bytes"),
                )),
                false,
            ),
            TReference => (
                RReference(u32::from_be_bytes(
                    d[0..4].try_into().expect("Not enough bytes"),
                )),
                false,
            ),
            TReturnAddress => (
                RTRetrunAddress(u32::from_be_bytes(
                    d[0..4].try_into().expect("Not enough bytes"),
                )),
                false,
            ),
            TLong => (
                RLong(u64::from_be_bytes(
                    d[0..8].try_into().expect("Not enough bytes"),
                )),
                true,
            ),
            TDouble => (
                RDouble(f64::from_be_bytes(
                    d[0..8].try_into().expect("Not enough bytes"),
                )),
                true,
            ),
        }
    }
}
