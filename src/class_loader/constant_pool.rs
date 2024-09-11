use crate::base_types::BaseTypes;
#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::{any::Any, fmt::Debug};
#[cfg(feature = "std")]
use std::string::String;
#[derive(Clone)]
pub struct ConstantPool {
    size: u16,
    pool: Vec<BaseTypes>,
}
impl ConstantPool {
    pub fn new(data: &[u8]) -> (ConstantPool, usize) {
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        let mut i = 1;
        let mut ptr_idx = 0;
        let mut pool = Vec::with_capacity(size as usize);
        pool.push(BaseTypes::JString("correct to 1 indexed".to_owned()));
        //trace!("{:#X?}", ptr_idx);
        while i < size {
            let (constant_pool_type, typesize, slot_count) = BaseTypes::new(&data[ptr_idx + 2..]);
            i += 1;
            ptr_idx += typesize as usize;
            pool.push(constant_pool_type.clone());
            if slot_count {
                pool.push(constant_pool_type);
                i += 1;
            }
        }
        (ConstantPool { size, pool }, ptr_idx + 2)
    }
    pub fn get_element_of_index(&self, index: u16) -> BaseTypes {
        self.pool[index as usize].clone()
    }
    pub fn get_string_of_index(&self, index: u16) -> String {
        if let BaseTypes::JString(str) = self.pool[index as usize].clone() {
            return str.clone();
        } else {
            panic!(
                "Constant Pool item of index {:} is not a valid JString",
                index
            );
        }
    }
    pub fn get_method_ref_of_index(&self, index: u16) -> (u16, u16) {
        if let BaseTypes::JMethodRef(class, name_type) = self.pool[index as usize].clone() {
            return (class, name_type);
        } else {
            panic!(
                "Constant Pool item of index {:} is not a valid JMethodRef",
                index
            );
        }
    }
    pub fn get_class_ref_of_index(&self, index: u16) -> u16 {
        if let BaseTypes::JClassRef(class) = self.pool[index as usize].clone() {
            return class;
        } else {
            panic!(
                "Constant Pool item of index {:} is not a valid JClassRef",
                index
            );
        }
    }
    pub fn get_name_type_descriptor_of_index(&self, index: u16) -> (u16, u16) {
        if let BaseTypes::JNameTypeDescriptor(class, name_type) = self.pool[index as usize].clone()
        {
            return (class, name_type);
        } else {
            panic!(
                "Constant Pool item of index {:} is not a valid JNmaeTypeDescriptor",
                index
            );
        }
    }
    pub fn get_field_ref_of_index(&self, index: u16) -> (u16, u16) {
        if let BaseTypes::JFieldRef(class, name_type) = self.pool[index as usize].clone() {
            return (class, name_type);
        } else {
            panic!(
                "Constant Pool item of index {:} is not a valid JFieldRef",
                index
            );
        }
    }
    pub fn solve_method_ref_of_index(&self, index: u16) -> (String, String, String) {
        let (class_idx, name_type_idx) = self.get_method_ref_of_index(index);
        let class_name_idx = self.get_class_ref_of_index(class_idx);
        let class_name = self.get_string_of_index(class_name_idx);
        let (method_name_idx, method_type_idx) =
            self.get_name_type_descriptor_of_index(name_type_idx);
        let method_name = self.get_string_of_index(method_name_idx);
        let method_type = self.get_string_of_index(method_type_idx);
        return (class_name, method_name, method_type);
    }
    pub fn solve_class_ref(&self, index: u16) -> String {
        let class_name_idx = self.get_class_ref_of_index(index);
        let class_name = self.get_string_of_index(class_name_idx);
        return class_name;
    }
    pub fn solve_field_ref_of_index(&self, index: u16) -> (String, String, String) {
        let (field_idx, name_type_idx) = self.get_field_ref_of_index(index);
        let class_name_idx = self.get_class_ref_of_index(field_idx);
        let class_name = self.get_string_of_index(class_name_idx);
        let (field_name_idx, field_type_idx) =
            self.get_name_type_descriptor_of_index(name_type_idx);
        let field_name = self.get_string_of_index(field_name_idx);
        let field_type = self.get_string_of_index(field_type_idx);
        return (class_name, field_name, field_type);
    }
}
impl Debug for ConstantPool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let indexed_elements: Vec<(usize, BaseTypes)> = self
            .pool
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (idx, val))
            .collect();
        f.debug_struct("ConstantPool")
            .field("size", &self.size)
            .field("pool", &format_args!("{:#?}", indexed_elements))
            .finish()
    }
}
