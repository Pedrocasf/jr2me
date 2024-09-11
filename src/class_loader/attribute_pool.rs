use super::{Attributes, CodeAttribute, ConstantPool};
#[derive(Debug, Clone)]
pub struct AttributePool {
    size: u16,
    info: Box<[Attributes]>,
}
impl AttributePool {
    pub fn new(data: &[u8], const_pool: &ConstantPool) -> (AttributePool, u32) {
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut info = Vec::with_capacity(size as usize);
        let mut length = 2;
        for _ in 0..size {
            let (attr, sz) = Attributes::new(&data[length..], const_pool);
            length += sz;
            info.push(attr);
        }
        let info = info.into_boxed_slice();
        (AttributePool { size, info }, length as u32)
    }
    pub fn get_attribute_code(&self) -> Box<[CodeAttribute]> {
        self.info
            .iter()
            .filter_map(|attr| {
                if let Attributes::Code(code_attribute) = attr.clone() {
                    Some(code_attribute)
                } else {
                    None
                }
            })
            .collect()
    }
}
