pub mod builtin;
pub use builtin::{BuiltinClass, BuiltinMethod, NativeContext, NativeFn};
pub mod object;
pub use object::OBJECT;
pub mod midlet;
pub use midlet::MIDLET;
pub mod implemented;
pub use implemented::{find_class, resolve_method, BUILTIN_CLASSES};
