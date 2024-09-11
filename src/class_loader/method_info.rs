use super::{AttributePool, CodeAttribute, CodePool, ConstantPool};
#[derive(Debug, Clone)]
pub struct MethodInfo {
    access_flags: u16,
    name: String,
    descriptor: String,
    attribute_pool: AttributePool,
}
impl MethodInfo {
    pub fn new(data: &[u8], const_pool: &ConstantPool) -> (MethodInfo, u32) {
        let access_flags = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let name_index = u16::from_be_bytes(data[2..4].try_into().unwrap());
        let name = const_pool.get_string_of_index(name_index);
        let descriptor_index = u16::from_be_bytes(data[4..6].try_into().unwrap());
        let descriptor = const_pool.get_string_of_index(descriptor_index);
        let (attribute_pool, size) = AttributePool::new(&data[6..], const_pool);
        (
            MethodInfo {
                access_flags,
                name,
                descriptor,
                attribute_pool,
            },
            size + 6,
        )
    }
    pub fn get_attribute_code(&self) -> Vec<CodeAttribute> {
        self.attribute_pool.get_attribute_code()
    }
}
