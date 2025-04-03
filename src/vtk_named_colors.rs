//! This module is still incomplete!

use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn named_colors_new() -> *mut c_void;
    fn named_colors_delete(named_colors_ptr: *mut c_void);
}

pub struct NamedColors {
    named_colors_ptr: *mut c_void,
    // TODO replace the above
    // named_colors_ptr: vtkObject,
}

pub trait Colors {
    type Color;
    fn get_color(&self) -> Self::Color;
    fn reset_colors(&mut self);
    fn set_color(&mut self, name: &str, color: &Self::Color);
    fn get_color_names(&self) -> String;
    fn get_synonyms(&self) -> String;
}

impl NamedColors {
    pub fn new() -> Self {
        Self {
            named_colors_ptr: unsafe { named_colors_new() },
        }
    }
}

impl Drop for NamedColors {
    fn drop(&mut self) {
        unsafe { named_colors_delete(self.named_colors_ptr) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_reset_colors() {
        let colors = NamedColors::new();
    }
}
