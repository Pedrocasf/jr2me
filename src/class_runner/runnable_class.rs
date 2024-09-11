use crate::class_loader::Class;

use super::MethodFrame;

pub struct RunnableClass<'r> {
    method_frames: Box<[MethodFrame<'r>]>,
}
impl<'r> RunnableClass<'r> {
    pub fn new(class: Class) -> RunnableClass {
        let constant_pool = class.get_constant_pool();
        let method_pool = class.get_methods().iter().map(|m| );
        RunnableClass {}
    }
}
