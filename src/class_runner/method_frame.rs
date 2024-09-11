use super::{LocalTypes, RunnableClass, StackTypes};
use crate::class_loader::{Bytecode, ConstantPool, MethodInfo};
pub struct MethodFrame {
    instruction_pointer: usize,
    code: Box<[Bytecode]>,
    locals: Box<[LocalTypes]>,
    stack: Box<[StackTypes]>,
}
impl MethodFrame {
    pub fn new(method: MethodInfo, constant_pool: &ConstantPool) -> MethodFrame {
        let code = method.get_attribute_code().clone();
        let max_stack = code.get_max_stack() as usize;
        let stack = Vec::with_capacity(max_stack).into_boxed_slice();
        let max_locals = code.get_max_locals() as usize;
        let locals = Vec::with_capacity(max_locals).into_boxed_slice();
        let bytecode = code.get_bytecode();
        MethodFrame {
            instruction_pointer: 0,
            code: bytecode,
            locals,
            stack,
        }
    }
}
