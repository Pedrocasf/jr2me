//! `javax.microedition.midlet.MIDlet` (MIDP 2.0).
//!
//! Every MIDlet suite entry point extends this class, so its `<init>` is the
//! first builtin any real application reaches — `HelloMIDlet.<init>` calls it
//! at bytecode offset 1.

use super::builtin::{
    BuiltinClass, BuiltinMethod, NativeContext, ACC_ABSTRACT, ACC_FINAL, ACC_PROTECTED, ACC_PUBLIC,
    ACC_SUPER,
};
use crate::class_runner::RuntimeTypes;

pub static MIDLET: BuiltinClass = BuiltinClass {
    name: "javax/microedition/midlet/MIDlet",
    super_name: Some("java/lang/Object"),
    access_flags: ACC_PUBLIC | ACC_SUPER | ACC_ABSTRACT,
    methods: &[
        BuiltinMethod {
            name: "<init>",
            descriptor: "()V",
            access_flags: ACC_PROTECTED,
            body: Some(init),
        },
        // Lifecycle callbacks. Declared abstract here so that resolution can
        // see them; the bodies come from the loaded subclass.
        BuiltinMethod {
            name: "startApp",
            descriptor: "()V",
            access_flags: ACC_PROTECTED | ACC_ABSTRACT,
            body: None,
        },
        BuiltinMethod {
            name: "pauseApp",
            descriptor: "()V",
            access_flags: ACC_PROTECTED | ACC_ABSTRACT,
            body: None,
        },
        BuiltinMethod {
            name: "destroyApp",
            descriptor: "(Z)V",
            access_flags: ACC_PROTECTED | ACC_ABSTRACT,
            body: None,
        },
        // Final methods the application manager provides.
        BuiltinMethod {
            name: "notifyDestroyed",
            descriptor: "()V",
            access_flags: ACC_PUBLIC | ACC_FINAL,
            body: Some(notify_destroyed),
        },
        BuiltinMethod {
            name: "notifyPaused",
            descriptor: "()V",
            access_flags: ACC_PUBLIC | ACC_FINAL,
            body: Some(notify_paused),
        },
        BuiltinMethod {
            name: "resumeRequest",
            descriptor: "()V",
            access_flags: ACC_PUBLIC | ACC_FINAL,
            body: Some(resume_request),
        },
        BuiltinMethod {
            name: "getAppProperty",
            descriptor: "(Ljava/lang/String;)Ljava/lang/String;",
            access_flags: ACC_PUBLIC | ACC_FINAL,
            body: Some(get_app_property),
        },
        BuiltinMethod {
            name: "platformRequest",
            descriptor: "(Ljava/lang/String;)Z",
            access_flags: ACC_PUBLIC | ACC_FINAL,
            body: Some(platform_request),
        },
        BuiltinMethod {
            name: "checkPermission",
            descriptor: "(Ljava/lang/String;)I",
            access_flags: ACC_PUBLIC | ACC_FINAL,
            body: Some(check_permission),
        },
    ],
};

/// `protected MIDlet()`
///
/// A no-op. The real constructor only registers the instance with the
/// application manager, which the VM already knows about by the time it
/// decides to run a MIDlet.
fn init(_ctx: &mut dyn NativeContext, _args: &[RuntimeTypes]) -> Option<RuntimeTypes> {
    None
}

/// `public final void notifyDestroyed()`
fn notify_destroyed(ctx: &mut dyn NativeContext, _args: &[RuntimeTypes]) -> Option<RuntimeTypes> {
    ctx.notify_destroyed();
    None
}

/// `public final void notifyPaused()`
fn notify_paused(ctx: &mut dyn NativeContext, _args: &[RuntimeTypes]) -> Option<RuntimeTypes> {
    ctx.notify_paused();
    None
}

/// `public final void resumeRequest()`
fn resume_request(ctx: &mut dyn NativeContext, _args: &[RuntimeTypes]) -> Option<RuntimeTypes> {
    ctx.resume_request();
    None
}

/// `public final String getAppProperty(String key)`
///
/// Blocked on the heap: the key arrives as a reference and the result has to be
/// handed back as one. `NativeContext::get_app_property` is already wired for
/// when string objects exist.
fn get_app_property(_ctx: &mut dyn NativeContext, _args: &[RuntimeTypes]) -> Option<RuntimeTypes> {
    unimplemented!("MIDlet.getAppProperty needs heap-allocated java/lang/String objects")
}

/// `public final boolean platformRequest(String url)`
///
/// Always reports `false`: the platform has not been asked to do anything, so
/// the MIDlet does not need to exit first. A real implementation throws
/// `ConnectionNotFoundException` when it cannot honour the URL.
fn platform_request(_ctx: &mut dyn NativeContext, _args: &[RuntimeTypes]) -> Option<RuntimeTypes> {
    Some(RuntimeTypes::RBoolean(false))
}

/// `public final int checkPermission(String permission)`
///
/// Always reports `1` (permitted). There is no security policy yet; `0` denies
/// and `-1` means "cannot be determined without asking the user".
fn check_permission(_ctx: &mut dyn NativeContext, _args: &[RuntimeTypes]) -> Option<RuntimeTypes> {
    Some(RuntimeTypes::RInt(1))
}
