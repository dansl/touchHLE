//! `UIApplication` and `UIApplicationMain`.

use crate::frameworks::uikit::ui_nib::load_main_nib_file;
use crate::mem::{MutPtr, MutVoidPtr};
use crate::objc::{id, msg, msg_class, nil, objc_classes, ClassExports, HostObject};
use crate::Environment;

#[derive(Default)]
pub struct State {
    /// [UIApplication sharedApplication]
    shared_application: Option<id>,
}

struct UIApplicationHostObject {
    delegate: id,
}
impl HostObject for UIApplicationHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIApplication: UIResponder

// This should only be called by UIApplicationMain
+ (id)allocWithZone:(MutVoidPtr)_zone {
    let host_object = Box::new(UIApplicationHostObject {
        delegate: nil,
    });
    env.objc.alloc_static_object(this, host_object, &mut env.mem)
}

+ (id)sharedApplication {
    env.framework_state.uikit.ui_application.shared_application.unwrap()
}

// This should only be called by UIApplicationMain
- (id)init {
    assert!(env.framework_state.uikit.ui_application.shared_application.is_none());
    env.framework_state.uikit.ui_application.shared_application = Some(this);
    this
}

// This is a singleton, it shouldn't be deallocated.
- (id)retain { this }
- (id)autorelease { this }
- (())release {}

- (id)delegate {
    env.objc.borrow::<UIApplicationHostObject>(this).delegate
}
- (())setDelegate:(id)delegate { // something implementing UIApplicationDelegate
    // This property is non-retaining!
    env.objc.borrow_mut::<UIApplicationHostObject>(this).delegate = delegate;
}

@end

};

/// `UIApplicationMain`, the entry point of the application.
///
/// This function should never return.
pub(super) fn UIApplicationMain(
    env: &mut Environment,
    _argc: i32,
    _argv: MutPtr<MutPtr<u8>>,
    principal_class_name: id, // NSString*
    delegate_class_name: id,  // NSString*
) {
    if principal_class_name != nil || delegate_class_name != nil {
        unimplemented!()
    }

    let ui_application: id = msg_class![env; UIApplication new];

    load_main_nib_file(env, ui_application);

    let delegate: id = msg![env; ui_application delegate];
    assert!(delegate != nil); // should have been set by now

    let _: () = msg![env; delegate applicationDidFinishLaunching:ui_application];

    // TODO: Are there more messages we need to send?
    // TODO: Send UIApplicationDidFinishLaunchingNotification?

    unimplemented!("Send more messages and enter main loop");
}
