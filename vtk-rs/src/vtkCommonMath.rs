/// nonlinear optimization with a simplex
///
///
/// vtkAmoebaMinimizer will modify a set of parameters in order to find
/// the minimum of a specified function.  The method used is commonly
/// known as the amoeba method, it constructs an n-dimensional simplex
/// in parameter space (i.e. a tetrahedron if the number or parameters
/// is 3) and moves the vertices around parameter space until a local
/// minimum is found.  The amoeba method is robust, reasonably efficient,
/// but is not guaranteed to find the global minimum if several local
/// minima exist.
#[allow(non_camel_case_types)]
pub struct vtkAmoebaMinimizer(*mut core::ffi::c_void);
impl vtkAmoebaMinimizer {
    /// Creates a new [vtkAmoebaMinimizer] wrapped inside `vtkNew`
    #[doc(alias = "vtkAmoebaMinimizer")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAmoebaMinimizer_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAmoebaMinimizer_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAmoebaMinimizer_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAmoebaMinimizer_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAmoebaMinimizer {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAmoebaMinimizer {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAmoebaMinimizer_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAmoebaMinimizer_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAmoebaMinimizer_create_drop() {
    let obj = vtkAmoebaMinimizer::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAmoebaMinimizer(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// perform Discrete Fourier Transforms
///
///
/// vtkFFT provides methods to perform Discrete Fourier Transforms (DFT).
/// These include providing forward and reverse Fourier transforms.
/// The current implementation uses the third-party library kissfft.
///
/// The terminology tries to follow the Numpy terminology, that is :
/// - Fft means the Fast Fourier Transform algorithm
/// - Prefix `R` stands for Real (meaning optimized function for real inputs)
/// - Prefix `I` stands for Inverse
#[allow(non_camel_case_types)]
pub struct vtkFFT(*mut core::ffi::c_void);
impl vtkFFT {
    /// Creates a new [vtkFFT] wrapped inside `vtkNew`
    #[doc(alias = "vtkFFT")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFFT_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkFFT_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkFFT_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkFFT_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkFFT {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFFT {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFFT_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFFT_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFFT_create_drop() {
    let obj = vtkFFT::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkFFT(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// represent and manipulate 3x3 transformation matrices
///
///
/// vtkMatrix3x3 is a class to represent and manipulate 3x3 matrices.
/// Specifically, it is designed to work on 3x3 transformation matrices
/// found in 2D rendering using homogeneous coordinates [x y w].
///
/// @sa
/// vtkTransform2D
#[allow(non_camel_case_types)]
pub struct vtkMatrix3x3(*mut core::ffi::c_void);
impl vtkMatrix3x3 {
    /// Creates a new [vtkMatrix3x3] wrapped inside `vtkNew`
    #[doc(alias = "vtkMatrix3x3")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMatrix3x3_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMatrix3x3_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMatrix3x3_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMatrix3x3_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMatrix3x3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMatrix3x3 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMatrix3x3_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMatrix3x3_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMatrix3x3_create_drop() {
    let obj = vtkMatrix3x3::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMatrix3x3(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// represent and manipulate 4x4 transformation matrices
///
///
/// vtkMatrix4x4 is a class to represent and manipulate 4x4 matrices.
/// Specifically, it is designed to work on 4x4 transformation matrices
/// found in 3D rendering using homogeneous coordinates [x y z w].
/// Many of the methods take an array of 16 doubles in row-major format.
/// Note that OpenGL stores matrices in column-major format, so the matrix
/// contents must be transposed when they are moved between OpenGL and VTK.
/// @sa
/// vtkTransform
#[allow(non_camel_case_types)]
pub struct vtkMatrix4x4(*mut core::ffi::c_void);
impl vtkMatrix4x4 {
    /// Creates a new [vtkMatrix4x4] wrapped inside `vtkNew`
    #[doc(alias = "vtkMatrix4x4")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMatrix4x4_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMatrix4x4_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMatrix4x4_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMatrix4x4_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMatrix4x4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMatrix4x4 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMatrix4x4_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMatrix4x4_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMatrix4x4_create_drop() {
    let obj = vtkMatrix4x4::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMatrix4x4(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// polynomial solvers
///
///
/// vtkPolynomialSolversUnivariate provides solvers for
/// univariate polynomial equations with real coefficients.
/// The Tartaglia-Cardan and Ferrari solvers work on polynomials of fixed
/// degree 3 and 4, respectively.
/// The Lin-Bairstow and Sturm solvers work on polynomials of arbitrary
/// degree. The Sturm solver is the most robust solver but only reports
/// roots within an interval and does not report multiplicities.
/// The Lin-Bairstow solver reports multiplicities.
///
/// For difficult polynomials, you may wish to use FilterRoots to
/// eliminate some of the roots reported by the Sturm solver.
/// FilterRoots evaluates the derivatives near each root to
/// eliminate cases where a local minimum or maximum is close
/// to zero.
///
/// @par Thanks:
/// Thanks to Philippe Pebay, Korben Rusek, David Thompson, and Maurice Rojas
/// for implementing these solvers.
#[allow(non_camel_case_types)]
pub struct vtkPolynomialSolversUnivariate(*mut core::ffi::c_void);
impl vtkPolynomialSolversUnivariate {
    /// Creates a new [vtkPolynomialSolversUnivariate] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolynomialSolversUnivariate")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolynomialSolversUnivariate_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolynomialSolversUnivariate_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolynomialSolversUnivariate_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolynomialSolversUnivariate_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolynomialSolversUnivariate {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolynomialSolversUnivariate {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolynomialSolversUnivariate_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolynomialSolversUnivariate_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolynomialSolversUnivariate_create_drop() {
    let obj = vtkPolynomialSolversUnivariate::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolynomialSolversUnivariate(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// interpolate a quaternion
///
///
/// This class is used to interpolate a series of quaternions representing
/// the rotations of a 3D object.  The interpolation may be linear in form
/// (using spherical linear interpolation SLERP), or via spline interpolation
/// (using SQUAD). In either case the interpolation is specialized to
/// quaternions since the interpolation occurs on the surface of the unit
/// quaternion sphere.
///
/// To use this class, specify at least two pairs of (t,q[4]) with the
/// AddQuaternion() method.  Next interpolate the tuples with the
/// InterpolateQuaternion(t,q[4]) method, where "t" must be in the range of
/// (t_min,t_max) parameter values specified by the AddQuaternion() method (t
/// is clamped otherwise), and q[4] is filled in by the method.
///
/// There are several important background references. Ken Shoemake described
/// the practical application of quaternions for the interpolation of rotation
/// (K. Shoemake, "Animating rotation with quaternion curves", Computer
/// Graphics (Siggraph '85) 19(3):245--254, 1985). Another fine reference
/// (available on-line) is E. B. Dam, M. Koch, and M. Lillholm, Technical
/// Report DIKU-TR-98/5, Dept. of Computer Science, University of Copenhagen,
/// Denmark.
///
/// @warning
/// Note that for two or less quaternions, Slerp (linear) interpolation is
/// performed even if spline interpolation is requested. Also, the tangents to
/// the first and last segments of spline interpolation are (arbitrarily)
/// defined by repeating the first and last quaternions.
///
/// @warning
/// There are several methods particular to quaternions (norms, products,
/// etc.) implemented interior to this class. These may be moved to a separate
/// quaternion class at some point.
///
/// @sa
/// vtkQuaternion
#[allow(non_camel_case_types)]
pub struct vtkQuaternionInterpolator(*mut core::ffi::c_void);
impl vtkQuaternionInterpolator {
    /// Creates a new [vtkQuaternionInterpolator] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuaternionInterpolator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuaternionInterpolator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuaternionInterpolator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuaternionInterpolator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuaternionInterpolator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuaternionInterpolator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuaternionInterpolator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuaternionInterpolator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuaternionInterpolator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuaternionInterpolator_create_drop() {
    let obj = vtkQuaternionInterpolator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuaternionInterpolator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Integrate an initial value problem using 2nd
///
/// order Runge-Kutta method.
///
///
/// This is a concrete sub-class of vtkInitialValueProblemSolver.
/// It uses a 2nd order Runge-Kutta method to obtain the values of
/// a set of functions at the next time step.
///
/// @sa
/// vtkInitialValueProblemSolver vtkRungeKutta4 vtkRungeKutta45 vtkFunctionSet
#[allow(non_camel_case_types)]
pub struct vtkRungeKutta2(*mut core::ffi::c_void);
impl vtkRungeKutta2 {
    /// Creates a new [vtkRungeKutta2] wrapped inside `vtkNew`
    #[doc(alias = "vtkRungeKutta2")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRungeKutta2_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkRungeKutta2_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkRungeKutta2_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkRungeKutta2_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkRungeKutta2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRungeKutta2 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRungeKutta2_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRungeKutta2_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRungeKutta2_create_drop() {
    let obj = vtkRungeKutta2::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkRungeKutta2(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Integrate an initial value problem using 4th
///
/// order Runge-Kutta method.
///
///
/// This is a concrete sub-class of vtkInitialValueProblemSolver.
/// It uses a 4th order Runge-Kutta method to obtain the values of
/// a set of functions at the next time step.
///
/// @sa
/// vtkInitialValueProblemSolver vtkRungeKutta45 vtkRungeKutta2 vtkFunctionSet
#[allow(non_camel_case_types)]
pub struct vtkRungeKutta4(*mut core::ffi::c_void);
impl vtkRungeKutta4 {
    /// Creates a new [vtkRungeKutta4] wrapped inside `vtkNew`
    #[doc(alias = "vtkRungeKutta4")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRungeKutta4_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkRungeKutta4_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkRungeKutta4_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkRungeKutta4_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkRungeKutta4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRungeKutta4 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRungeKutta4_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRungeKutta4_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRungeKutta4_create_drop() {
    let obj = vtkRungeKutta4::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkRungeKutta4(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Integrate an initial value problem using 5th
///
/// order Runge-Kutta method with adaptive stepsize control.
///
///
/// This is a concrete sub-class of vtkInitialValueProblemSolver.
/// It uses a 5th order Runge-Kutta method with stepsize control to obtain
/// the values of a set of functions at the next time step. The stepsize
/// is adjusted by calculating an estimated error using an embedded 4th
/// order Runge-Kutta formula:
/// Press, W. H. et al., 1992, Numerical Recipes in Fortran, Second
/// Edition, Cambridge University Press
/// Cash, J.R. and Karp, A.H. 1990, ACM Transactions on Mathematical
/// Software, vol 16, pp 201-222
///
/// @sa
/// vtkInitialValueProblemSolver vtkRungeKutta4 vtkRungeKutta2 vtkFunctionSet
#[allow(non_camel_case_types)]
pub struct vtkRungeKutta45(*mut core::ffi::c_void);
impl vtkRungeKutta45 {
    /// Creates a new [vtkRungeKutta45] wrapped inside `vtkNew`
    #[doc(alias = "vtkRungeKutta45")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRungeKutta45_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkRungeKutta45_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkRungeKutta45_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkRungeKutta45_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkRungeKutta45 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRungeKutta45 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRungeKutta45_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRungeKutta45_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRungeKutta45_create_drop() {
    let obj = vtkRungeKutta45::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkRungeKutta45(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
