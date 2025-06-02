#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_sphere_source.h");

        type vtkSphereSource;

        fn vtk_sphere_source_new() -> *mut vtkSphereSource;
        fn vtk_sphere_source_delete(ptr: Pin<&mut vtkSphereSource>);
        fn vtk_sphere_source_set_radius(sphere_source: Pin<&mut vtkSphereSource>, radius: f64);
        fn vtk_sphere_source_get_radius(sphere_source: &vtkSphereSource) -> f64;
        fn vtk_sphere_source_set_center(sphere_source: Pin<&mut vtkSphereSource>, center: [f64; 3]);
        fn vtk_sphere_source_get_center(sphere_source: &vtkSphereSource) -> [f64; 3];
        fn vtk_sphere_source_set_phi_resolution(
            sphere_source: Pin<&mut vtkSphereSource>,
            resolution: i64,
        );
        fn vtk_sphere_source_get_phi_resolution(sphere_source: &vtkSphereSource) -> i64;
        fn vtk_sphere_source_set_theta_resolution(
            sphere_source: Pin<&mut vtkSphereSource>,
            resolution: i64,
        );
        fn vtk_sphere_source_get_theta_resolution(sphere_source: &vtkSphereSource) -> i64;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkNamedColors.html",
    @name SphereSource, ffi::vtkSphereSource,
    @new ffi::vtk_sphere_source_new,
    @delete ffi::vtk_sphere_source_delete,
    @inherit vtkPolyDataAlgorithm
);

impl SphereSource {
    #[doc(alias = "SetRadius")]
    pub fn set_radius(&mut self, radius: f64) {
        ffi::vtk_sphere_source_set_radius(self.ptr.as_mut(), radius)
    }

    #[doc(alias = "GetRadius")]
    pub fn get_radius(&self) -> f64 {
        ffi::vtk_sphere_source_get_radius(&self.ptr.as_ref())
    }

    #[doc(alias = "SetCenter")]
    pub fn set_center(&mut self, center: [f64; 3]) {
        ffi::vtk_sphere_source_set_center(self.ptr.as_mut(), center)
    }

    #[doc(alias = "GetCenter")]
    pub fn get_center(&self) -> [f64; 3] {
        ffi::vtk_sphere_source_get_center(&self.ptr.as_ref())
    }

    #[doc(alias = "GetPhiResolution")]
    pub fn get_phi_resolution(&self) -> i64 {
        ffi::vtk_sphere_source_get_phi_resolution(&self.ptr.as_ref())
    }

    #[doc(alias = "SetPhiResolution")]
    pub fn set_phi_resolution(&mut self, phi_resolution: i64) {
        ffi::vtk_sphere_source_set_phi_resolution(self.ptr.as_mut(), phi_resolution)
    }

    #[doc(alias = "GetThetaResolution")]
    pub fn get_theta_resolution(&self) -> i64 {
        ffi::vtk_sphere_source_get_theta_resolution(&self.ptr.as_ref())
    }

    #[doc(alias = "SetThetaResolution")]
    pub fn set_theta_resolution(&mut self, theta_resolution: i64) {
        ffi::vtk_sphere_source_set_theta_resolution(self.ptr.as_mut(), theta_resolution)
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
    fn get_set_resolution() {
        let mut sphere = SphereSource::new();
        sphere.set_theta_resolution(20);
        assert_eq!(sphere.get_theta_resolution(), 20);
        sphere.set_phi_resolution(245);
        assert_eq!(sphere.get_phi_resolution(), 245);
    }

    #[test]
    fn print_self() {
        use crate::vtk_object_base::*;
        let sphere = SphereSource::new();
        let string = sphere.print_self(3);
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
