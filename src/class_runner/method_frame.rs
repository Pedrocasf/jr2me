use super::{RunnableClass, RuntimeTypes};
use crate::class_loader::{constant_pool, Bytecode, ConstantPool, MethodInfo};
pub struct MethodFrame {
    access_flags: u16,
    name: String,
    descriptor: String,
    instruction_pointer: usize,
    stack_pointer: usize,
    code: Box<[Bytecode]>,
    locals: Box<[Option<RuntimeTypes>]>,
    stack: Box<[Option<RuntimeTypes>]>,
}
impl MethodFrame {
    pub fn new(method: MethodInfo, _constant_pool: &ConstantPool) -> MethodFrame {
        let code = method.get_attribute_code().clone();
        let max_stack = code.get_max_stack() as usize;
        let stack = vec![None; max_stack].into_boxed_slice();
        let max_locals = code.get_max_locals() as usize;
        let locals = vec![None; max_locals].into_boxed_slice();
        let bytecode = code.get_bytecode();
        MethodFrame {
            access_flags: method.get_access_flags(),
            name: method.get_name(),
            descriptor: method.get_descriptor(),
            instruction_pointer: 0,
            stack_pointer: 0,
            code: bytecode,
            locals,
            stack,
        }
    }
    pub fn run(&mut self, constant_pool: &ConstantPool, args: Option<Box<[RuntimeTypes]>>) {
        use Bytecode::*;
        let op = self.code[self.instruction_pointer].clone();
        println!("opcode:{:?}", op);
        match op {
            OpALoad0 => {
                if let Some(args) = args.clone() {
                    self.stack[self.stack_pointer] = Some(args[0]);
                } else {
                    let (a, bump_one_more) = RuntimeTypes::new(
                        crate::class_runner::RuntimeTypesOfTypes::TReference,
                        None,
                    );
                    self.stack[self.stack_pointer] = Some(a);
                    if bump_one_more {
                        self.stack_pointer += 1;
                    }
                }
                self.stack_pointer += 1;
                self.instruction_pointer += 1;
            }
            _ => panic!("Unimplemented opcode {:?}", op),
        }
    }
    pub fn get_access_flags(&self) -> u16 {
        self.access_flags
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_descriptor(&self) -> String {
        self.descriptor.clone()
    }
}
