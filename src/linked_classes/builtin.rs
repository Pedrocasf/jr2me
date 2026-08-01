#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::class_runner::RuntimeTypes;

/// Access flags (JVMS 4.1, 4.6). Shared by builtin and loaded classes;
/// `resolve_special` needs `ACC_SUPER` and `ACC_ABSTRACT` from both.
pub const ACC_PUBLIC: u16 = 0x0001;
pub const ACC_PRIVATE: u16 = 0x0002;
pub const ACC_PROTECTED: u16 = 0x0004;
pub const ACC_STATIC: u16 = 0x0008;
pub const ACC_FINAL: u16 = 0x0010;
pub const ACC_SUPER: u16 = 0x0020;
pub const ACC_NATIVE: u16 = 0x0100;
pub const ACC_ABSTRACT: u16 = 0x0400;

/// The half of the VM a builtin method is allowed to touch.
///
/// This exists so `linked_classes` does not have to name the `Vm` type: the VM
/// owns the builtin registry, and builtins call back into the VM, so a concrete
/// type here would tie the two modules together in both directions. `Vm`
/// implements this trait.
pub trait NativeContext {
    /// The MIDlet has entered the *Destroyed* state and will not be re-entered.
    fn notify_destroyed(&mut self);
    /// The MIDlet has entered the *Paused* state.
    fn notify_paused(&mut self);
    /// The MIDlet is asking the application manager to be made active again.
    fn resume_request(&mut self);
    /// Attribute lookup against the JAD / manifest of the running suite.
    fn get_app_property(&self, key: &str) -> Option<String>;
}

/// A method body implemented in Rust rather than in bytecode.
///
/// `args[0]` is the objectref (`this`) for instance methods; the declared
/// arguments follow in descriptor order. Returns `None` for a `void` method.
pub type NativeFn = fn(&mut dyn NativeContext, &[RuntimeTypes]) -> Option<RuntimeTypes>;

#[derive(Debug, Clone, Copy)]
pub struct BuiltinMethod {
    pub name: &'static str,
    pub descriptor: &'static str,
    pub access_flags: u16,
    /// `None` marks an abstract method: it is declared here so resolution can
    /// see it, but the body lives in the loaded subclass.
    pub body: Option<NativeFn>,
}

impl BuiltinMethod {
    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }
}

/// A class the VM provides itself, in place of a `.class` file.
#[derive(Debug, Clone, Copy)]
pub struct BuiltinClass {
    /// Internal form, e.g. `javax/microedition/midlet/MIDlet`.
    pub name: &'static str,
    /// `None` only for `java/lang/Object`.
    pub super_name: Option<&'static str>,
    pub access_flags: u16,
    pub methods: &'static [BuiltinMethod],
}

impl BuiltinClass {
    /// Declared methods only — does not walk the superclass chain.
    pub fn find_method(&self, name: &str, descriptor: &str) -> Option<&BuiltinMethod> {
        self.methods
            .iter()
            .find(|m| m.name == name && m.descriptor == descriptor)
    }
}
