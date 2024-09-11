use log::debug;

use super::attribute_pool;
use super::AttributePool;
use super::ConstantPool;
use super::FieldInfo;
use super::FieldPool;
use super::InterfacePool;
use super::MethodInfo;
use super::MethodPool;
#[derive(Debug, Clone)]
pub struct Class {
    magic: u32,
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class_idx: u16,
    super_class_idx: u16,
    interface_pool: InterfacePool,
    field_pool: FieldPool,
    method_pool: MethodPool,
    attribute_pool: AttributePool,
}
impl Class {
    pub fn new(data: Vec<u8>) -> Class {
        let (constant_pool, constant_pool_size) = ConstantPool::new(&data[8..]);
        let (interface_pool, interface_pool_size) =
            InterfacePool::new(&data[14 + constant_pool_size..]);
        let (field_pool, field_pool_size) = FieldPool::new(
            &data[14 + constant_pool_size + interface_pool_size..],
            &constant_pool,
        );
        let (method_pool, method_pool_size) = MethodPool::new(
            &data[14 + constant_pool_size + interface_pool_size + field_pool_size..],
            &constant_pool,
        );
        let (attribute_pool, attribute_pool_size) = AttributePool::new(
            &data[14
                + constant_pool_size
                + interface_pool_size
                + field_pool_size
                + method_pool_size..],
            &constant_pool,
        );
        let c = Class {
            magic: u32::from_be_bytes(
                data[0..4]
                    .try_into()
                    .expect("Class must have the magic number"),
            ),
            minor: u16::from_be_bytes(data[4..6].try_into().expect("Class must have minor")),
            major: u16::from_be_bytes(data[6..8].try_into().expect("Class must have major")),
            constant_pool,
            access_flags: u16::from_be_bytes(
                data[8 + constant_pool_size..8 + constant_pool_size + 2]
                    .try_into()
                    .expect("Class must have access flags"),
            ),
            this_class_idx: u16::from_be_bytes(
                data[10 + constant_pool_size..10 + constant_pool_size + 2]
                    .try_into()
                    .expect("Class must have this class index"),
            ),
            super_class_idx: u16::from_be_bytes(
                data[12 + constant_pool_size..12 + constant_pool_size + 2]
                    .try_into()
                    .expect("Class must have super class index"),
            ),
            interface_pool,
            field_pool,
            method_pool,
            attribute_pool,
        };
        return c;
    }
    pub fn get_constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }
    pub fn get_methods(&self) -> &[MethodInfo] {
        &self.method_pool.get_methods()
    }
    pub fn get_class_idx(&self)->u16{
        self.this_class_idx
    }
}
