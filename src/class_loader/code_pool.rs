use super::{constant_pool, ByteCodeID, Bytecode, ConstantPool};
#[derive(Debug, Clone)]
pub struct CodePool {
    code_length: u32,
    code: Box<[Bytecode]>,
}
impl CodePool {
    pub fn new(data: &[u8], constant_pool: &ConstantPool) -> (CodePool, u32) {
        let code_length = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let mut code = Vec::with_capacity(code_length as usize);
        let mut i = 0;
        while i < code_length {
            let (bytecode, size) = Bytecode::new(&data[i as usize + 4..], constant_pool);
            i += size;
            code.push(bytecode)
        }
        let code = code.into_boxed_slice();
        (CodePool { code_length, code }, code_length + 4)
    }
    pub fn get_bytecode(&self) -> Box<[Bytecode]> {
        self.code.clone()
    }
}
