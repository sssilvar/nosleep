//! `IOPMAssertionCreateWithName` — same class of API as `caffeinate -i` (no fake input).

use std::ffi::{c_char, c_void, CString};

use super::{IdleError, IdleGuard};

type CFStringRef = *const c_void;

const K_CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;
const K_IOPM_ASSERTION_LEVEL_ON: u32 = 255;
const KERN_SUCCESS: i32 = 0;

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {
    fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: u32,
        assertion_name: CFStringRef,
        assertion_id: *mut u32,
    ) -> i32;

    fn IOPMAssertionRelease(assertion_id: u32) -> i32;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFStringCreateWithCString(
        alloc: *const c_void,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;

    fn CFRelease(cf: *const c_void);
}

unsafe fn cf_string(s: &str) -> Result<CFStringRef, IdleError> {
    let c = CString::new(s).map_err(|_| IdleError::System("CString::new failed"))?;
    let r = unsafe {
        CFStringCreateWithCString(std::ptr::null(), c.as_ptr(), K_CF_STRING_ENCODING_UTF8)
    };
    if r.is_null() {
        return Err(IdleError::System("CFStringCreateWithCString returned null"));
    }
    Ok(r)
}

/// Holds `PreventUserIdleSystemSleep` until dropped (same idea as `caffeinate -i`).
pub fn prevent_user_idle_system_sleep() -> Result<IdleGuard, IdleError> {
    unsafe {
        let typ = cf_string("PreventUserIdleSystemSleep")?;
        let name = cf_string("nosleep")?;
        let mut id = 0u32;
        let ret = IOPMAssertionCreateWithName(typ, K_IOPM_ASSERTION_LEVEL_ON, name, &mut id);
        CFRelease(typ);
        CFRelease(name);
        if ret != KERN_SUCCESS {
            return Err(IdleError::AssertionFailed(ret));
        }
        Ok(IdleGuard { id })
    }
}

impl Drop for IdleGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = IOPMAssertionRelease(self.id);
        }
    }
}
