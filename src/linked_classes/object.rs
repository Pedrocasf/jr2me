//! `java.lang.Object` — the root of the hierarchy, and the terminator for any
//! superclass walk. Only `<init>` is filled in; the rest of the API arrives
//! with the heap.

use super::builtin::{BuiltinClass, BuiltinMethod, NativeContext, ACC_PUBLIC, ACC_SUPER};
use crate::class_runner::RuntimeTypes;

pub static OBJECT: BuiltinClass = BuiltinClass {
    name: "java/lang/Object",
    super_name: None,
    access_flags: ACC_PUBLIC | ACC_SUPER,
    methods: &[BuiltinMethod {
        name: "<init>",
        descriptor: "()V",
        access_flags: ACC_PUBLIC,
        body: Some(init),
    }],
};

/// `public Object()` — a no-op; allocation already initialised the fields.
fn init(_ctx: &mut dyn NativeContext, _args: &[RuntimeTypes]) -> Option<RuntimeTypes> {
    None
}
