use core::panic;

use crate::class_loader::{Class, CodeAttribute, ConstantPool};
use super::LocalStackTypes;
use super::MethodFrame;

pub struct RunnableClass {
    method_frames: Box<[MethodFrame]>,
}
impl RunnableClass {
    pub fn new(class: &Class) -> RunnableClass {
        let constant_pool = class.get_constant_pool();
        let method_frames: Box<[MethodFrame]> = class
            .get_methods()
            .iter()
            .map(|m| MethodFrame::new(m.clone(), &constant_pool))
            .collect();
        RunnableClass { method_frames }
    }
    pub fn run_method(&mut self,name:&String,descriptor:&String, constant_pool:&ConstantPool, class_idx:u16, args:Option<Box<[LocalStackTypes]>>){
        if let Some(method_idx) = self.find_method(name, descriptor){
            return self.method_frames[method_idx].run(constant_pool, args);
        }else{
            panic!("Method {}, not found in class {}", name, constant_pool.solve_str_ref_of_index(class_idx));
        }
    }
    pub fn find_method(&self, name:&String, descriptor:&String)->Option<usize>{
        self.method_frames
            .iter()
            .position(|m| &m.get_name()==name && &m.get_descriptor()==descriptor)
    }
}
