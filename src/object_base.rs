use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn object_delete(object: *mut c_void);
    fn object_fast_delete(object: *mut c_void);
    fn object_new() -> *mut c_void;
}

pub struct ObjectBase {
    // Raw pointer to the vtkObjectBase
    object: *mut c_void,
}

#[allow(non_snake_case)]
impl ObjectBase {
    pub fn New() -> Self {
        Self {
            object: unsafe { object_new() },
        }
    }

    pub fn FastDelete(self) {
        unsafe { object_fast_delete(self.object) }
    }
}

impl Drop for ObjectBase {
    fn drop(&mut self) {
        unsafe { object_delete(self.object) };
    }
}
