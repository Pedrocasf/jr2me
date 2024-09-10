#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum ConstantPoolID {
    IString = 1,
    IInteger = 3,
    IFloat = 4,
    ILong = 5,
    IDouble = 6,
    IClassRef = 7,
    IStrRef = 8,
    IFieldRef = 9,
    IMethodRef = 10,
    IInterfaceRef = 11,
    INameTypeDescriptor = 12,
}
impl TryFrom<u8> for ConstantPoolID {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use ConstantPoolID::*;
        match value {
            1 => Ok(IString),
            3 => Ok(IInteger),
            4 => Ok(IFloat),
            5 => Ok(ILong),
            6 => Ok(IDouble),
            7 => Ok(IClassRef),
            8 => Ok(IStrRef),
            9 => Ok(IFieldRef),
            10 => Ok(IMethodRef),
            11 => Ok(IInterfaceRef),
            12 => Ok(INameTypeDescriptor),
            w => Err(format!("Invalid ConstatndPoolId {:#X}", w)),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BaseTypes {
    JString(String),
    JInteger(u32),
    JFloat(f32),
    JLong(u64),
    JDouble(f64),
    JClassRef(u16),
    JStrRef(u16),
    JFieldRef(u16, u16),
    JMethodRef(u16, u16),
    JInterfaceRef(u16, u16),
    JNameTypeDescriptor(u16, u16),
}
impl BaseTypes {
    pub fn new(d: &[u8]) -> (BaseTypes, u16, bool) {
        use BaseTypes::*;
        use ConstantPoolID::*;
        match d[0].try_into().unwrap() {
            IString => {
                let sz = u16::from_be_bytes(d[1..3].try_into().expect("Not enough bytes"));
                return (
                    JString(String::from_utf8(d[3..(3 + sz as usize)].to_vec()).unwrap()),
                    sz + 3,
                    false,
                );
            }
            IInteger => {
                return (
                    JInteger(u32::from_be_bytes(d[1..5].try_into().unwrap())),
                    5,
                    false,
                )
            }
            IFloat => {
                return (
                    JFloat(f32::from_be_bytes(d[1..5].try_into().unwrap())),
                    5,
                    false,
                )
            }
            ILong => {
                return (
                    JLong(u64::from_be_bytes(d[1..9].try_into().unwrap())),
                    9,
                    true,
                )
            }
            IDouble => {
                return (
                    JDouble(f64::from_be_bytes(d[1..9].try_into().unwrap())),
                    9,
                    true,
                )
            }
            IClassRef => {
                return (
                    JClassRef(u16::from_be_bytes(d[1..3].try_into().unwrap())),
                    3,
                    false,
                )
            }
            IStrRef => {
                return (
                    JStrRef(u16::from_be_bytes(d[1..3].try_into().unwrap())),
                    3,
                    false,
                )
            }
            IFieldRef => {
                return (
                    JFieldRef(
                        u16::from_be_bytes(d[1..3].try_into().unwrap()),
                        u16::from_be_bytes(d[3..5].try_into().unwrap()),
                    ),
                    5,
                    false,
                )
            }
            IMethodRef => {
                return (
                    JMethodRef(
                        u16::from_be_bytes(d[1..3].try_into().unwrap()),
                        u16::from_be_bytes(d[3..5].try_into().unwrap()),
                    ),
                    5,
                    false,
                )
            }
            IInterfaceRef => {
                return (
                    JInterfaceRef(
                        u16::from_be_bytes(d[1..3].try_into().unwrap()),
                        u16::from_be_bytes(d[3..5].try_into().unwrap()),
                    ),
                    5,
                    false,
                )
            }
            INameTypeDescriptor => {
                return (
                    JNameTypeDescriptor(
                        u16::from_be_bytes(d[1..3].try_into().unwrap()),
                        u16::from_be_bytes(d[3..5].try_into().unwrap()),
                    ),
                    5,
                    false,
                )
            }
        }
    }
}
/*
    String=1,
    Integer=3,
    Float=4,
    Long=5,
    Double=6,
    ClassRef=7,
    StrRef=8,
    FieldRef=9,
    MethodRef=10,
    InterfaceRef=11,
    NameTypeDescriptor=12,
*/
