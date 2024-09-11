use super::{CodeAttribute, ConstantPool};
use crate::{base_types::BaseTypes, class_loader::CodePool};
#[derive(Debug, Clone)]
pub enum Attributes {
    Code(CodeAttribute),
    Unknown,
}
impl Attributes {
    pub fn new(data: &[u8], const_pool: &ConstantPool) -> (Attributes, usize) {
        let name_index = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let attribute_type = const_pool.get_string_of_index(name_index);
        let length = u32::from_be_bytes(data[2..6].try_into().unwrap());
        match attribute_type.as_str() {
            "Code" => {
                let (attrc, sz) = CodeAttribute::new(&data[6..], const_pool);
                return (Attributes::Code(attrc), length as usize + 6);
            }
            _ => panic!("unimplemented attribute: {:}", attribute_type),
        }
    }
}
