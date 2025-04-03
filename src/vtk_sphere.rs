use core::ffi::{c_char, c_void};
use std::ffi::CString;

unsafe extern "C" {
    fn sphere_new() -> *mut c_void;
    fn sphere_delete(sphere_ptr: *mut c_void);
    fn sphere_set_radius(sphere_ptr: *mut c_void, radius: f64);
    fn sphere_get_radius(sphere_ptr: *mut c_void) -> f64;
    fn sphere_set_center(spherr_ptr: *mut c_void, center: *const f64);
    fn sphere_get_center(sphere_ptr: *mut c_void, center: *mut f64);
    fn sphere_print_self(sphere_ptr: *mut c_void, indent: usize) -> *const c_char;
}

pub struct Sphere {
    sphere_ptr: *mut c_void,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            sphere_ptr: unsafe { sphere_new() },
        }
    }

    #[doc(alias = "SetRadius")]
    pub fn set_radius(&mut self, radius: f64) {
        unsafe { sphere_set_radius(self.sphere_ptr, radius) };
    }

    #[doc(alias = "GetRadius")]
    pub fn radius(&self) -> f64 {
        unsafe { sphere_get_radius(self.sphere_ptr) }
    }

    #[doc(alias = "SetCenter")]
    pub fn set_center(&mut self, center: [f64; 3]) {
        unsafe { sphere_set_center(self.sphere_ptr, center.as_ptr()) };
    }

    #[doc(alias = "GetCenter")]
    pub fn center(&self) -> [f64; 3] {
        unsafe {
            let mut center = [0.; 3];
            sphere_get_center(self.sphere_ptr, center.as_mut_ptr());
            center
        }
    }

    #[doc(alias = "PrintSelf")]
    pub fn print(&self, indent: usize) -> CString {
        unsafe {
            let char_ptr = sphere_print_self(self.sphere_ptr, indent);
            CString::from_raw(char_ptr.cast_mut())
        }
    }
}

impl Drop for Sphere {
    #[doc(alias = "Delete")]
    fn drop(&mut self) {
        unsafe { sphere_delete(self.sphere_ptr) };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn get_set_radius() {
        let mut sphere = Sphere::new();
        let r1 = sphere.radius();
        assert_abs_diff_eq!(r1, 0.5);
        sphere.set_radius(1.0);
        let r2 = sphere.radius();
        assert_abs_diff_eq!(r2, 1.0);
    }

    #[test]
    fn get_set_center() {
        let mut sphere = Sphere::new();
        let c1 = sphere.center();
        assert_abs_diff_eq!(c1, [0.0; 3]);
        sphere.set_center([1., 2., 3.]);
        let c2 = sphere.center();
        assert_abs_diff_eq!(c2, [1., 2., 3.]);
    }

    #[test]
    fn print_self() {
        let sphere = Sphere::new();
        let result = sphere.print(3);
        let string = result.to_str().unwrap().to_string();
        assert!(string.contains("   Debug"));
        assert!(string.contains("   Reference Count: 1"));
        assert!(string.contains("   Registered Events"));
        assert!(string.contains("   Transform"));
        assert!(string.contains("   Radius"));
        assert!(string.contains("   Center"));
    }
}
