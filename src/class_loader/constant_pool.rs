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
