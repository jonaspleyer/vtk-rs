#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_sphere.h");

        type vtkSphere;

        fn sphere_new() -> *mut vtkSphere;
        unsafe fn sphere_delete(sphere: *mut vtkSphere);
        fn sphere_delete_pin(sphere: Pin<&mut vtkSphere>);
        fn sphere_set_radius(sphere: Pin<&mut vtkSphere>, radius: f64);
        fn sphere_get_radius(sphere: Pin<&mut vtkSphere>) -> f64;
        fn sphere_set_center(spherr: Pin<&mut vtkSphere>, center: [f64; 3]);
        fn sphere_get_center(sphere: Pin<&mut vtkSphere>) -> [f64; 3];
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkSphere.html",
    @name Sphere, *mut ffi::vtkSphere,
    @new ffi::sphere_new,
    // @clone ffi::poly_data_clone,
    @delete ffi::sphere_delete
);

crate::inherit!(Sphere vtkImplicitFunction);

impl Sphere {
    #[doc(alias = "SetRadius")]
    pub fn set_radius(&mut self, radius: f64) {
        let pinned = unsafe { core::pin::Pin::new_unchecked(&mut *self.ptr) };
        ffi::sphere_set_radius(pinned, radius);
    }

    #[doc(alias = "GetRadius")]
    pub fn radius(&self) -> f64 {
        let pinned = unsafe { core::pin::Pin::new_unchecked(&mut *self.ptr) };
        ffi::sphere_get_radius(pinned)
    }

    #[doc(alias = "SetCenter")]
    pub fn set_center(&mut self, center: [f64; 3]) {
        let pinned = unsafe { core::pin::Pin::new_unchecked(&mut *self.ptr) };
        ffi::sphere_set_center(pinned, center);
    }

    #[doc(alias = "GetCenter")]
    pub fn get_center(&self) -> [f64; 3] {
        let pinned = unsafe { core::pin::Pin::new_unchecked(&mut *self.ptr) };
        ffi::sphere_get_center(pinned)
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
        let c1 = sphere.get_center();
        assert_abs_diff_eq!(c1, [0.0; 3]);
        sphere.set_center([1., 2., 3.]);
        let c2 = sphere.get_center();
        assert_abs_diff_eq!(c2, [1., 2., 3.]);
    }

    /* #[test]
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
    }*/
}
