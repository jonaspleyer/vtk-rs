//! This module is still incomplete!

use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn named_colors_new() -> *mut c_void;
    fn named_colors_delete(named_colors_ptr: *mut c_void);
}

pub struct vtkNamedColors {
    named_colors_ptr: *mut c_void,
    // TODO replace the above
    // named_colors_ptr: vtkObject,
}

pub trait Colors {
    type Color;
    fn GetColor(&self) -> Self::Color;
    fn ResetColors(&mut self);
    fn SetColor(&mut self, name: &str, color: &Self::Color);
    fn GetColorNames(&self) -> String;
    fn GetSynonyms(&self) -> String;
}

impl vtkNamedColors {
    pub fn New() -> Self {
        Self {
            named_colors_ptr: unsafe { named_colors_new() },
        }
    }
}

impl Drop for vtkNamedColors {
    fn drop(&mut self) {
        unsafe { named_colors_delete(self.named_colors_ptr) };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn SetResetColors() {
        let colors = vtkNamedColors::New();
    }
}
