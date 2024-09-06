use super::ConstantPool;
pub struct Class {
    magic: u32,
    minor: u16,
    major: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class_idx: u16,
    super_class_idx: u16,
}
