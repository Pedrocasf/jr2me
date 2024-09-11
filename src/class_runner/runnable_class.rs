use crate::class_loader::{Class, CodeAttribute, ConstantPool};

use super::MethodFrame;

pub struct RunnableClass {
    method_frames: Box<[MethodFrame]>,
}
impl RunnableClass {
    pub fn new(class: Class) -> RunnableClass {
        let constant_pool = class.get_constant_pool();
        let method_frames: Box<[MethodFrame]> = class
            .get_methods()
            .iter()
            .map(|m| MethodFrame::new(m.clone(), &constant_pool))
            .collect();
        RunnableClass { method_frames }
    }
    pub fn run_method(&mut self,name:String, constant_pool:&ConstantPool){

    }
    pub fn find_method(&self, name:String, type:String)->u16{
        self.methods
            .iter()
            .position(|m|)
    }
}
