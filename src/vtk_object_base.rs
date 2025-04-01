use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn object_delete(object: *mut c_void);
    fn object_fast_delete(object: *mut c_void);
    fn object_new() -> *mut c_void;
    fn object_get_class_name(object: *mut c_void) -> *const c_char;
    fn object_get_object_description(object: *mut c_void) -> *const c_char;
}

pub struct vtkObjectBase {
    // Raw pointer to the vtkObjectBase
    object: *mut c_void,
}

impl vtkObjectBase {
    pub fn New() -> Self {
        Self {
            object: unsafe { object_new() },
        }
    }

    pub fn FastDelete(self) {
        unsafe { object_fast_delete(self.object) }
    }

    pub fn GetClassName(&self) -> Result<&str, core::str::Utf8Error> {
        unsafe {
            let char_ptr = object_get_class_name(self.object);
            let c_str = core::ffi::CStr::from_ptr(char_ptr);
            c_str.to_str()
        }
    }

    pub fn GetObjectDescription(&self) -> CString {
        unsafe {
            let char_ptr = object_get_object_description(self.object);
            CString::from_raw(char_ptr.cast_mut())
        }
    }
}

impl Drop for vtkObjectBase {
    fn drop(&mut self) {
        unsafe { object_delete(self.object) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn GetClassName() {
        let object = vtkObjectBase::New();
        let name = object.GetClassName().unwrap();
        assert_eq!("vtkObjectBase", name);
    }

    #[test]
    fn GetObjectDescription() {
        let object = vtkObjectBase::New();
        let name = object.GetObjectDescription();
        assert_eq!(
            &name.as_bytes()[0..13],
            CString::new("vtkObjectBase").unwrap().as_bytes()
        );
    }
}
