//! The set of classes the VM supplies itself, and lookup over them.

use super::builtin::{BuiltinClass, BuiltinMethod};
use super::midlet::MIDLET;
use super::object::OBJECT;

pub static BUILTIN_CLASSES: &[&BuiltinClass] = &[&OBJECT, &MIDLET];

pub fn find_class(name: &str) -> Option<&'static BuiltinClass> {
    BUILTIN_CLASSES.iter().copied().find(|c| c.name == name)
}

/// Look up a method starting at `class` and walking up the superclass chain,
/// the way `invokespecial` resolution does (JVMS 5.4.3.3).
///
/// Only reaches builtin classes: a chain that passes through a loaded `.class`
/// has to be walked by the VM, which can consult both. Returns the declaring
/// class alongside the method, since `resolve_special` needs to know where the
/// hit came from to apply the `ACC_SUPER` rule.
pub fn resolve_method(
    class: &str,
    name: &str,
    descriptor: &str,
) -> Option<(&'static BuiltinClass, &'static BuiltinMethod)> {
    let mut current = find_class(class)?;
    loop {
        if let Some(method) = current.find_method(name, descriptor) {
            return Some((current, method));
        }
        current = find_class(current.super_name?)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class_runner::RuntimeTypes;
    use crate::linked_classes::builtin::NativeContext;
    #[cfg(not(feature = "std"))]
    use alloc::string::String;

    #[derive(Default)]
    struct TestContext {
        destroyed: bool,
        paused: bool,
        resumed: bool,
    }

    impl NativeContext for TestContext {
        fn notify_destroyed(&mut self) {
            self.destroyed = true;
        }
        fn notify_paused(&mut self) {
            self.paused = true;
        }
        fn resume_request(&mut self) {
            self.resumed = true;
        }
        fn get_app_property(&self, _key: &str) -> Option<String> {
            None
        }
    }

    /// The call `HelloMIDlet.<init>` makes at bytecode offset 1.
    #[test]
    fn midlet_init_resolves_and_runs() {
        let (class, method) =
            resolve_method("javax/microedition/midlet/MIDlet", "<init>", "()V").unwrap();
        assert_eq!(class.name, "javax/microedition/midlet/MIDlet");

        let this = RuntimeTypes::RReference(0);
        let body = method.body.expect("<init> must have a body");
        assert!(body(&mut TestContext::default(), &[this]).is_none());
    }

    #[test]
    fn superclass_chain_is_walked() {
        // The walk must terminate at Object rather than loop or panic.
        assert!(resolve_method("javax/microedition/midlet/MIDlet", "<init>", "(I)V").is_none());
        assert!(resolve_method("javax/microedition/midlet/MIDlet", "nope", "()V").is_none());
        assert!(resolve_method("no/such/Class", "<init>", "()V").is_none());
    }

    #[test]
    fn lifecycle_callbacks_are_abstract() {
        for (name, descriptor) in [
            ("startApp", "()V"),
            ("pauseApp", "()V"),
            ("destroyApp", "(Z)V"),
        ] {
            let (_, method) =
                resolve_method("javax/microedition/midlet/MIDlet", name, descriptor).unwrap();
            assert!(method.is_abstract(), "{} should be abstract", name);
            assert!(method.body.is_none(), "{} should have no builtin body", name);
        }
    }

    #[test]
    fn notify_destroyed_reaches_the_context() {
        let (_, method) =
            resolve_method("javax/microedition/midlet/MIDlet", "notifyDestroyed", "()V").unwrap();
        let mut ctx = TestContext::default();
        (method.body.unwrap())(&mut ctx, &[RuntimeTypes::RReference(0)]);
        assert!(ctx.destroyed);
        assert!(!ctx.paused);
        assert!(!ctx.resumed);
    }
}
