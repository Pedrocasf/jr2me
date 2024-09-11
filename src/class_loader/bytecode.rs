use super::ConstantPool;

#[derive(Debug, Clone)]
pub enum Bytecode {
    OpNop,
    OpAConstNull,
    OpFConst0,
    OpLdc,
    OpALoad0,
    OpDup,
    OpReturn,
    OpGetField(String, String, String),
    OpPutField(String, String, String),
    OpInvokeVirtual(String, String, String),
    OpInvokeSpecial(String, String, String),
    OpInvokeStatic(String, String, String),
    OpNew(String),
}
impl Bytecode {
    pub fn new(data: &[u8], constant_pool: &ConstantPool) -> (Bytecode, u32) {
        use ByteCodeID::*;
        use Bytecode::*;
        match data[0].try_into().unwrap() {
            IdNop => return (OpNop, 1),
            IdAConstNull => return (OpAConstNull, 1),
            IdFConst0 => return (OpFConst0, 1),
            IdLdc => return (OpLdc, 1),
            IdALoad0 => return (OpALoad0, 1),
            IdDup => return (OpDup, 1),
            IdReturn => return (OpReturn, 1),
            IdGetField => {
                let (class_name, field_name, field_type) = constant_pool
                    .solve_field_ref_of_index(u16::from_be_bytes(data[1..3].try_into().unwrap()));
                return (OpGetField(class_name, field_name, field_type), 3);
            }
            IdPutField => {
                let (class_name, field_name, field_type) = constant_pool
                    .solve_field_ref_of_index(u16::from_be_bytes(data[1..3].try_into().unwrap()));
                return (OpPutField(class_name, field_name, field_type), 3);
            }
            IdInvokeVirtual => {
                let (class_name, method_name, method_type) = constant_pool
                    .solve_method_ref_of_index(u16::from_be_bytes(data[1..3].try_into().unwrap()));
                return (OpInvokeVirtual(class_name, method_name, method_type), 3);
            }
            IdInvokeSpecial => {
                let (class_name, method_name, method_type) = constant_pool
                    .solve_method_ref_of_index(u16::from_be_bytes(data[1..3].try_into().unwrap()));
                return (OpInvokeSpecial(class_name, method_name, method_type), 3);
            }
            IdInvokeStatic => {
                let (class_name, method_name, method_type) = constant_pool
                    .solve_method_ref_of_index(u16::from_be_bytes(data[1..3].try_into().unwrap()));
                return (OpInvokeStatic(class_name, method_name, method_type), 3);
            }
            IdNew => {
                let class_name = constant_pool
                    .solve_class_ref(u16::from_be_bytes(data[1..3].try_into().unwrap()));
                return (OpNew(class_name), 3);
            }
        }
    }
}
#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum ByteCodeID {
    IdNop = 0x00,
    IdAConstNull = 0x01,
    IdFConst0 = 0x0B,
    IdLdc = 0x12,
    IdALoad0 = 0x2A,
    IdDup = 0x59,
    IdReturn = 0xB1,
    IdGetField = 0xB4,
    IdPutField = 0xB5,
    IdInvokeVirtual = 0xB6,
    IdInvokeSpecial = 0xB7,
    IdInvokeStatic = 0xB8,
    IdNew = 0xBB,
}
impl TryFrom<u8> for ByteCodeID {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use ByteCodeID::*;
        match value {
            0x00 => Ok(IdNop),
            0x01 => Ok(IdAConstNull),
            0x0B => Ok(IdFConst0),
            0x12 => Ok(IdLdc),
            0x2A => Ok(IdALoad0),
            0x59 => Ok(IdDup),
            0xB1 => Ok(IdReturn),
            0xB4 => Ok(IdGetField),
            0xB5 => Ok(IdPutField),
            0xB6 => Ok(IdInvokeVirtual),
            0xB7 => Ok(IdInvokeSpecial),
            0xB8 => Ok(IdInvokeStatic),
            0xBB => Ok(IdNew),
            _ => Err(format!("Op:{:x} not implemented", value)),
        }
    }
}
