/// computes an interpolating spline using a
///
/// a Cardinal basis.
///
///
/// vtkCardinalSpline is a concrete implementation of vtkSpline using a
/// Cardinal basis.
///
/// @sa
/// vtkSpline vtkKochanekSpline
#[allow(non_camel_case_types)]
pub struct vtkCardinalSpline(*mut core::ffi::c_void);
impl vtkCardinalSpline {
    /// Creates a new [vtkCardinalSpline] wrapped inside `vtkNew`
    #[doc(alias = "vtkCardinalSpline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCardinalSpline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCardinalSpline_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCardinalSpline_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCardinalSpline_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCardinalSpline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCardinalSpline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCardinalSpline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCardinalSpline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCardinalSpline_create_drop() {
    let obj = vtkCardinalSpline::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCardinalSpline(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// computes an interpolating spline using a Kochanek basis.
///
///
/// Implements the Kochanek interpolating spline described in: Kochanek, D.,
/// Bartels, R., "Interpolating Splines with Local Tension, Continuity, and
/// Bias Control," Computer Graphics, vol. 18, no. 3, pp. 33-41, July 1984.
/// These splines give the user more control over the shape of the curve than
/// the cardinal splines implemented in vtkCardinalSpline. Three parameters
/// can be specified. All have a range from -1 to 1.
///
/// Tension controls how sharply the curve bends at an input point. A
/// value of -1 produces more slack in the curve. A value of 1 tightens
/// the curve.
///
/// Continuity controls the continuity of the first derivative at input
/// points.
///
/// Bias controls the direction of the curve at it passes through an input
/// point. A value of -1 undershoots the point while a value of 1
/// overshoots the point.
///
/// These three parameters give the user broad control over the shape of
/// the interpolating spline. The original Kochanek paper describes the
/// effects nicely and is recommended reading.
///
/// @sa
/// vtkSpline vtkCardinalSpline
#[allow(non_camel_case_types)]
pub struct vtkKochanekSpline(*mut core::ffi::c_void);
impl vtkKochanekSpline {
    /// Creates a new [vtkKochanekSpline] wrapped inside `vtkNew`
    #[doc(alias = "vtkKochanekSpline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkKochanekSpline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkKochanekSpline_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkKochanekSpline_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkKochanekSpline_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkKochanekSpline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkKochanekSpline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkKochanekSpline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkKochanekSpline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkKochanekSpline_create_drop() {
    let obj = vtkKochanekSpline::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkKochanekSpline(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a Bohemian dome.
///
///
/// vtkParametricBohemianDome generates a parametric Bohemian dome. The Bohemian
/// dome is a quartic surface, and is described in much better detail at
/// <a href="https://www.math.hmc.edu/math142-01/mellon/curves_and_surfaces/surfaces/bohdom.html">HMC
/// page</a>.
/// @warning
/// I haven't set any restrictions on the A, B, or C values.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricBohemianDome(*mut core::ffi::c_void);
impl vtkParametricBohemianDome {
    /// Creates a new [vtkParametricBohemianDome] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricBohemianDome")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricBohemianDome_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricBohemianDome_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricBohemianDome_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricBohemianDome_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricBohemianDome {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricBohemianDome {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricBohemianDome_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricBohemianDome_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricBohemianDome_create_drop() {
    let obj = vtkParametricBohemianDome::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricBohemianDome(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Bour's minimal surface.
///
///
/// vtkParametricBour generates Bour's minimal surface parametrically. More
/// information can be found at
/// <a href="http://en.wikipedia.org/wiki/Bour%27s_minimal_surface">Wikipedia</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricBour(*mut core::ffi::c_void);
impl vtkParametricBour {
    /// Creates a new [vtkParametricBour] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricBour")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricBour_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricBour_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricBour_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricBour_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricBour {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricBour {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricBour_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricBour_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricBour_create_drop() {
    let obj = vtkParametricBour::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricBour(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Boy's surface.
///
///
/// vtkParametricBoy generates Boy's surface.
/// This is a Model of the projective plane without singularities.
/// It was found by Werner Boy on assignment from David Hilbert.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricBoy(*mut core::ffi::c_void);
impl vtkParametricBoy {
    /// Creates a new [vtkParametricBoy] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricBoy")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricBoy_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricBoy_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricBoy_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricBoy_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricBoy {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricBoy {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricBoy_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricBoy_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricBoy_create_drop() {
    let obj = vtkParametricBoy::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricBoy(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Catalan's minimal surface.
///
///
/// vtkParametricCatalanMinimal generates Catalan's minimal surface
/// parametrically. This minimal surface contains the cycloid as a geodesic.
/// More information about it can be found at
/// <a href="https://en.wikipedia.org/wiki/Catalan%27s_minimal_surface">Wikipedia</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricCatalanMinimal(*mut core::ffi::c_void);
impl vtkParametricCatalanMinimal {
    /// Creates a new [vtkParametricCatalanMinimal] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricCatalanMinimal")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricCatalanMinimal_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricCatalanMinimal_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricCatalanMinimal_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricCatalanMinimal_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricCatalanMinimal {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricCatalanMinimal {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricCatalanMinimal_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricCatalanMinimal_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricCatalanMinimal_create_drop() {
    let obj = vtkParametricCatalanMinimal::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricCatalanMinimal(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate conic spiral surfaces that resemble sea-shells.
///
///
/// vtkParametricConicSpiral generates conic spiral surfaces. These can resemble sea shells, or
/// may look like a torus "eating" its own tail.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricConicSpiral(*mut core::ffi::c_void);
impl vtkParametricConicSpiral {
    /// Creates a new [vtkParametricConicSpiral] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricConicSpiral")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricConicSpiral_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricConicSpiral_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricConicSpiral_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricConicSpiral_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricConicSpiral {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricConicSpiral {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricConicSpiral_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricConicSpiral_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricConicSpiral_create_drop() {
    let obj = vtkParametricConicSpiral::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricConicSpiral(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a cross-cap.
///
///
/// vtkParametricCrossCap generates a cross-cap which is a
/// non-orientable self-intersecting single-sided surface.
/// This is one possible image of a projective plane in three-space.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricCrossCap(*mut core::ffi::c_void);
impl vtkParametricCrossCap {
    /// Creates a new [vtkParametricCrossCap] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricCrossCap")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricCrossCap_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricCrossCap_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricCrossCap_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricCrossCap_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricCrossCap {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricCrossCap {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricCrossCap_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricCrossCap_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricCrossCap_create_drop() {
    let obj = vtkParametricCrossCap::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricCrossCap(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Dini's surface.
///
///
/// vtkParametricDini generates Dini's surface.
/// Dini's surface is a surface that possesses constant negative
/// Gaussian curvature
///
/// For further information about this surface, please consult
/// https://en.wikipedia.org/wiki/Dini%27s_surface
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricDini(*mut core::ffi::c_void);
impl vtkParametricDini {
    /// Creates a new [vtkParametricDini] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricDini")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricDini_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricDini_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricDini_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricDini_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricDini {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricDini {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricDini_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricDini_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricDini_create_drop() {
    let obj = vtkParametricDini::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricDini(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate an ellipsoid.
///
///
/// vtkParametricEllipsoid generates an ellipsoid.
/// If all the radii are the same, we have a sphere.
/// An oblate spheroid occurs if RadiusX = RadiusY > RadiusZ.
/// Here the Z-axis forms the symmetry axis. To a first
/// approximation, this is the shape of the earth.
/// A prolate spheroid occurs if RadiusX = RadiusY < RadiusZ.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricEllipsoid(*mut core::ffi::c_void);
impl vtkParametricEllipsoid {
    /// Creates a new [vtkParametricEllipsoid] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricEllipsoid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricEllipsoid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricEllipsoid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricEllipsoid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricEllipsoid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricEllipsoid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricEllipsoid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricEllipsoid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricEllipsoid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricEllipsoid_create_drop() {
    let obj = vtkParametricEllipsoid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricEllipsoid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Enneper's surface.
///
///
/// vtkParametricEnneper generates Enneper's surface.
/// Enneper's surface is a a self-intersecting minimal surface
/// possessing constant negative Gaussian curvature
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricEnneper(*mut core::ffi::c_void);
impl vtkParametricEnneper {
    /// Creates a new [vtkParametricEnneper] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricEnneper")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricEnneper_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricEnneper_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricEnneper_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricEnneper_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricEnneper {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricEnneper {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricEnneper_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricEnneper_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricEnneper_create_drop() {
    let obj = vtkParametricEnneper::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricEnneper(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a figure-8 Klein bottle.
///
///
/// vtkParametricFigure8Klein generates a figure-8 Klein bottle.  A Klein bottle
/// is a closed surface with no interior and only one surface.  It is
/// unrealisable in 3 dimensions without intersecting surfaces.  It can be
/// realised in 4 dimensions by considering the map \f$F:R^2 \rightarrow R^4\f$  given by:
///
/// - \f$f(u,v) = ((r*cos(v)+a)*cos(u),(r*cos(v)+a)*sin(u),r*sin(v)*cos(u/2),r*sin(v)*sin(u/2))\f$
///
/// This representation of the immersion in \f$R^3\f$ is formed by taking two Mobius
/// strips and joining them along their boundaries, this is the so called
/// "Figure-8 Klein Bottle"
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricFigure8Klein(*mut core::ffi::c_void);
impl vtkParametricFigure8Klein {
    /// Creates a new [vtkParametricFigure8Klein] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricFigure8Klein")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricFigure8Klein_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricFigure8Klein_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricFigure8Klein_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricFigure8Klein_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricFigure8Klein {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricFigure8Klein {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricFigure8Klein_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricFigure8Klein_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricFigure8Klein_create_drop() {
    let obj = vtkParametricFigure8Klein::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricFigure8Klein(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Henneberg's minimal surface.
///
///
/// vtkParametricHenneberg generates Henneberg's minimal surface parametrically.
/// Henneberg's minimal surface is discussed further at
/// <a href="http://mathworld.wolfram.com/HennebergsMinimalSurface.html">Math World</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricHenneberg(*mut core::ffi::c_void);
impl vtkParametricHenneberg {
    /// Creates a new [vtkParametricHenneberg] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricHenneberg")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricHenneberg_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricHenneberg_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricHenneberg_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricHenneberg_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricHenneberg {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricHenneberg {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricHenneberg_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricHenneberg_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricHenneberg_create_drop() {
    let obj = vtkParametricHenneberg::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricHenneberg(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generates a "classical" representation of a Klein bottle.
///
///
/// vtkParametricKlein generates a "classical" representation of a Klein
/// bottle.  A Klein bottle is a closed surface with no interior and only one
/// surface.  It is unrealisable in 3 dimensions without intersecting
/// surfaces.  It can be
/// realised in 4 dimensions by considering the map \f$F:R^2 \rightarrow R^4\f$  given by:
///
/// - \f$f(u,v) = ((r*cos(v)+a)*cos(u),(r*cos(v)+a)*sin(u),r*sin(v)*cos(u/2),r*sin(v)*sin(u/2))\f$
///
/// The classical representation of the immersion in \f$R^3\f$ is returned by this function.
///
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricKlein(*mut core::ffi::c_void);
impl vtkParametricKlein {
    /// Creates a new [vtkParametricKlein] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricKlein")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricKlein_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricKlein_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricKlein_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricKlein_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricKlein {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricKlein {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricKlein_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricKlein_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricKlein_create_drop() {
    let obj = vtkParametricKlein::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricKlein(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Kuens' surface.
///
///
/// vtkParametricKuen generates Kuens' surface. This surface has a constant
/// negative gaussian curvature. For more information about this surface, see
/// Dr. O'Niell's page at the
/// <a href="http://www.math.ucla.edu/~bon/kuen.html">UCLA Mathematics Department</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricKuen(*mut core::ffi::c_void);
impl vtkParametricKuen {
    /// Creates a new [vtkParametricKuen] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricKuen")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricKuen_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricKuen_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricKuen_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricKuen_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricKuen {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricKuen {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricKuen_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricKuen_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricKuen_create_drop() {
    let obj = vtkParametricKuen::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricKuen(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a Mobius strip.
///
///
/// vtkParametricMobius generates a Mobius strip.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricMobius(*mut core::ffi::c_void);
impl vtkParametricMobius {
    /// Creates a new [vtkParametricMobius] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricMobius")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricMobius_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricMobius_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricMobius_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricMobius_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricMobius {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricMobius {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricMobius_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricMobius_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricMobius_create_drop() {
    let obj = vtkParametricMobius::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricMobius(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Plucker's conoid surface.
///
///
/// vtkParametricPluckerConoid generates Plucker's conoid surface parametrically.
/// Plucker's conoid is a ruled surface, named after Julius Plucker. It is
/// possible to set the number of folds in this class via the parameter 'N'.
///
/// For more information, see the Wikipedia page on
/// <a href="https://en.wikipedia.org/wiki/Pl%c3%bccker%27s_conoid">Plucker's Conoid</a>.
/// @warning
/// I haven't done any special checking on the number of folds parameter, N.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricPluckerConoid(*mut core::ffi::c_void);
impl vtkParametricPluckerConoid {
    /// Creates a new [vtkParametricPluckerConoid] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricPluckerConoid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricPluckerConoid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricPluckerConoid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricPluckerConoid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricPluckerConoid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricPluckerConoid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricPluckerConoid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricPluckerConoid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricPluckerConoid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricPluckerConoid_create_drop() {
    let obj = vtkParametricPluckerConoid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricPluckerConoid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a pseudosphere.
///
///
/// vtkParametricPseudosphere generates a parametric pseudosphere. The
/// pseudosphere is generated as a surface of revolution of the tractrix about
/// it's asymptote, and is a surface of constant negative Gaussian curvature.
/// You can find out more about this interesting surface at
/// <a href="http://mathworld.wolfram.com/Pseudosphere.html">Math World</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricPseudosphere(*mut core::ffi::c_void);
impl vtkParametricPseudosphere {
    /// Creates a new [vtkParametricPseudosphere] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricPseudosphere")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricPseudosphere_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricPseudosphere_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricPseudosphere_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricPseudosphere_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricPseudosphere {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricPseudosphere {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricPseudosphere_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricPseudosphere_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricPseudosphere_create_drop() {
    let obj = vtkParametricPseudosphere::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricPseudosphere(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a surface covered with randomly placed hills.
///
///
/// vtkParametricRandomHills generates a surface covered with randomly placed
/// hills. Hills will vary in shape and height since the presence
/// of nearby hills will contribute to the shape and height of a given hill.
/// An option is provided for placing hills on a regular grid on the surface.
/// In this case the hills will all have the same shape and height.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricRandomHills(*mut core::ffi::c_void);
impl vtkParametricRandomHills {
    /// Creates a new [vtkParametricRandomHills] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricRandomHills")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricRandomHills_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricRandomHills_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricRandomHills_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricRandomHills_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricRandomHills {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricRandomHills {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricRandomHills_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricRandomHills_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricRandomHills_create_drop() {
    let obj = vtkParametricRandomHills::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricRandomHills(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate Steiner's Roman Surface.
///
///
/// vtkParametricRoman generates Steiner's Roman Surface.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricRoman(*mut core::ffi::c_void);
impl vtkParametricRoman {
    /// Creates a new [vtkParametricRoman] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricRoman")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricRoman_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricRoman_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricRoman_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricRoman_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricRoman {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricRoman {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricRoman_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricRoman_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricRoman_create_drop() {
    let obj = vtkParametricRoman::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricRoman(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// parametric function for 1D interpolating splines
///
///
/// vtkParametricSpline is a parametric function for 1D interpolating splines.
/// vtkParametricSpline maps the single parameter u into a 3D point (x,y,z)
/// using three instances of interpolating splines.  This family of 1D splines
/// is guaranteed to be parameterized in the interval [0,1].  Attempting to
/// evaluate outside this interval will cause the parameter u to be clamped in
/// the range [0,1].
///
/// When constructed, this class creates instances of vtkCardinalSpline for
/// each of the x-y-z coordinates. The user may choose to replace these with
/// their own instances of subclasses of vtkSpline.
///
/// @warning
/// If you wish to tessellate the spline, use the class
/// vtkParametricFunctionSource.
///
/// @sa
/// vtkSpline vtkKochanekSpline vtkCardinalSpline
#[allow(non_camel_case_types)]
pub struct vtkParametricSpline(*mut core::ffi::c_void);
impl vtkParametricSpline {
    /// Creates a new [vtkParametricSpline] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricSpline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricSpline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricSpline_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricSpline_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricSpline_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricSpline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricSpline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricSpline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricSpline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricSpline_create_drop() {
    let obj = vtkParametricSpline::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricSpline(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a superellipsoid.
///
///
/// vtkParametricSuperEllipsoid generates a superellipsoid.  A superellipsoid
/// is a versatile primitive that is controlled by two parameters n1 and
/// n2. As special cases it can represent a sphere, square box, and closed
/// cylindrical can.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// Also see: http://paulbourke.net/geometry/superellipse/
///
/// @warning
/// Care needs to be taken specifying the bounds correctly. You may need to
/// carefully adjust MinimumU, MinimumV, MaximumU, MaximumV.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricSuperEllipsoid(*mut core::ffi::c_void);
impl vtkParametricSuperEllipsoid {
    /// Creates a new [vtkParametricSuperEllipsoid] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricSuperEllipsoid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricSuperEllipsoid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricSuperEllipsoid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricSuperEllipsoid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricSuperEllipsoid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricSuperEllipsoid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricSuperEllipsoid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricSuperEllipsoid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricSuperEllipsoid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricSuperEllipsoid_create_drop() {
    let obj = vtkParametricSuperEllipsoid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricSuperEllipsoid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a supertoroid.
///
///
/// vtkParametricSuperToroid generates a supertoroid.  Essentially a
/// supertoroid is a torus with the sine and cosine terms raised to a power.
/// A supertoroid is a versatile primitive that is controlled by four
/// parameters r0, r1, n1 and n2. r0, r1 determine the type of torus whilst
/// the value of n1 determines the shape of the torus ring and n2 determines
/// the shape of the cross section of the ring. It is the different values of
/// these powers which give rise to a family of 3D shapes that are all
/// basically toroidal in shape.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// Also see: http://paulbourke.net/geometry/torus/#super.
///
/// @warning
/// Care needs to be taken specifying the bounds correctly. You may need to
/// carefully adjust MinimumU, MinimumV, MaximumU, MaximumV.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricSuperToroid(*mut core::ffi::c_void);
impl vtkParametricSuperToroid {
    /// Creates a new [vtkParametricSuperToroid] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricSuperToroid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricSuperToroid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricSuperToroid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricSuperToroid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricSuperToroid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricSuperToroid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricSuperToroid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricSuperToroid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricSuperToroid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricSuperToroid_create_drop() {
    let obj = vtkParametricSuperToroid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricSuperToroid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generate a torus.
///
///
/// vtkParametricTorus generates a torus.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricTorus(*mut core::ffi::c_void);
impl vtkParametricTorus {
    /// Creates a new [vtkParametricTorus] wrapped inside `vtkNew`
    #[doc(alias = "vtkParametricTorus")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricTorus_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkParametricTorus_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkParametricTorus_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkParametricTorus_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkParametricTorus {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricTorus {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricTorus_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricTorus_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricTorus_create_drop() {
    let obj = vtkParametricTorus::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkParametricTorus(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
