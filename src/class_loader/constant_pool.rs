use crate::base_types::{
    java_class_ref::JClassRef, java_double::JDouble, java_field_ref::JFieldRef, java_float::JFloat,
    java_integer::JInteger, java_interface_ref::JInterfaceRef, java_long::JLong,
    java_method_ref::JMethodRef, java_name_type_descriptor::JNameTypeDescriptor,
    java_str_ref::JStrRef, java_string::JString, JBaseType,
};
use core::{any::Any, fmt::Debug};
#[cfg(feature = "std")]
use std::string::String;
//use crate::ClassLoader::ConstantPool::ConstantPoolID::IString;
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(feature = "log")]
//use log::trace;
//use std::any::Any;
#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum ConstantPoolID {
    IString = 1,
    IInteger = 3,
    IFloat = 4,
    ILong = 5,
    IDouble = 6,
    IClassRef = 7,
    IStrRef = 8,
    IFieldRef = 9,
    IMethodRef = 10,
    IInterfaceRef = 11,
    INameTypeDescriptor = 12,
}
impl TryFrom<u8> for ConstantPoolID {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use ConstantPoolID::*;
        match value {
            1 => Ok(IString),
            3 => Ok(IInteger),
            4 => Ok(IFloat),
            5 => Ok(ILong),
            6 => Ok(IDouble),
            7 => Ok(IClassRef),
            8 => Ok(IStrRef),
            9 => Ok(IFieldRef),
            10 => Ok(IMethodRef),
            11 => Ok(IInterfaceRef),
            12 => Ok(INameTypeDescriptor),
            w => Err(format!("Invalid ConstatndPoolId {:#X}", w)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ConstantPoolType {
    pub id: ConstantPoolID,
    pub val: Box<dyn JBaseType>,
}
impl ConstantPoolType {
    pub fn new(v: &[u8]) -> (ConstantPoolType, u16, bool) {
        use ConstantPoolID::*;
        //trace!("id:{:?}", <u8 as TryInto<ConstantPoolID>>::try_into(v[0]).unwrap());
        match v[0]
            .try_into()
            .expect(&format!("Invalid ConstatndPoolId {:#X?}", v[0]))
        {
            IString => (
                ConstantPoolType {
                    id: IString,
                    val: Box::new(JString::new(&v[1..])),
                },
                u16::from_be_bytes(v[1..3].try_into().unwrap()) + 3,
                false,
            ),
            IInteger => (
                ConstantPoolType {
                    id: IInteger,
                    val: Box::new(JInteger::new(v[1..5].try_into().unwrap())),
                },
                5,
                false,
            ),
            IFloat => (
                ConstantPoolType {
                    id: IFloat,
                    val: Box::new(JFloat::new(v[1..5].try_into().unwrap())),
                },
                5,
                false,
            ),
            ILong => (
                ConstantPoolType {
                    id: ILong,
                    val: Box::new(JLong::new(v[1..9].try_into().unwrap())),
                },
                9,
                true,
            ),
            IDouble => (
                ConstantPoolType {
                    id: IDouble,
                    val: Box::new(JDouble::new(v[1..9].try_into().unwrap())),
                },
                9,
                true,
            ),
            IClassRef => (
                ConstantPoolType {
                    id: IClassRef,
                    val: Box::new(JClassRef::new(v[1..3].try_into().unwrap())),
                },
                3,
                false,
            ),
            IStrRef => (
                ConstantPoolType {
                    id: IStrRef,
                    val: Box::new(JStrRef::new(v[1..3].try_into().unwrap())),
                },
                3,
                false,
            ),
            IFieldRef => (
                ConstantPoolType {
                    id: IFieldRef,
                    val: Box::new(JFieldRef::new(v[1..5].try_into().unwrap())),
                },
                5,
                false,
            ),
            IMethodRef => (
                ConstantPoolType {
                    id: IMethodRef,
                    val: Box::new(JMethodRef::new(v[1..5].try_into().unwrap())),
                },
                5,
                false,
            ),
            IInterfaceRef => (
                ConstantPoolType {
                    id: IInterfaceRef,
                    val: Box::new(JInterfaceRef::new(v[1..5].try_into().unwrap())),
                },
                5,
                false,
            ),
            INameTypeDescriptor => (
                ConstantPoolType {
                    id: INameTypeDescriptor,
                    val: Box::new(JNameTypeDescriptor::new(v[1..5].try_into().unwrap())),
                },
                5,
                false,
            ),
        }
    }
}
#[derive(Clone)]
pub struct ConstantPool {
    size: u16,
    pool: Vec<ConstantPoolType>,
}
impl ConstantPool {
    pub fn new(data: &[u8]) -> (ConstantPool, usize) {
        let size = u16::from_be_bytes(data[0..2].try_into().unwrap());
        //trace!("{:#X?}",size);
        let mut i = 1;
        let mut ptr_idx = 2;
        let mut pool = Vec::new();
        //trace!("{:#X?}", ptr_idx);
        while i < size {
            let (constant_pool_type, typesize, slot_count) =
                ConstantPoolType::new(&data[ptr_idx..]);
            i += 1;
            ptr_idx += typesize as usize;
            pool.push(constant_pool_type.clone());
            if slot_count {
                pool.push(constant_pool_type);
                i += 1;
            }
        }
        (ConstantPool { size, pool }, ptr_idx)
    }
    pub fn get_elements_of_type(&self, type_id: ConstantPoolID) -> Vec<(u16, ConstantPoolType)> {
        self.pool
            .iter()
            .enumerate()
            .filter(|(_idx, i)| type_id == ((**i).id))
            .map(|(idx, i)| (idx as u16, i.clone()))
            .collect()
    }
    pub fn get_element_of_index(&self, index: u16) -> ConstantPoolType {
        self.pool[index as usize].clone()
    }
    pub fn get_element_of_type_and_index(
        &self,
        type_id: ConstantPoolID,
        index: u16,
    ) -> ConstantPoolType {
        if self.pool[index as usize].id == type_id {
            return self.pool[index as usize].clone();
        } else {
            unreachable!()
        }
    }
}
impl Debug for ConstantPool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let indexed_elements: Vec<(usize, ConstantPoolType)> = self
            .pool
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (idx + 1, val))
            .collect();
        f.debug_struct("ConstantPool")
            .field("size", &self.size)
            .field("pool", &format_args!("{:#?}", indexed_elements))
            .finish()
    }
}
