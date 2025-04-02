use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn object_new() -> *mut c_void;
    fn object_delete(object: *mut c_void);
    fn object_debug_on(object_ptr: *mut c_void);
    fn object_debug_off(object_ptr: *mut c_void);
    fn object_get_debug(object_ptr: *mut c_void) -> bool;
    fn object_set_debug(object_ptr: *mut c_void, flag: bool);
}

pub struct vtkObject {
    object: *mut c_void,
}

impl vtkObject {
    pub fn New() -> Self {
        Self {
            object: unsafe { object_new() },
        }
    }

    pub fn DebugOff(&mut self) {
        unsafe { object_debug_off(self.object) }
    }

    pub fn DebugOn(&mut self) {
        unsafe { object_debug_on(self.object) }
    }

    pub fn GetDebug(&mut self) -> bool {
        unsafe { object_get_debug(self.object) }
    }

    pub fn SetDebug(&mut self, flag: bool) {
        unsafe { object_set_debug(self.object, flag) }
    }

    // pub fn GetMTime(&self) -> vtkMTimeType {}
}

impl Drop for vtkObject {
    fn drop(&mut self) {
        unsafe { object_delete(self.object) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn DebugOnOff() {
        let mut object = vtkObject::New();
        object.DebugOn();
        assert!(object.GetDebug());
        object.DebugOff();
        assert!(!object.GetDebug());
        object.SetDebug(true);
        assert!(object.GetDebug());
        object.SetDebug(false);
        assert!(!object.GetDebug());
    }
}
