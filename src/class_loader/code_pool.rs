use super::{Bytecode, ByteCodeID};
#[derive(Debug, Clone)]
pub struct CodePool {
    code_length: u32,
    code: Vec<Bytecode>,
}
impl CodePool {
    pub fn new(data: &[u8]) -> (CodePool, u32) {
        let code_length = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let mut code = Vec::with_capacity(code_length as usize);
        let mut i = 0;
        while i < code_length {
            let (bytecode, size) = Bytecode::new(&data[i as usize + 4..]);
            println!("bytecode: {:#X?}",bytecode);
            i += size;
            code.push(bytecode)
        }
        (CodePool { code_length, code }, code_length + 4)
    }
}
