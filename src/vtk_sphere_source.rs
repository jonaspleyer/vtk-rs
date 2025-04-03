use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn sphere_source_new() -> *mut c_void;
    fn sphere_source_delete(sphere_source_ptr: *mut c_void);
    fn sphere_source_set_radius(sphere_source_ptr: *mut c_void, radius: f64);
    fn sphere_source_get_radius(sphere_source_ptr: *mut c_void) -> f64;
    fn sphere_source_set_center(spherr_ptr: *mut c_void, center: *const f64);
    fn sphere_source_get_center(sphere_source_ptr: *mut c_void, center: *mut f64);
    fn sphere_source_print_self(sphere_source_ptr: *mut c_void, indent: usize) -> *const c_char;
}

pub struct SphereSource {
    sphere_source_ptr: *mut c_void,
}

impl Default for SphereSource {
    fn default() -> Self {
        Self::new()
    }
}

impl SphereSource {
    #[doc(alias = "New")]
    pub fn new() -> Self {
        Self {
            sphere_source_ptr: unsafe { sphere_source_new() },
        }
    }

    #[doc(alias = "SetRadius")]
    pub fn set_radius(&mut self, radius: f64) {
        unsafe { sphere_source_set_radius(self.sphere_source_ptr, radius) };
    }

    #[doc(alias = "GetRadius")]
    pub fn get_radius(&self) -> f64 {
        unsafe { sphere_source_get_radius(self.sphere_source_ptr) }
    }

    #[doc(alias = "SetCenter")]
    pub fn set_center(&mut self, center: [f64; 3]) {
        unsafe { sphere_source_set_center(self.sphere_source_ptr, center.as_ptr()) };
    }

    #[doc(alias = "GetCenter")]
    pub fn get_center(&self) -> [f64; 3] {
        unsafe {
            let mut center = [0.; 3];
            sphere_source_get_center(self.sphere_source_ptr, center.as_mut_ptr());
            center
        }
    }

    #[doc(alias = "PrintSelf")]
    pub fn print_self(&self, indent: usize) -> CString {
        unsafe {
            let char_ptr = sphere_source_print_self(self.sphere_source_ptr, indent);
            CString::from_raw(char_ptr.cast_mut())
        }
    }
}

impl Drop for SphereSource {
    fn drop(&mut self) {
        unsafe { sphere_source_delete(self.sphere_source_ptr) };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn get_set_radius() {
        let mut sphere = SphereSource::new();
        let r1 = sphere.get_radius();
        assert_abs_diff_eq!(r1, 0.5);
        sphere.set_radius(1.0);
        let r2 = sphere.get_radius();
        assert_abs_diff_eq!(r2, 1.0);
    }

    #[test]
    fn get_set_center() {
        let mut sphere = SphereSource::new();
        let c1 = sphere.get_center();
        assert_abs_diff_eq!(c1, [0.0; 3]);
        sphere.set_center([1., 2., 3.]);
        let c2 = sphere.get_center();
        assert_abs_diff_eq!(c2, [1., 2., 3.]);
    }

    #[test]
    fn print_self() {
        let sphere = SphereSource::new();
        let result = sphere.print_self(3);
        let string = result.to_str().unwrap().to_string();
        println!("{}", result.to_str().unwrap());
        assert!(string.contains("   Debug"));
        assert!(string.contains("   Reference Count: 2"));
        assert!(string.contains("   Registered Events"));
        assert!(string.contains("   Executive"));
        assert!(string.contains("   Phi"));
        assert!(string.contains("   Theta"));
        assert!(string.contains("   Phi Resolution"));
        assert!(string.contains("   Theta Resolution"));
    }
}
