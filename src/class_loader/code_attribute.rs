use super::{AttributePool, Bytecode, CodePool, ConstantPool, ExceptionTablePool};
#[derive(Debug, Clone)]
pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code: CodePool,
    exception_table: ExceptionTablePool,
    attribute_pool: AttributePool,
}
impl CodeAttribute {
    pub fn new(data: &[u8], const_pool: &ConstantPool) -> (CodeAttribute, u32) {
        let (code_pool, sz_code) = CodePool::new(&data[4..], const_pool);
        let (exception_table, sz_exception) =
            ExceptionTablePool::new(&data[sz_code as usize + 4..]);
        let (attr_pool, sz_attr) = AttributePool::new(
            &data[sz_code as usize + sz_exception as usize + 4..],
            const_pool,
        );
        (
            CodeAttribute {
                max_stack: u16::from_be_bytes(data[0..2].try_into().unwrap()),
                max_locals: u16::from_be_bytes(data[2..4].try_into().unwrap()),
                code: code_pool,
                exception_table,
                attribute_pool: attr_pool,
            },
            sz_code + sz_exception + sz_attr + 4,
        )
    }
    pub fn get_max_stack(&self) -> u16 {
        self.max_stack
    }
    pub fn get_max_locals(&self) -> u16 {
        self.max_locals
    }
    pub fn get_bytecode(&self) -> Vec<Bytecode> {
        self.code.get_bytecode()
    }
}
