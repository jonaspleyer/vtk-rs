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
    pub fn New() -> Self {
        Self {
            sphere_ptr: unsafe { sphere_new() },
        }
    }

    pub fn SetRadius(&mut self, radius: f64) {
        unsafe { sphere_set_radius(self.sphere_ptr, radius) };
    }

    pub fn GetRadius(&self) -> f64 {
        unsafe { sphere_get_radius(self.sphere_ptr) }
    }

    pub fn SetCenter(&mut self, center: [f64; 3]) {
        unsafe { sphere_set_center(self.sphere_ptr, center.as_ptr()) };
    }

    pub fn GetCenter(&self) -> [f64; 3] {
        unsafe {
            let mut center = [0.; 3];
            sphere_get_center(self.sphere_ptr, center.as_mut_ptr());
            center
        }
    }

    pub fn PrintSelf(&self, indent: usize) -> CString {
        unsafe {
            let char_ptr = sphere_print_self(self.sphere_ptr, indent);
            CString::from_raw(char_ptr.cast_mut())
        }
    }
}

impl Drop for Sphere {
    fn drop(&mut self) {
        unsafe { sphere_delete(self.sphere_ptr) };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn GetSetRadius() {
        let mut sphere = Sphere::New();
        let r1 = sphere.GetRadius();
        assert_abs_diff_eq!(r1, 0.5);
        sphere.SetRadius(1.0);
        let r2 = sphere.GetRadius();
        assert_abs_diff_eq!(r2, 1.0);
    }

    #[test]
    fn GetSetCenter() {
        let mut sphere = Sphere::New();
        let c1 = sphere.GetCenter();
        assert_abs_diff_eq!(c1, [0.0; 3]);
        sphere.SetCenter([1., 2., 3.]);
        let c2 = sphere.GetCenter();
        assert_abs_diff_eq!(c2, [1., 2., 3.]);
    }

    #[test]
    fn PrintSelf() {
        let sphere = Sphere::New();
        let result = sphere.PrintSelf(3);
        let string = result.to_str().unwrap().to_string();
        assert!(string.contains("   Debug"));
        assert!(string.contains("   Reference Count: 1"));
        assert!(string.contains("   Registered Events"));
        assert!(string.contains("   Transform"));
        assert!(string.contains("   Radius"));
        assert!(string.contains("   Center"));
    }
}
