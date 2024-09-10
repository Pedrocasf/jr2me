use super::ConstantPool;
use super::MethodInfo;
#[derive(Debug, Clone)]
pub struct MethodPool {
    size: u16,
    methods: Vec<MethodInfo>,
}
impl MethodPool {
    pub fn new(data: &[u8], const_pool: &ConstantPool) -> (MethodPool, usize) {
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut methods = Vec::with_capacity(size as usize);
        let mut idx = 2;
        for _ in 0..(size as usize) {
            let (m, sz) = MethodInfo::new(&data[idx..], const_pool);
            methods.push(m);
            idx += sz as usize;
        }
        (MethodPool { size, methods }, idx)
    }
}
