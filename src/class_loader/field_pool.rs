use super::ConstantPool;
use super::FieldInfo;
#[derive(Debug, Clone)]
pub struct FieldPool {
    size: u16,
    fields: Box<[FieldInfo]>,
}
impl FieldPool {
    pub fn new(data: &[u8], const_pool: &ConstantPool) -> (FieldPool, usize) {
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut fields = Vec::with_capacity(size as usize);
        let mut idx = 2;
        for _ in 0..(size as usize) {
            let (f, sz) = FieldInfo::new(&data[idx..], const_pool);
            fields.push(f);
            idx += sz as usize;
        }
        let fields = fields.into_boxed_slice();
        (FieldPool { size, fields }, idx as usize)
    }
}
