/// cylindrical to rectangular coords and back
///
///
/// vtkCylindricalTransform will convert (r,theta,z) coordinates to
/// (x,y,z) coordinates and back again.  The angles are given in radians.
/// By default, it converts cylindrical coordinates to rectangular, but
/// GetInverse() returns a transform that will do the opposite.  The
/// equation that is used is x = r*cos(theta), y = r*sin(theta), z = z.
/// @warning
/// This transform is not well behaved along the line x=y=0 (i.e. along
/// the z-axis)
/// @sa
/// vtkSphericalTransform vtkGeneralTransform
#[allow(non_camel_case_types)]
pub struct vtkCylindricalTransform(*mut core::ffi::c_void);
impl vtkCylindricalTransform {
    /// Creates a new [vtkCylindricalTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkCylindricalTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCylindricalTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCylindricalTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCylindricalTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCylindricalTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCylindricalTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCylindricalTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCylindricalTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCylindricalTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCylindricalTransform_create_drop() {
    let obj = vtkCylindricalTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCylindricalTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// allows operations on any transforms
///
///
/// vtkGeneralTransform is like vtkTransform and vtkPerspectiveTransform,
/// but it will work with any vtkAbstractTransform as input.  It is
/// not as efficient as the other two, however, because arbitrary
/// transformations cannot be concatenated by matrix multiplication.
/// Transform concatenation is simulated by passing each input point
/// through each transform in turn.
/// @sa
/// vtkTransform vtkPerspectiveTransform
#[allow(non_camel_case_types)]
pub struct vtkGeneralTransform(*mut core::ffi::c_void);
impl vtkGeneralTransform {
    /// Creates a new [vtkGeneralTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkGeneralTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGeneralTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGeneralTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGeneralTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGeneralTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGeneralTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGeneralTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGeneralTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGeneralTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGeneralTransform_create_drop() {
    let obj = vtkGeneralTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGeneralTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a transform that doesn't do anything
///
///
/// vtkIdentityTransform is a transformation which will simply pass coordinate
/// data unchanged.  All other transform types can also do this, however,
/// the vtkIdentityTransform does so with much greater efficiency.
/// @sa
/// vtkLinearTransform
#[allow(non_camel_case_types)]
pub struct vtkIdentityTransform(*mut core::ffi::c_void);
impl vtkIdentityTransform {
    /// Creates a new [vtkIdentityTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkIdentityTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIdentityTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkIdentityTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkIdentityTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkIdentityTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkIdentityTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIdentityTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIdentityTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIdentityTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIdentityTransform_create_drop() {
    let obj = vtkIdentityTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkIdentityTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a linear transform specified by two corresponding point sets
///
///
/// A vtkLandmarkTransform is defined by two sets of landmarks, the
/// transform computed gives the best fit mapping one onto the other, in a
/// least squares sense. The indices are taken to correspond, so point 1
/// in the first set will get mapped close to point 1 in the second set,
/// etc. Call SetSourceLandmarks and SetTargetLandmarks to specify the two
/// sets of landmarks, ensure they have the same number of points.
/// @warning
/// Whenever you add, subtract, or set points you must call Modified()
/// on the vtkPoints object, or the transformation might not update.
/// @sa
/// vtkLinearTransform
#[allow(non_camel_case_types)]
pub struct vtkLandmarkTransform(*mut core::ffi::c_void);
impl vtkLandmarkTransform {
    /// Creates a new [vtkLandmarkTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkLandmarkTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLandmarkTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLandmarkTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLandmarkTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLandmarkTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLandmarkTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLandmarkTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLandmarkTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLandmarkTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLandmarkTransform_create_drop() {
    let obj = vtkLandmarkTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLandmarkTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// convert a matrix to a transform
///
///
/// This is a very simple class which allows a vtkMatrix4x4 to be used in
/// place of a vtkHomogeneousTransform or vtkAbstractTransform.  For example,
/// if you use it as a proxy between a matrix and vtkTransformPolyDataFilter
/// then any modifications to the matrix will automatically be reflected in
/// the output of the filter.
/// @sa
/// vtkPerspectiveTransform vtkMatrix4x4 vtkMatrixToLinearTransform
#[allow(non_camel_case_types)]
pub struct vtkMatrixToHomogeneousTransform(*mut core::ffi::c_void);
impl vtkMatrixToHomogeneousTransform {
    /// Creates a new [vtkMatrixToHomogeneousTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkMatrixToHomogeneousTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMatrixToHomogeneousTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMatrixToHomogeneousTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMatrixToHomogeneousTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMatrixToHomogeneousTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMatrixToHomogeneousTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMatrixToHomogeneousTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMatrixToHomogeneousTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMatrixToHomogeneousTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMatrixToHomogeneousTransform_create_drop() {
    let obj = vtkMatrixToHomogeneousTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMatrixToHomogeneousTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// convert a matrix to a transform
///
///
/// This is a very simple class which allows a vtkMatrix4x4 to be used in
/// place of a vtkLinearTransform or vtkAbstractTransform.  For example,
/// if you use it as a proxy between a matrix and vtkTransformPolyDataFilter
/// then any modifications to the matrix will automatically be reflected in
/// the output of the filter.
/// @sa
/// vtkTransform vtkMatrix4x4 vtkMatrixToHomogeneousTransform
#[allow(non_camel_case_types)]
pub struct vtkMatrixToLinearTransform(*mut core::ffi::c_void);
impl vtkMatrixToLinearTransform {
    /// Creates a new [vtkMatrixToLinearTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkMatrixToLinearTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMatrixToLinearTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMatrixToLinearTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMatrixToLinearTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMatrixToLinearTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMatrixToLinearTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMatrixToLinearTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMatrixToLinearTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMatrixToLinearTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMatrixToLinearTransform_create_drop() {
    let obj = vtkMatrixToLinearTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMatrixToLinearTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// describes a 4x4 matrix transformation
///
///
/// A vtkPerspectiveTransform can be used to describe the full range of
/// homogeneous transformations.  It was designed in particular
/// to describe a camera-view of a scene.
/// <P>The order in which you set up the display coordinates (via
/// AdjustZBuffer() and AdjustViewport()), the projection (via Perspective(),
/// Frustum(), or Ortho()) and the camera view (via SetupCamera()) are
/// important.  If the transform is in PreMultiply mode, which is the
/// default, set the Viewport and ZBuffer first, then the projection, and
/// finally the camera view.  Once the view is set up, the Translate
/// and Rotate methods can be used to move the camera around in world
/// coordinates.  If the Oblique() or Stereo() methods are used, they
/// should be called just before SetupCamera().
/// <P>In PostMultiply mode, you must perform all transformations
/// in the opposite order.  This is necessary, for example, if you
/// already have a perspective transformation set up but must adjust
/// the viewport.  Another example is if you have a view transformation,
/// and wish to perform translations and rotations in the camera's
/// coordinate system rather than in world coordinates.
/// <P>The SetInput and Concatenate methods can be used to create
/// a transformation pipeline with vtkPerspectiveTransform.  See vtkTransform
/// for more information on the transformation pipeline.
/// @sa
/// vtkGeneralTransform vtkTransform vtkMatrix4x4 vtkCamera
#[allow(non_camel_case_types)]
pub struct vtkPerspectiveTransform(*mut core::ffi::c_void);
impl vtkPerspectiveTransform {
    /// Creates a new [vtkPerspectiveTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkPerspectiveTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPerspectiveTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPerspectiveTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPerspectiveTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPerspectiveTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPerspectiveTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPerspectiveTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPerspectiveTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPerspectiveTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPerspectiveTransform_create_drop() {
    let obj = vtkPerspectiveTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPerspectiveTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// spherical to rectangular coords and back
///
///
/// vtkSphericalTransform will convert (r,phi,theta) coordinates to
/// (x,y,z) coordinates and back again.  The angles are given in radians.
/// By default, it converts spherical coordinates to rectangular, but
/// GetInverse() returns a transform that will do the opposite.  The equation
/// that is used is x = r*sin(phi)*cos(theta), y = r*sin(phi)*sin(theta),
/// z = r*cos(phi).
/// @warning
/// This transform is not well behaved along the line x=y=0 (i.e. along
/// the z-axis)
/// @sa
/// vtkCylindricalTransform vtkGeneralTransform
#[allow(non_camel_case_types)]
pub struct vtkSphericalTransform(*mut core::ffi::c_void);
impl vtkSphericalTransform {
    /// Creates a new [vtkSphericalTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkSphericalTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSphericalTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSphericalTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSphericalTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSphericalTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSphericalTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSphericalTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSphericalTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSphericalTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSphericalTransform_create_drop() {
    let obj = vtkSphericalTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSphericalTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a nonlinear warp transformation
///
///
/// vtkThinPlateSplineTransform describes a nonlinear warp transform defined
/// by a set of source and target landmarks. Any point on the mesh close to a
/// source landmark will be moved to a place close to the corresponding target
/// landmark. The points in between are interpolated smoothly using
/// Bookstein's Thin Plate Spline algorithm.
///
/// To obtain a correct TPS warp, use the R2LogR kernel if your data is 2D, and
/// the R kernel if your data is 3D. Or you can specify your own RBF. (Hence this
/// class is more general than a pure TPS transform.)
/// @warning
/// 1) The inverse transform is calculated using an iterative method,
/// and is several times more expensive than the forward transform.
/// 2) Whenever you add, subtract, or set points you must call Modified()
/// on the vtkPoints object, or the transformation might not update.
/// 3) Collinear point configurations (except those that lie in the XY plane)
/// result in an unstable transformation. Forward transform can be computed
/// for any configuration by disabling bulk transform regularization.
/// @sa
/// vtkGridTransform vtkGeneralTransform
#[allow(non_camel_case_types)]
pub struct vtkThinPlateSplineTransform(*mut core::ffi::c_void);
impl vtkThinPlateSplineTransform {
    /// Creates a new [vtkThinPlateSplineTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkThinPlateSplineTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkThinPlateSplineTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkThinPlateSplineTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkThinPlateSplineTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkThinPlateSplineTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkThinPlateSplineTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkThinPlateSplineTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkThinPlateSplineTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkThinPlateSplineTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkThinPlateSplineTransform_create_drop() {
    let obj = vtkThinPlateSplineTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkThinPlateSplineTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// describes linear transformations via a 4x4 matrix
///
///
/// A vtkTransform can be used to describe the full range of linear (also
/// known as affine) coordinate transformations in three dimensions,
/// which are internally represented as a 4x4 homogeneous transformation
/// matrix.  When you create a new vtkTransform, it is always initialized
/// to the identity transformation.
/// <P>The SetInput() method allows you to set another transform,
/// instead of the identity transform, to be the base transformation.
/// There is a pipeline mechanism to ensure that when the input is
/// modified, the current transformation will be updated accordingly.
/// This pipeline mechanism is also supported by the Concatenate() method.
/// <P>Most of the methods for manipulating this transformation,
/// e.g. Translate, Rotate, and Concatenate, can operate in either
/// PreMultiply (the default) or PostMultiply mode.  In PreMultiply
/// mode, the translation, concatenation, etc. will occur before any
/// transformations which are represented by the current matrix.  In
/// PostMultiply mode, the additional transformation will occur after
/// any transformations represented by the current matrix.
/// <P>This class performs all of its operations in a right handed
/// coordinate system with right handed rotations. Some other graphics
/// libraries use left handed coordinate systems and rotations.
/// @sa
/// vtkPerspectiveTransform vtkGeneralTransform vtkMatrix4x4
/// vtkTransformCollection vtkTransformFilter vtkTransformPolyDataFilter
/// vtkImageReslice
#[allow(non_camel_case_types)]
pub struct vtkTransform(*mut core::ffi::c_void);
impl vtkTransform {
    /// Creates a new [vtkTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTransform_create_drop() {
    let obj = vtkTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// describes linear transformations via a 3x3 matrix
///
///
/// A vtkTransform2D can be used to describe the full range of linear (also
/// known as affine) coordinate transformations in two dimensions,
/// which are internally represented as a 3x3 homogeneous transformation
/// matrix.  When you create a new vtkTransform2D, it is always initialized
/// to the identity transformation.
///
/// All multiplicitive operations (Translate, Rotate, Scale, etc) are
/// post-multiplied in this class (i.e. add them in the reverse of the order
/// that they should be applied).
///
/// This class performs all of its operations in a right handed
/// coordinate system with right handed rotations. Some other graphics
/// libraries use left handed coordinate systems and rotations.
#[allow(non_camel_case_types)]
pub struct vtkTransform2D(*mut core::ffi::c_void);
impl vtkTransform2D {
    /// Creates a new [vtkTransform2D] wrapped inside `vtkNew`
    #[doc(alias = "vtkTransform2D")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTransform2D_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTransform2D_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTransform2D_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTransform2D_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTransform2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTransform2D {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTransform2D_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTransform2D_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTransform2D_create_drop() {
    let obj = vtkTransform2D::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTransform2D(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain a list of transforms
///
///
///
/// vtkTransformCollection is an object that creates and manipulates lists of
/// objects of type vtkTransform.
///
/// @sa
/// vtkCollection vtkTransform
#[allow(non_camel_case_types)]
pub struct vtkTransformCollection(*mut core::ffi::c_void);
impl vtkTransformCollection {
    /// Creates a new [vtkTransformCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkTransformCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTransformCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTransformCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTransformCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTransformCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTransformCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTransformCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTransformCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTransformCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTransformCollection_create_drop() {
    let obj = vtkTransformCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTransformCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
