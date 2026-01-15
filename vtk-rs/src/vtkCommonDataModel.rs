/// container of vtkUniformGrid for an AMR data set
///
///
///
/// vtkAMRDataInternals stores a list of non-empty blocks of an AMR data set
///
/// @sa
/// vtkOverlappingAMR, vtkAMRBox
#[allow(non_camel_case_types)]
pub struct vtkAMRDataInternals(*mut core::ffi::c_void);
impl vtkAMRDataInternals {
    /// Creates a new [vtkAMRDataInternals] wrapped inside `vtkNew`
    #[doc(alias = "vtkAMRDataInternals")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAMRDataInternals_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAMRDataInternals_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAMRDataInternals_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAMRDataInternals_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAMRDataInternals {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAMRDataInternals {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAMRDataInternals_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAMRDataInternals_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAMRDataInternals_create_drop() {
    let obj = vtkAMRDataInternals::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAMRDataInternals(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Iterates through adjacent vertices in a graph.
///
///
///
/// vtkAdjacentVertexIterator iterates through all vertices adjacent to a
/// vertex, i.e. the vertices which may be reached by traversing an out edge
/// of the source vertex. Use graph->GetAdjacentVertices(v, it) to initialize
/// the iterator.
#[allow(non_camel_case_types)]
pub struct vtkAdjacentVertexIterator(*mut core::ffi::c_void);
impl vtkAdjacentVertexIterator {
    /// Creates a new [vtkAdjacentVertexIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkAdjacentVertexIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAdjacentVertexIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAdjacentVertexIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAdjacentVertexIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAdjacentVertexIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAdjacentVertexIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAdjacentVertexIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAdjacentVertexIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAdjacentVertexIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAdjacentVertexIterator_create_drop() {
    let obj = vtkAdjacentVertexIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAdjacentVertexIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// the animation scene manager.
///
///
/// vtkAnimationCue and vtkAnimationScene provide the framework to support
/// animations in VTK. vtkAnimationCue represents an entity that changes/
/// animates with time, while vtkAnimationScene represents scene or setup
/// for the animation, which consists of individual cues or other scenes.
///
/// A scene can be played in real time mode, or as a sequence of frames
/// 1/frame rate apart in time.
/// @sa
/// vtkAnimationCue
#[allow(non_camel_case_types)]
pub struct vtkAnimationScene(*mut core::ffi::c_void);
impl vtkAnimationScene {
    /// Creates a new [vtkAnimationScene] wrapped inside `vtkNew`
    #[doc(alias = "vtkAnimationScene")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAnimationScene_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAnimationScene_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAnimationScene_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAnimationScene_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAnimationScene {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAnimationScene {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAnimationScene_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAnimationScene_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAnimationScene_create_drop() {
    let obj = vtkAnimationScene::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAnimationScene(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Stores a collection of annotation artifacts.
///
///
///
/// vtkAnnotation is a collection of annotation properties along with
/// an associated selection indicating the portion of data the annotation
/// refers to.
///
/// @par Thanks:
/// Timothy M. Shead (tshead@sandia.gov) at Sandia National Laboratories
/// contributed code to this class.
#[allow(non_camel_case_types)]
pub struct vtkAnnotation(*mut core::ffi::c_void);
impl vtkAnnotation {
    /// Creates a new [vtkAnnotation] wrapped inside `vtkNew`
    #[doc(alias = "vtkAnnotation")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAnnotation_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAnnotation_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAnnotation_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAnnotation_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAnnotation {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAnnotation {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAnnotation_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAnnotation_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAnnotation_create_drop() {
    let obj = vtkAnnotation::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAnnotation(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Stores a ordered collection of annotation sets
///
///
///
/// vtkAnnotationLayers stores a vector of annotation layers. Each layer
/// may contain any number of vtkAnnotation objects. The ordering of the
/// layers introduces a prioritization of annotations. Annotations in
/// higher layers may obscure annotations in lower layers.
#[allow(non_camel_case_types)]
pub struct vtkAnnotationLayers(*mut core::ffi::c_void);
impl vtkAnnotationLayers {
    /// Creates a new [vtkAnnotationLayers] wrapped inside `vtkNew`
    #[doc(alias = "vtkAnnotationLayers")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAnnotationLayers_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAnnotationLayers_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAnnotationLayers_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAnnotationLayers_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAnnotationLayers {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAnnotationLayers {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAnnotationLayers_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAnnotationLayers_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAnnotationLayers_create_drop() {
    let obj = vtkAnnotationLayers::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAnnotationLayers(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a annulus
///
///
/// vtkAnnulus computes the implicit function and function gradient
/// for an annulus composed of two co-axial cylinders. vtkAnnulus is a concrete
/// implementation of vtkImplicitFunction. By default the Annulus is
/// centered at the origin and the axis of rotation is along the
/// y-axis. You can redefine the center and axis of rotation by setting
/// the Center and Axis data members. (Note that it is also possible to
/// use the superclass' vtkImplicitFunction transformation matrix if
/// necessary to reposition by using FunctionValue() and
/// FunctionGradient().)
///
/// @warning
/// The annulus is infinite in extent. To truncate the annulus in
/// modeling operations use the vtkImplicitBoolean in combination with
/// clipping planes.
#[allow(non_camel_case_types)]
pub struct vtkAnnulus(*mut core::ffi::c_void);
impl vtkAnnulus {
    /// Creates a new [vtkAnnulus] wrapped inside `vtkNew`
    #[doc(alias = "vtkAnnulus")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAnnulus_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAnnulus_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAnnulus_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAnnulus_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAnnulus {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAnnulus {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAnnulus_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAnnulus_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAnnulus_create_drop() {
    let obj = vtkAnnulus::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAnnulus(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Pipeline data object that contains multiple vtkArray objects.
///
///
///
/// Because vtkArray cannot be stored as attributes of data objects (yet), a "carrier"
/// object is needed to pass vtkArray through the pipeline.  vtkArrayData acts as a
/// container of zero-to-many vtkArray instances, which can be retrieved via a zero-based
/// index.  Note that a collection of arrays stored in vtkArrayData may-or-may-not have related
/// types, dimensions, or extents.
///
/// @sa
/// vtkArrayDataAlgorithm, vtkArray
///
/// @par Thanks:
/// Developed by Timothy M. Shead (tshead@sandia.gov) at Sandia National Laboratories.
#[allow(non_camel_case_types)]
pub struct vtkArrayData(*mut core::ffi::c_void);
impl vtkArrayData {
    /// Creates a new [vtkArrayData] wrapped inside `vtkNew`
    #[doc(alias = "vtkArrayData")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkArrayData_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkArrayData_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkArrayData_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkArrayData_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkArrayData {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkArrayData {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkArrayData_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkArrayData_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkArrayData_create_drop() {
    let obj = vtkArrayData::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkArrayData(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects that compute
///
/// attribute-based error during cell tessellation.
///
///
/// It is a concrete error metric, based on an attribute criterium:
/// the variation of the active attribute/component value from a linear ramp
///
/// @sa
/// vtkGenericCellTessellator vtkGenericSubdivisionErrorMetric
#[allow(non_camel_case_types)]
pub struct vtkAttributesErrorMetric(*mut core::ffi::c_void);
impl vtkAttributesErrorMetric {
    /// Creates a new [vtkAttributesErrorMetric] wrapped inside `vtkNew`
    #[doc(alias = "vtkAttributesErrorMetric")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAttributesErrorMetric_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAttributesErrorMetric_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAttributesErrorMetric_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAttributesErrorMetric_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAttributesErrorMetric {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAttributesErrorMetric {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAttributesErrorMetric_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAttributesErrorMetric_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAttributesErrorMetric_create_drop() {
    let obj = vtkAttributesErrorMetric::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAttributesErrorMetric(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// This class represents an axis-aligned Binary Spatial
///
/// Partitioning of a 3D space.
///
///
/// This class converts between the vtkKdTree
/// representation of a tree of vtkKdNodes (used by vtkDistributedDataFilter)
/// and a compact array representation that might be provided by a
/// graph partitioning library like Zoltan.  Such a representation
/// could be used in message passing.
///
/// @sa
/// vtkKdTree vtkKdNode vtkDistributedDataFilter
#[allow(non_camel_case_types)]
pub struct vtkBSPCuts(*mut core::ffi::c_void);
impl vtkBSPCuts {
    /// Creates a new [vtkBSPCuts] wrapped inside `vtkNew`
    #[doc(alias = "vtkBSPCuts")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBSPCuts_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBSPCuts_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBSPCuts_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBSPCuts_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBSPCuts {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBSPCuts {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBSPCuts_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBSPCuts_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBSPCuts_create_drop() {
    let obj = vtkBSPCuts::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBSPCuts(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Perform calculations (mostly intersection
///
/// calculations) on regions of a 3D binary spatial partitioning.
///
///
/// Given an axis aligned binary spatial partitioning described by a
/// vtkBSPCuts object, perform intersection queries on various
/// geometric entities with regions of the spatial partitioning.
///
/// @sa
/// vtkBSPCuts  vtkKdTree
#[allow(non_camel_case_types)]
pub struct vtkBSPIntersections(*mut core::ffi::c_void);
impl vtkBSPIntersections {
    /// Creates a new [vtkBSPIntersections] wrapped inside `vtkNew`
    #[doc(alias = "vtkBSPIntersections")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBSPIntersections_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBSPIntersections_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBSPIntersections_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBSPIntersections_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBSPIntersections {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBSPIntersections {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBSPIntersections_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBSPIntersections_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBSPIntersections_create_drop() {
    let obj = vtkBSPIntersections::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBSPIntersections(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkBezierCurve(*mut core::ffi::c_void);
impl vtkBezierCurve {
    /// Creates a new [vtkBezierCurve] wrapped inside `vtkNew`
    #[doc(alias = "vtkBezierCurve")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBezierCurve_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBezierCurve_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBezierCurve_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBezierCurve_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBezierCurve {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBezierCurve {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBezierCurve_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBezierCurve_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBezierCurve_create_drop() {
    let obj = vtkBezierCurve::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBezierCurve(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 3D cell that represents an arbitrary order Bezier hex
///
///
/// vtkBezierHexahedron is a concrete implementation of vtkCell to represent a
/// 3D hexahedron using Bezier shape functions of user specified order.
///
/// @sa
/// vtkHexahedron
#[allow(non_camel_case_types)]
pub struct vtkBezierHexahedron(*mut core::ffi::c_void);
impl vtkBezierHexahedron {
    /// Creates a new [vtkBezierHexahedron] wrapped inside `vtkNew`
    #[doc(alias = "vtkBezierHexahedron")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBezierHexahedron_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBezierHexahedron_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBezierHexahedron_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBezierHexahedron_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBezierHexahedron {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBezierHexahedron {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBezierHexahedron_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBezierHexahedron_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBezierHexahedron_create_drop() {
    let obj = vtkBezierHexahedron::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBezierHexahedron(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkBezierInterpolation(*mut core::ffi::c_void);
impl vtkBezierInterpolation {
    /// Creates a new [vtkBezierInterpolation] wrapped inside `vtkNew`
    #[doc(alias = "vtkBezierInterpolation")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBezierInterpolation_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBezierInterpolation_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBezierInterpolation_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBezierInterpolation_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBezierInterpolation {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBezierInterpolation {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBezierInterpolation_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBezierInterpolation_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBezierInterpolation_create_drop() {
    let obj = vtkBezierInterpolation::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBezierInterpolation(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkBezierQuadrilateral(*mut core::ffi::c_void);
impl vtkBezierQuadrilateral {
    /// Creates a new [vtkBezierQuadrilateral] wrapped inside `vtkNew`
    #[doc(alias = "vtkBezierQuadrilateral")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBezierQuadrilateral_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBezierQuadrilateral_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBezierQuadrilateral_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBezierQuadrilateral_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBezierQuadrilateral {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBezierQuadrilateral {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBezierQuadrilateral_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBezierQuadrilateral_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBezierQuadrilateral_create_drop() {
    let obj = vtkBezierQuadrilateral::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBezierQuadrilateral(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 3D cell that represents an arbitrary order Bezier tetrahedron
///
///
/// vtkBezierTetra is a concrete implementation of vtkCell to represent a
/// 3D tetrahedron using Bezier shape functions of user specified order.
///
/// The number of points in a Bezier cell determines the order over which they
/// are iterated relative to the parametric coordinate system of the cell. The
/// first points that are reported are vertices. They appear in the same order in
/// which they would appear in linear cells. Mid-edge points are reported next.
/// They are reported in sequence. For two- and three-dimensional (3D) cells, the
/// following set of points to be reported are face points. Finally, 3D cells
/// report points interior to their volume.
#[allow(non_camel_case_types)]
pub struct vtkBezierTetra(*mut core::ffi::c_void);
impl vtkBezierTetra {
    /// Creates a new [vtkBezierTetra] wrapped inside `vtkNew`
    #[doc(alias = "vtkBezierTetra")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBezierTetra_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBezierTetra_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBezierTetra_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBezierTetra_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBezierTetra {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBezierTetra {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBezierTetra_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBezierTetra_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBezierTetra_create_drop() {
    let obj = vtkBezierTetra::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBezierTetra(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 2D cell that represents an arbitrary order Bezier triangle
///
///
/// vtkBezierTriangle is a concrete implementation of vtkCell to represent a
/// 2D triangle using Bezier shape functions of user specified order.
///
/// The number of points in a Bezier cell determines the order over which they
/// are iterated relative to the parametric coordinate system of the cell. The
/// first points that are reported are vertices. They appear in the same order in
/// which they would appear in linear cells. Mid-edge points are reported next.
/// They are reported in sequence. For two- and three-dimensional (3D) cells, the
/// following set of points to be reported are face points. Finally, 3D cells
/// report points interior to their volume.
#[allow(non_camel_case_types)]
pub struct vtkBezierTriangle(*mut core::ffi::c_void);
impl vtkBezierTriangle {
    /// Creates a new [vtkBezierTriangle] wrapped inside `vtkNew`
    #[doc(alias = "vtkBezierTriangle")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBezierTriangle_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBezierTriangle_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBezierTriangle_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBezierTriangle_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBezierTriangle {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBezierTriangle {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBezierTriangle_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBezierTriangle_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBezierTriangle_create_drop() {
    let obj = vtkBezierTriangle::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBezierTriangle(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 3D cell that represents an arbitrary order Bezier wedge
///
///
/// vtkBezierWedge is a concrete implementation of vtkCell to represent a
/// 3D wedge using Bezier shape functions of user specified order.
/// A wedge consists of two triangular and three quadrilateral faces.
/// The first six points of the wedge (0-5) are the "corner" points
/// where the first three points are the base of the wedge. This wedge
/// point ordering is opposite the vtkWedge ordering though in that
/// the base of the wedge defined by the first three points (0,1,2) form
/// a triangle whose normal points inward (toward the triangular face (3,4,5)).
/// While this is opposite the vtkWedge convention it is consistent with
/// every other cell type in VTK. The first 2 parametric coordinates of the
/// Bezier wedge or for the triangular base and vary between 0 and 1. The
/// third parametric coordinate is between the two triangular faces and goes
/// from 0 to 1 as well.
#[allow(non_camel_case_types)]
pub struct vtkBezierWedge(*mut core::ffi::c_void);
impl vtkBezierWedge {
    /// Creates a new [vtkBezierWedge] wrapped inside `vtkNew`
    #[doc(alias = "vtkBezierWedge")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBezierWedge_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBezierWedge_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBezierWedge_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBezierWedge_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBezierWedge {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBezierWedge {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBezierWedge_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBezierWedge_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBezierWedge_create_drop() {
    let obj = vtkBezierWedge::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBezierWedge(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 9-node
///
/// isoparametric quad
///
/// vtkQuadraticQuad is a concrete implementation of vtkNonLinearCell to
/// represent a two-dimensional, 9-node isoparametric parabolic quadrilateral
/// element with a Centerpoint. The interpolation is the standard finite
/// element, quadratic isoparametric shape function. The cell includes a
/// mid-edge node for each of the four edges of the cell and a center node at
/// the surface. The ordering of the eight points defining the cell are point
/// ids (0-3,4-8) where ids 0-3 define the four corner vertices of the quad;
/// ids 4-7 define the midedge nodes (0,1), (1,2), (2,3), (3,0) and 8 define
/// the face center node.
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticHexahedron vtkQuadraticWedge vtkQuadraticPyramid
/// vtkQuadraticQuad
///
/// @par Thanks:
/// Thanks to Soeren Gebbert who developed this class and
/// integrated it into VTK 5.0.
#[allow(non_camel_case_types)]
pub struct vtkBiQuadraticQuad(*mut core::ffi::c_void);
impl vtkBiQuadraticQuad {
    /// Creates a new [vtkBiQuadraticQuad] wrapped inside `vtkNew`
    #[doc(alias = "vtkBiQuadraticQuad")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBiQuadraticQuad_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBiQuadraticQuad_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBiQuadraticQuad_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBiQuadraticQuad_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBiQuadraticQuad {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBiQuadraticQuad {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBiQuadraticQuad_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBiQuadraticQuad_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBiQuadraticQuad_create_drop() {
    let obj = vtkBiQuadraticQuad::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBiQuadraticQuad(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a biquadratic,
///
/// 24-node isoparametric hexahedron
///
/// vtkBiQuadraticQuadraticHexahedron is a concrete implementation of vtkNonLinearCell to
/// represent a three-dimensional, 24-node isoparametric biquadratic
/// hexahedron. The interpolation is the standard finite element,
/// biquadratic-quadratic
/// isoparametric shape function. The cell includes mid-edge and center-face nodes. The
/// ordering of the 24 points defining the cell is point ids (0-7,8-19, 20-23)
/// where point ids 0-7 are the eight corner vertices of the cube; followed by
/// twelve midedge nodes (8-19), nodes 20-23 are the center-face nodes. Note that
/// these midedge nodes correspond lie
/// on the edges defined by (0,1), (1,2), (2,3), (3,0), (4,5), (5,6), (6,7),
/// (7,4), (0,4), (1,5), (2,6), (3,7). The center face nodes laying in quad
/// 22-(0,1,5,4), 21-(1,2,6,5), 23-(2,3,7,6) and 22-(3,0,4,7)
///
/// \verbatim
///
/// top
/// 7--14--6
/// |      |
/// 15      13
/// |      |
/// 4--12--5
///
/// middle
/// 19--23--18
/// |      |
/// 20      21
/// |      |
/// 16--22--17
///
/// bottom
/// 3--10--2
/// |      |
/// 11      9
/// |      |
/// 0-- 8--1
///
/// \endverbatim
///
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticQuad vtkQuadraticPyramid vtkQuadraticWedge
///
/// @par Thanks:
/// Thanks to Soeren Gebbert  who developed this class and
/// integrated it into VTK 5.0.
#[allow(non_camel_case_types)]
pub struct vtkBiQuadraticQuadraticHexahedron(*mut core::ffi::c_void);
impl vtkBiQuadraticQuadraticHexahedron {
    /// Creates a new [vtkBiQuadraticQuadraticHexahedron] wrapped inside `vtkNew`
    #[doc(alias = "vtkBiQuadraticQuadraticHexahedron")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBiQuadraticQuadraticHexahedron_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBiQuadraticQuadraticHexahedron_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBiQuadraticQuadraticHexahedron_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBiQuadraticQuadraticHexahedron_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBiQuadraticQuadraticHexahedron {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBiQuadraticQuadraticHexahedron {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBiQuadraticQuadraticHexahedron_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkBiQuadraticQuadraticHexahedron_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBiQuadraticQuadraticHexahedron_create_drop() {
    let obj = vtkBiQuadraticQuadraticHexahedron::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBiQuadraticQuadraticHexahedron(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 18-node isoparametric wedge
///
///
/// vtkBiQuadraticQuadraticWedge is a concrete implementation of vtkNonLinearCell to
/// represent a three-dimensional, 18-node isoparametric biquadratic
/// wedge. The interpolation is the standard finite element,
/// biquadratic-quadratic isoparametric shape function plus the linear functions.
/// The cell includes a mid-edge node. The
/// ordering of the 18 points defining the cell is point ids (0-5,6-15, 16-18)
/// where point ids 0-5 are the six corner vertices of the wedge; followed by
/// nine midedge nodes (6-15) and 3 center-face nodes. Note that these midedge
/// nodes correspond lie
/// on the edges defined by (0,1), (1,2), (2,0), (3,4), (4,5), (5,3), (0,3),
/// (1,4), (2,5), and the center-face nodes are laying in quads 16-(0,1,4,3),
/// 17-(1,2,5,4) and (2,0,3,5).
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticHexahedron vtkQuadraticQuad vtkQuadraticPyramid
///
/// @par Thanks:
/// Thanks to Soeren Gebbert who developed this class and
/// integrated it into VTK 5.0.
#[allow(non_camel_case_types)]
pub struct vtkBiQuadraticQuadraticWedge(*mut core::ffi::c_void);
impl vtkBiQuadraticQuadraticWedge {
    /// Creates a new [vtkBiQuadraticQuadraticWedge] wrapped inside `vtkNew`
    #[doc(alias = "vtkBiQuadraticQuadraticWedge")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBiQuadraticQuadraticWedge_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBiQuadraticQuadraticWedge_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBiQuadraticQuadraticWedge_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBiQuadraticQuadraticWedge_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBiQuadraticQuadraticWedge {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBiQuadraticQuadraticWedge {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBiQuadraticQuadraticWedge_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBiQuadraticQuadraticWedge_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBiQuadraticQuadraticWedge_create_drop() {
    let obj = vtkBiQuadraticQuadraticWedge::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBiQuadraticQuadraticWedge(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, isoparametric triangle
///
///
/// vtkBiQuadraticTriangle is a concrete implementation of vtkNonLinearCell to
/// represent a two-dimensional, 7-node, isoparametric parabolic triangle. The
/// interpolation is the standard finite element, bi-quadratic isoparametric
/// shape function. The cell includes three mid-edge nodes besides the three
/// triangle vertices and a center node. The ordering of the three points defining the cell is
/// point ids (0-2,3-6) where id #3 is the midedge node between points
/// (0,1); id #4 is the midedge node between points (1,2); and id #5 is the
/// midedge node between points (2,0). id #6 is the center node of the cell.
///
/// @sa
/// vtkTriangle vtkQuadraticTriangle
/// vtkBiQuadraticQuad vtkBiQuadraticQuadraticWedge vtkBiQuadraticQuadraticHexahedron
/// @par Thanks:
/// @verbatim
/// This file has been developed by Oxalya - www.oxalya.com
/// @endverbatim
#[allow(non_camel_case_types)]
pub struct vtkBiQuadraticTriangle(*mut core::ffi::c_void);
impl vtkBiQuadraticTriangle {
    /// Creates a new [vtkBiQuadraticTriangle] wrapped inside `vtkNew`
    #[doc(alias = "vtkBiQuadraticTriangle")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBiQuadraticTriangle_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBiQuadraticTriangle_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBiQuadraticTriangle_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBiQuadraticTriangle_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBiQuadraticTriangle {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBiQuadraticTriangle {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBiQuadraticTriangle_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBiQuadraticTriangle_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBiQuadraticTriangle_create_drop() {
    let obj = vtkBiQuadraticTriangle::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBiQuadraticTriangle(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a bounding box
///
///
/// vtkBox computes the implicit function and/or gradient for a axis-aligned
/// bounding box. (The superclasses transform can be used to modify this
/// orientation.) Each side of the box is orthogonal to all other sides
/// meeting along shared edges and all faces are orthogonal to the x-y-z
/// coordinate axes.  (If you wish to orient this box differently, recall that
/// the superclass vtkImplicitFunction supports a transformation matrix.)
/// vtkBox is a concrete implementation of vtkImplicitFunction.
///
/// @sa
/// vtkCubeSource vtkImplicitFunction vtkBoundingBox
#[allow(non_camel_case_types)]
pub struct vtkBox(*mut core::ffi::c_void);
impl vtkBox {
    /// Creates a new [vtkBox] wrapped inside `vtkNew`
    #[doc(alias = "vtkBox")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBox_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBox_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBox_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBox_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBox {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBox {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBox_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBox_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBox_create_drop() {
    let obj = vtkBox::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBox(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// object to represent cell connectivity
///
///
/// vtkCellArray stores dataset topologies as an explicit connectivity table
/// listing the point ids that make up each cell.
///
/// Internally, the connectivity table is represented as two arrays: Offsets and
/// Connectivity.
///
/// Offsets is an array of [numCells+1] values indicating the index in the
/// Connectivity array where each cell's points start. The last value is always
/// the length of the Connectivity array.
///
/// The Connectivity array stores the lists of point ids for each cell.
///
/// Thus, for a dataset consisting of 2 triangles, a quad, and a line, the
/// internal arrays will appear as follows:
///
/// ```cpp,ignore
/// Topology:
/// ---------
/// Cell 0: Triangle | point ids: {0, 1, 2}
/// Cell 1: Triangle | point ids: {5, 7, 2}
/// Cell 2: Quad     | point ids: {3, 4, 6, 7}
/// Cell 3: Line     | point ids: {5, 8}
///
/// vtkCellArray (current):
/// -----------------------
/// Offsets:      {0, 3, 6, 10, 12}
/// Connectivity: {0, 1, 2, 5, 7, 2, 3, 4, 6, 7, 5, 8}
/// ```
///
/// While this class provides traversal methods (the legacy InitTraversal(),
/// GetNextCell() methods, and the newer method GetCellAtId()) these are in
/// general not thread-safe. Whenever possible it is preferable to use a
/// local thread-safe, vtkCellArrayIterator object, which can be obtained via:
///
/// ```cpp,ignore
/// auto iter = vtk::TakeSmartPointer(cellArray->NewIterator());
/// for (iter->GoToFirstCell(); !iter->IsDoneWithTraversal(); iter->GoToNextCell())
/// {
/// // do work with iter
/// }
/// ```
/// (Note however that depending on the type and structure of internal
/// storage, a cell array iterator may be significantly slower than direct
/// traversal over the cell array due to extra data copying. Factors of 3-4X
/// are not uncommon. See vtkCellArrayIterator for more information. Also note
/// that an iterator may become invalid if the internal vtkCellArray storage
/// is modified.)
///
/// Other methods are also available for allocation and memory-related
/// management; insertion of new cells into the vtkCellArray; and limited
/// editing operations such as replacing one cell with a new cell of the
/// same size.
///
/// The internal arrays may store either 32- or 64-bit values, though most of
/// the API will prefer to use vtkIdType to refer to items in these
/// arrays. This enables significant memory savings when vtkIdType is 64-bit,
/// but 32 bits are sufficient to store all of the values in the connectivity
/// table. Using 64-bit storage with a 32-bit vtkIdType is permitted, but
/// values too large to fit in a 32-bit signed integer will be truncated when
/// accessed through the API. (The particular internal storage type has
/// implications on performance depending on vtkIdType. If the internal
/// storage is equivalent to vtkIdType, then methods that return pointers to
/// arrays of point ids can share the internal storage; otherwise a copy of
/// internal memory must be performed.)
///
/// Methods for managing the storage type are:
///
/// - `bool IsStorage64Bit()`
/// - `bool IsStorageShareable() // Can pointers to internal storage be shared`
/// - `void Use32BitStorage()`
/// - `void Use64BitStorage()`
/// - `void UseDefaultStorage() // Depends on vtkIdType`
/// - `bool CanConvertTo32BitStorage()`
/// - `bool CanConvertTo64BitStorage()`
/// - `bool CanConvertToDefaultStorage() // Depends on vtkIdType`
/// - `bool ConvertTo32BitStorage()`
/// - `bool ConvertTo64BitStorage()`
/// - `bool ConvertToDefaultStorage() // Depends on vtkIdType`
/// - `bool ConvertToSmallestStorage() // Depends on current values in arrays`
///
/// Note that some legacy methods are still available that reflect the
/// previous storage format of this data, which embedded the cell sizes into
/// the Connectivity array:
///
/// ```cpp,ignore
/// vtkCellArray (legacy):
/// ----------------------
/// Connectivity: {3, 0, 1, 2, 3, 5, 7, 2, 4, 3, 4, 6, 7, 2, 5, 8}
/// |--Cell 0--||--Cell 1--||----Cell 2---||--C3-|
/// ```
///
/// The methods require an external lookup table to allow random access, which
/// was historically stored in the vtkCellTypes object. The following methods in
/// vtkCellArray still support this style of indexing for compatibility
/// purposes, but these are slow as they must perform some complex computations
/// to convert the old "location" into the new "offset" and should be avoided.
/// These methods (and their modern equivalents) are:
///
/// - GetCell (Prefer GetCellAtId)
/// - GetInsertLocation (Prefer GetNumberOfCells)
/// - GetTraversalLocation (Prefer GetTraversalCellId, or better, NewIterator)
/// - SetTraversalLocation (Prefer SetTraversalLocation, or better, NewIterator)
/// - ReverseCell (Prefer ReverseCellAtId)
/// - ReplaceCell (Prefer ReplaceCellAtId)
/// - SetCells (Use ImportLegacyFormat, or SetData)
/// - GetData (Use ExportLegacyFormat, or Get[Offsets|Connectivity]Array[|32|64])
///
/// Some other legacy methods were completely removed, such as GetPointer() /
/// WritePointer(), since they are cannot be effectively emulated under the
/// current design. If external code needs to support both the old and new
/// version of the vtkCellArray API, the VTK_CELL_ARRAY_V2 preprocessor
/// definition may be used to detect which API is being compiled against.
///
/// @sa vtkAbstractCellArray vtkStructuredCellArray vtkCellTypes vtkCellLinks
#[allow(non_camel_case_types)]
pub struct vtkCellArray(*mut core::ffi::c_void);
impl vtkCellArray {
    /// Creates a new [vtkCellArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellArray_create_drop() {
    let obj = vtkCellArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Encapsulate traversal logic for vtkCellArray.
///
///
/// This is iterator for thread-safe traversal of a vtkCellArray. It provides
/// random access and forward iteration. Typical usage for forward iteration
/// looks like:
///
/// ```cpp,ignore
/// auto iter = vtk::TakeSmartPointer(cellArray->NewIterator());
/// for (iter->GoToFirstCell(); !iter->IsDoneWithTraversal(); iter->GoToNextCell())
/// {
/// // do work with iter
/// iter->GetCurrentCell(numCellPts, cellPts);
/// }
/// ```
///
/// Typical usage for random access looks like:
///
/// ```cpp,ignore
/// auto iter = vtk::TakeSmartPointer(cellArray->NewIterator());
/// iter->GetCellAtId(cellId, numCellPts, cellPts);
/// ```
///
/// Here @a cellId is the id of the ith cell in the vtkCellArray;
/// @a numCellPts is the number of points defining the cell represented
/// as vtkIdType; and @a cellPts is a pointer to the point ids defined
/// as vtkIdType const*&.
///
/// Internally the iterator may copy data from the vtkCellArray, or reference
/// the internal vtkCellArray storage. This depends on the relationship of
/// vtkIdType to the type and structure of internal storage. If the type of
/// storage is the same as vtkIdType, and the storage is a single-component
/// AOS array (i.e., a 1D array), then shared access to the vtkCellArray
/// storage is provided. Otherwise, the data from storage is copied into an
/// internal iterator buffer. (Of course copying is slower and can result in
/// 3-4x reduction in traversal performance. On the other hand, the
/// vtkCellArray can use the appropriate storage to save memory, perform
/// zero-copy, and/or efficiently represent the cell connectivity
/// information.) Note that referencing internal vtkCellArray storage has
/// implications on the validity of the iterator. If the underlying
/// vtkCellArray storage changes while iterating, and the iterator is
/// referencing this storage, unpredictable and catastrophic results are
/// likely - hence do not modify the vtkCellArray while iterating.
///
/// @sa
/// vtkCellArray
#[allow(non_camel_case_types)]
pub struct vtkCellArrayIterator(*mut core::ffi::c_void);
impl vtkCellArrayIterator {
    /// Creates a new [vtkCellArrayIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellArrayIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellArrayIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellArrayIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellArrayIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellArrayIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellArrayIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellArrayIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellArrayIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellArrayIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellArrayIterator_create_drop() {
    let obj = vtkCellArrayIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellArrayIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A function defined over the physical domain of a vtkCellGrid.
///
///
/// This is a base class for attributes (functions) defined on the space
/// discretized by a vtkCellGrid. A vtkCellAttribute class must handle
/// cells of all types present in the grid.
///
/// @sa vtkCellGrid
#[allow(non_camel_case_types)]
pub struct vtkCellAttribute(*mut core::ffi::c_void);
impl vtkCellAttribute {
    /// Creates a new [vtkCellAttribute] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellAttribute")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellAttribute_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellAttribute_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellAttribute_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellAttribute_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellAttribute {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellAttribute {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellAttribute_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellAttribute_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellAttribute_create_drop() {
    let obj = vtkCellAttribute::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellAttribute(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Perform a per-cell calculation on a vtkCellAttribute.
///
///
/// This empty class serves as a common base class for calculators that
/// compute quantities based on cell-attribute data.
///
/// Examples of calculators include
/// + computing interpolated values;
/// + computing spatial derivatives (such as the Jacobian or Hessian matrices); or
/// + computing integrals over an entire cell.
///
/// Each type of calculator provides its own abstract subclass with virtual methods
/// and then per-cell-type, per-attribute-type concrete implementations.
///
/// @sa vtkCellGridAttribute
#[allow(non_camel_case_types)]
pub struct vtkCellAttributeCalculator(*mut core::ffi::c_void);
impl vtkCellAttributeCalculator {
    /// Creates a new [vtkCellAttributeCalculator] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellAttributeCalculator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellAttributeCalculator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellAttributeCalculator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellAttributeCalculator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellAttributeCalculator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellAttributeCalculator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellAttributeCalculator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellAttributeCalculator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellAttributeCalculator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellAttributeCalculator_create_drop() {
    let obj = vtkCellAttributeCalculator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellAttributeCalculator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// represent and manipulate cell attribute data
///
///
/// vtkCellData is a class that is used to represent and manipulate
/// cell attribute data (e.g., scalars, vectors, normals, texture
/// coordinates, etc.) Special methods are provided to work with filter
/// objects, such as passing data through filter, copying data from one
/// cell to another, and interpolating data given cell interpolation weights.
///
/// By default, `GhostTypesToSkip` is set to `DUPLICATECELL | HIDDENCELL | REFINEDCELL`.
/// See `vtkDataSetAttributes` for the definition of those constants.
#[allow(non_camel_case_types)]
pub struct vtkCellData(*mut core::ffi::c_void);
impl vtkCellData {
    /// Creates a new [vtkCellData] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellData")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellData_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellData_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellData_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellData_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellData {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellData {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellData_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellData_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellData_create_drop() {
    let obj = vtkCellData::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellData(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Visualization data composed of cells of arbitrary type.
///
///
/// vtkCellGrid inherits vtkDataObject in order to introduce the concept
/// of cells that, instead of relying on spatial points to specify their
/// shape, rely on degrees of freedom (which may or may not be embedded
/// in a world coordinate system).
///
/// The degrees of freedom that define cells and the functions using those
/// cells as their domain are provided in data arrays.
/// The arrays are partitioned into groups (vtkDataSetAttributes) by the
/// registered cell types. Each array in a group has the same number of tuples.
///
/// @sa vtkDataObject vtkDataSetAttributes
#[allow(non_camel_case_types)]
pub struct vtkCellGrid(*mut core::ffi::c_void);
impl vtkCellGrid {
    /// Creates a new [vtkCellGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGrid_create_drop() {
    let obj = vtkCellGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Compute the geometric bounds of a cell-grid.
///
///
/// If no cells are present, invalid bounds will be returned
/// (i.e., bds[1] < bds[0] after calling `GetBounds(bds)`).
#[allow(non_camel_case_types)]
pub struct vtkCellGridBoundsQuery(*mut core::ffi::c_void);
impl vtkCellGridBoundsQuery {
    /// Creates a new [vtkCellGridBoundsQuery] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGridBoundsQuery")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGridBoundsQuery_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGridBoundsQuery_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGridBoundsQuery_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGridBoundsQuery_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGridBoundsQuery {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGridBoundsQuery {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGridBoundsQuery_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGridBoundsQuery_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGridBoundsQuery_create_drop() {
    let obj = vtkCellGridBoundsQuery::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGridBoundsQuery(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Copy the cell metadata and attribute(s) of one cell-grid into another.
///
///
/// Note that this query is run by vtkCellGrid::ShallowCopy(), vtkCellGrid::DeepCOpy(),
/// and vtkCellGrid::CopyStructure().
///
/// In general, there are five types of information in cell grids that may
/// be transferred from the source cell-grid to the target. Here are
/// the types of information and the options which control how that
/// information is copied. Exactly how these flags on the query are used is
/// up to each responder.
///
/// + **Cell metadata records.** These records are always copied.
/// In the future, there may be an option to omit cells of specific types.
///
/// + **Individual cells.** If subclasses of vtkCellMetadata contain
/// further information, you may use SetCopyCells() to control
/// whether that is copied or whether the new vtkCellMetadata
/// instance is left uninitialized.
/// When GetCopyCells() is enabled, the cell topology should be copied
/// (though not necessarily the shape attribute's arrays);
/// CopyCells overrides the copying of topological arrays even if
/// CopyArrays is turned off.
/// This way, if CopyCells is on, you should expect the source and
/// target to report the same number of cells.
///
/// + **Cell attributes.** You may request that only the shape attribute
/// is copied from the source to the target with CopyOnlyShapeOn()
/// or control which attributes are copied by calling
/// AddSourceCellAttributeId() with the ID of each source attribute
/// you wish copied.
///
/// + **Cell-attribute arrays.** For each cell-attribute that is copied,
/// zero or more arrays may be associated the attribute. You can
/// control how the arrays are copied like so:
///
/// + SetCopyArrays() controls whether arrays should be created or not.
/// How the arrays are copied depends on whether CopyArrayValues and
/// DeepCopyArrays are enabled. Note that this setting should be
/// ignored when copying cell topology (as opposed to attribute) arrays
/// as CopyCells should control whether cells are present in the output.
/// If cell-topology arrays are referenced by a cell attribute, be aware
/// that disabling CopyArrays may still produce some entries for topology
/// arrays.
/// + SetCopyArrayValues() controls whether arrays should be (a) created
/// but left empty or (b) created and populated with the source-array's
/// values. This is useful for creating an empty copy that has all
/// the necessary arrays prepared but no tuples so that further
/// processing can insert new cells and attribute data.
/// + SetDeepCopyArrays() controls whether to create deep copies of
/// arrays or shallow copies, but only when GetCopyArrayValues()
/// returns true.
///
/// + **Schema and content version.** A cell-grid may advertise that its
/// data adheres to a formal specification (which is indicated by a
/// name and version number). If you wish to copy this information,
/// ensure CopySchemaOn() has been called.
/// If GetCopySchema() is true and the source has a content version
/// number, the target cell-grid will have its content version
/// incremented past the source's content version.
/// Incrementing the content version even when the grids are otherwise
/// identical improves track-ability, since the version number informs
/// which grid preceded the other.
///
/// ## For Callers
///
/// You **must** execute this query on the source cell-grid, not the target.
/// Only the source is guaranteed to have cells of the proper types present;
/// the query iterates over each cell-type, so they must be present.
///
/// Executing this query will overwrite the target cell-grid with the source,
/// erasing all of its cell metadata.
/// In the future, this class may offer more control over which types of
/// cells to copy from the source to the target.
///
/// ## For Responders
///
/// Responders to this query may call the helper methods provided
/// to copy a cell-attribute's arrays and create/update a cell-attribute.
/// These calls update maps from source to target arrays and attributes,
/// which you can inspect by calling GetArrayMap() and GetAttributeMap(),
/// respectively.
/// The latter is important since distinct attributes may have identical
/// names (though this is not advised).
#[allow(non_camel_case_types)]
pub struct vtkCellGridCopyQuery(*mut core::ffi::c_void);
impl vtkCellGridCopyQuery {
    /// Creates a new [vtkCellGridCopyQuery] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGridCopyQuery")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGridCopyQuery_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGridCopyQuery_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGridCopyQuery_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGridCopyQuery_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGridCopyQuery {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGridCopyQuery {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGridCopyQuery_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGridCopyQuery_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGridCopyQuery_create_drop() {
    let obj = vtkCellGridCopyQuery::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGridCopyQuery(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Evaluate a field (vtkCellAttribute) at some points inside cells.
///
///
/// This class is a cell-grid query whose purpose is to determine the
/// value a vtkCellAttribute takes on at one or more points inside
/// the domain of a vtkCellGrid.
///
/// This class performs its work in two phases:
/// + Classification. Input points are classified by the type and index
/// of cell in the grid in which they lie.
/// + Attribute/field interpolation. Each cell is asked to interpolate
/// the value of a cell-attribute at each point classified to it.
///
/// As an example, consider a cell-grid holding 10 triangles and 20 quads
/// and a query that is provided 5 points. The first phase will identify
/// which of the 5 points are insides triangles, which lie in quadrilaterals,
/// and which lie in neither. Say that 2 lie inside triangles, 2 inside
/// quadrilaterals, and 1 is outside the domain.
/// Furthermore, the first phase will identify which particular triangles
/// or quadrilaterals contain the input points. The two points which
/// lie in triangles will report a number in [0,10[ while the two points
/// which lie in quadrilaterals will report a number in [0, 20[.
/// Finally, for cells which have a reference element, the parametric
/// coordinates of each input point are computed.
///
/// The second phase additionally interpolates a cell-attribute (let's
/// say "Velocity" in our example) at each input point.
///
/// You may configure the query to skip either phase (classification or
/// interpolation). If you skip classification, you must provide the
/// the classification information for the input points.
/// The method you call (ClassifyPoints, InterpolatePoints, or
/// InterpolateCellParameters) determines which phase(s) are applied
/// during evaluation.
///
/// When running in InterpolatePoints mode (both classification and
/// interpolation phases are performed), the output from our example
/// is reported like so:
///
/// + `GetClassifierCellTypes()` – returns an array with a cell-type hash
/// for each type of cell containing an input point. The hash value can
/// be used to construct a vtkStringToken.
/// Our example would return an array with 3 values which might be
/// ordered: "vtkDGTri"_hash, "vtkDGQuad"_hash, and 0 (an "invalid" hash).
/// + `GetClassifierCellOffsets()` – returns an array with the same number
/// of values as the call above. Each value specifies the start of
/// reporting for points contained in the corresponding cell type.
/// Our example would return [0, 2, 4] to match the values above.
/// + `GetClassifierPointIDs()` – returns an array whose length matches
/// the number of input points. Each value is the index of an input
/// point. Input points do not have their order preserved so that
/// all the points contained in a single cell can be reported together.
/// Our example might return [4, 2, 1, 0, 3]. This will always be a
/// permutation of the counting integers and, for our example, always
/// hold integers in [0, 5[.
/// + `GetClassifierCellIndices()` – returns an array whose length matches
/// the number of input points. Each value is the index into cells
/// of the corresponding type, indicating which cell contains
/// the input point.
/// For our example, the first two numbers will be in [0, 10[, the second
/// two will be in [0, 20[, and the last will be -1. (This is because
/// we have two points inside 10 triangles, two points inside 20 quads,
/// and one un-classifiable input point.)
/// + `GetClassifierPointParameters()` – returns an array whose length
/// matches the number of input points. Each value is a 3-tuple of
/// reference-cell coordinates (or indeterminate if the cell type does
/// not provide a reference cell).
/// + `GetInterpolatedValues()` – returns an array whose number of tuples
/// matches the number of input points and whose number of components
/// matches the number of components of the requested cell-attribute.
/// For our example, an array with 5 tuples of 3 components each would be
/// returned; it would be named "Velocity" (matching the cell-attribute's
/// name).
///
/// Note that because you can pass in the arrays above (except the interpolated
/// values) to short-circuit classification, it is possible to evaluate
/// multiple cell-attributes without duplicating the classification work.
///
/// In InterpolateCellParameters mode, calling the methods above which begin
/// with `GetClassifier…` will simply return the input arrays you passed to
/// configure the query.
///
/// ## Warnings
///
/// The output arrays above generally match the number of input points, but
/// will sometimes exceed the number of input points. This will occur when
/// multiple cells contain an input point (either on a shared boundary or
/// because the cells overlap).
///
/// Note that the output should never have fewer points than the input as
/// even points outside the cells will be classified as such.
///
/// Currently, this class is limited to evaluating numeric attributes;
/// string or variant arrays are not supported.
#[allow(non_camel_case_types)]
pub struct vtkCellGridEvaluator(*mut core::ffi::c_void);
impl vtkCellGridEvaluator {
    /// Creates a new [vtkCellGridEvaluator] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGridEvaluator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGridEvaluator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGridEvaluator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGridEvaluator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGridEvaluator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGridEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGridEvaluator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGridEvaluator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGridEvaluator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGridEvaluator_create_drop() {
    let obj = vtkCellGridEvaluator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGridEvaluator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Compute the range of a component of some vtkCellAttribute.
///
///
/// If \a FiniteRange is true, then the range will omit any NaN or ±Inf
/// values present in the data. Otherwise (the default), the range may
/// contain these exceptional values.
///
/// If \a Component is
/// + -2 (the default), the range of L₂-norms is computed.
/// + -1, the range of L₁-norms is computed.
/// + out of bounds, then an invalid range will be returned ([1, 0]).
///
/// Note that this query is intended to be run by vtkCellGrid::GetRange()
/// since the cell-grid holds a cache of ranges. You may run it outside
/// of this method, but that may cause unnecessary re-computation of ranges.
#[allow(non_camel_case_types)]
pub struct vtkCellGridRangeQuery(*mut core::ffi::c_void);
impl vtkCellGridRangeQuery {
    /// Creates a new [vtkCellGridRangeQuery] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGridRangeQuery")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGridRangeQuery_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGridRangeQuery_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGridRangeQuery_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGridRangeQuery_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGridRangeQuery {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGridRangeQuery {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGridRangeQuery_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGridRangeQuery_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGridRangeQuery_create_drop() {
    let obj = vtkCellGridRangeQuery::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGridRangeQuery(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A container that holds objects able to respond to queries
///
/// specialized for particular vtkCellMetadata types.
///
/// This class holds sets of responders for vtkCellGridQuery and for
/// vtkCellAttributeQuery.
///
/// ## Cell-grid query responders
///
/// vtkCellGridResponders holds a list of objects statically registered to
/// the vtkCellMetadata subclass. These objects respond to
/// queries for information (e.g., a bounding box) or processing
/// (e.g., rendering, picking, generating isocontours) for one cell
/// type (and subclasses of that cell type if no more specific
/// responder is registered).
///
/// Application code (such as a plugin) can register subclasses of
/// vtkCellGridResponse which accept the API of a particular
/// vtkCellGridQuery for that cell type.
/// Then, when a query is passed to the cell, this collection will
/// identify matching responders for the query and invoke them until
/// one returns true (indicating success).
/// Multiple matches can exist as a responder can be registered to a
/// common base cell class and/or to handle common base query classes.
///
/// If a given cell type cannot respond to a query, its superclasses
/// are asked to respond. If no superclass can respond to the query,
/// then query's superclasses are searched for responders.
///
/// Because queries can be registered to cell types at any time,
/// existing cell types can be extended to support new features
/// by additional libraries.
///
/// ## Cell-attribute calculators
///
/// In order to support the evaluation of vtkCellAttribute data
/// on any vtkCellMetadata (cell type), this class also holds
/// "calculators" grouped by both attribute and cell type.
/// This API is different that vtkCellGridQuery because
/// vtkCellAttribute is not subclassed by attribute type but
/// rather uses vtkStringToken data to determine the nature of the
/// attribute (e.g., Lagrange; Nedelec; Raviart-Thomas; etc.).
///
/// It is also different in that these objects are expected to be
/// used inside vtkCellGridQuery to evaluate a single cell at a
/// time rather than longer-running queries across all cells.
///
/// @sa vtkCellMetadata vtkCellGrid vtkCellAttribute
#[allow(non_camel_case_types)]
pub struct vtkCellGridResponders(*mut core::ffi::c_void);
impl vtkCellGridResponders {
    /// Creates a new [vtkCellGridResponders] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGridResponders")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGridResponders_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGridResponders_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGridResponders_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGridResponders_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGridResponders {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGridResponders {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGridResponders_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGridResponders_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGridResponders_create_drop() {
    let obj = vtkCellGridResponders::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGridResponders(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Hold a map from hash-ids (representing sides of cells of multiple types)
///
/// to details on the cells that claim the corresponding side.
///
/// This class is created by filters such as vtkCellGridComputeSides and
/// vtkCellGridExtractCrinkle; it can be re-used by the same filter and
/// any others that process the same input (since it is stored in a
/// cache available to them).
#[allow(non_camel_case_types)]
pub struct vtkCellGridSidesCache(*mut core::ffi::c_void);
impl vtkCellGridSidesCache {
    /// Creates a new [vtkCellGridSidesCache] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGridSidesCache")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGridSidesCache_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGridSidesCache_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGridSidesCache_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGridSidesCache_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGridSidesCache {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGridSidesCache {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGridSidesCache_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGridSidesCache_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGridSidesCache_create_drop() {
    let obj = vtkCellGridSidesCache::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGridSidesCache(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
/// \brief A cell-grid query for enumerating sides of cells.
///
/// This query runs in 3 passes (see vtkCellGridSidesQuery::PassWork):
///
/// + In the first pass, responders invoke the AddSides() method on
/// this query, entries are added to this->Hashes storage indicating
/// the cells which are bounded by a given shape + connectivity.
/// + In the second pass, responders mark the entries created above and
/// add entries in this->Sides. This reorganizes the hashes into groups
/// more amenable to output as side arrays. This pass is called
/// "Summarization," since not every input side identified will be
/// output.
/// + In the third and final pass, responders create new cells in
/// the output cell-grid that correspond to the selected sides of
/// the input.
#[allow(non_camel_case_types)]
pub struct vtkCellGridSidesQuery(*mut core::ffi::c_void);
impl vtkCellGridSidesQuery {
    /// Creates a new [vtkCellGridSidesQuery] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGridSidesQuery")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGridSidesQuery_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGridSidesQuery_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGridSidesQuery_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGridSidesQuery_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGridSidesQuery {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGridSidesQuery {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGridSidesQuery_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGridSidesQuery_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGridSidesQuery_create_drop() {
    let obj = vtkCellGridSidesQuery::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGridSidesQuery(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// object represents upward pointers from points to list of cells using each point
///
///
/// vtkCellLinks is a supplemental object to vtkCellArray and vtkCellTypes,
/// enabling access from points to the cells using the points. vtkCellLinks is
/// a list of cell ids, each such link representing a dynamic list of cell ids
/// using the point. The information provided by this object can be used to
/// determine neighbors and construct other local topological information.
///
/// @warning
/// vtkCellLinks supports incremental (i.e., "editable") operations such as
/// inserting a new cell, or deleting a point. Because of this, it is less
/// memory efficient, and slower to construct and delete than static classes
/// such as vtkStaticCellLinks or vtkStaticCellLinksTemplate. However these
/// other classes are typically meant for one-time (static) construction.
///
/// @sa
/// vtkCellArray vtkCellTypes vtkStaticCellLinks vtkStaticCellLinksTemplate
#[allow(non_camel_case_types)]
pub struct vtkCellLinks(*mut core::ffi::c_void);
impl vtkCellLinks {
    /// Creates a new [vtkCellLinks] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellLinks")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellLinks_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellLinks_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellLinks_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellLinks_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellLinks {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellLinks {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellLinks_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellLinks_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellLinks_create_drop() {
    let obj = vtkCellLinks::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellLinks(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// octree-based spatial search object to quickly locate cells
///
///
/// vtkCellLocator is a spatial search object to quickly locate cells in 3D.
/// vtkCellLocator uses a uniform-level octree subdivision, where each octant
/// (an octant is also referred to as a bucket) carries an indication of
/// whether it is empty or not, and each leaf octant carries a list of the
/// cells inside of it. (An octant is not empty if it has one or more cells
/// inside of it.)  Typical operations are intersection with a line to return
/// candidate cells, or intersection with another vtkCellLocator to return
/// candidate cells.
///
/// @warning
/// vtkCellLocator utilizes the following parent class parameters:
/// - Automatic                   (default true)
/// - Level                       (default 8)
/// - MaxLevel                    (default 8)
/// - NumberOfCellsPerNode        (default 25)
/// - CacheCellBounds             (default true)
/// - UseExistingSearchStructure  (default false)
///
/// vtkCellLocator does NOT utilize the following parameters:
/// - Tolerance
/// - RetainCellLists
///
/// @sa
/// vtkAbstractCellLocator vtkStaticCellLocator vtkCellTreeLocator vtkModifiedBSPTree vtkOBBTree
#[allow(non_camel_case_types)]
pub struct vtkCellLocator(*mut core::ffi::c_void);
impl vtkCellLocator {
    /// Creates a new [vtkCellLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellLocator_create_drop() {
    let obj = vtkCellLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implement a specific vtkPointSet::FindCell() strategy based
///
/// on using a cell locator
///
/// vtkCellLocatorStrategy is implements a FindCell() strategy based on
/// using the FindCell() method in a cell locator. This is often the
/// slowest strategy, but the most robust.
///
/// @sa
/// vtkFindCellStrategy vtkPointSet
#[allow(non_camel_case_types)]
pub struct vtkCellLocatorStrategy(*mut core::ffi::c_void);
impl vtkCellLocatorStrategy {
    /// Creates a new [vtkCellLocatorStrategy] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellLocatorStrategy")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellLocatorStrategy_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellLocatorStrategy_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellLocatorStrategy_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellLocatorStrategy_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellLocatorStrategy {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellLocatorStrategy {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellLocatorStrategy_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellLocatorStrategy_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellLocatorStrategy_create_drop() {
    let obj = vtkCellLocatorStrategy::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellLocatorStrategy(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// This class implements the data structures, construction
///
/// algorithms for fast cell location.
///
/// Cell Tree is a bounding interval hierarchy based data structure, where child boxes
/// do not form an exact split of the parent boxes along a dimension.  Therefore two axis-
/// aligned bounding planes (left max and right min) are stored for each node along a
/// dimension. This class implements the data structure (Cell Tree Node) and its build
/// and traversal algorithms described in the paper.
/// Some methods in building and traversing the cell tree in this class were derived
/// from avtCellLocatorBIH class in the VisIT Visualization Tool.
///
/// vtkCellTreeLocator utilizes the following parent class parameters:
/// - NumberOfCellsPerNode        (default 8)
/// - CacheCellBounds             (default true)
/// - UseExistingSearchStructure  (default false)
///
/// vtkCellTreeLocator does NOT utilize the following parameters:
/// - Automatic
/// - Level
/// - MaxLevel
/// - Tolerance
/// - RetainCellLists
///
/// @warning
/// This class is templated. It may run slower than serial execution if the code
/// is not optimized during compilation. Build in Release or ReleaseWithDebugInfo.
///
/// From the article: "Fast, Memory-Efficient Cell location in Unstructured Grids for Visualization"
/// by Christoph Garth and Kenneth I. Joy in VisWeek, 2011.
///
/// @sa
/// vtkAbstractCellLocator vtkCellLocator vtkStaticCellLocator vtkModifiedBSPTree vtkOBBTree
#[allow(non_camel_case_types)]
pub struct vtkCellTreeLocator(*mut core::ffi::c_void);
impl vtkCellTreeLocator {
    /// Creates a new [vtkCellTreeLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellTreeLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellTreeLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellTreeLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellTreeLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellTreeLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellTreeLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellTreeLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellTreeLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellTreeLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellTreeLocator_create_drop() {
    let obj = vtkCellTreeLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellTreeLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// object provides direct access to cells in vtkCellArray and type information
///
///
/// This class is a supplemental object to vtkCellArray to allow random access
/// into cells as well as representing cell type information.  The "location"
/// field is the location in the vtkCellArray list in terms of an integer
/// offset.  An integer offset was used instead of a pointer for easy storage
/// and inter-process communication. The type information is defined in the
/// file vtkCellType.h.
///
/// @warning
/// Sometimes this class is used to pass type information independent of the
/// random access (i.e., location) information. For example, see
/// vtkDataSet::GetCellTypes(). If you use the class in this way, you can use
/// a location value of -1.
///
/// @sa
/// vtkCellArray vtkCellLinks
#[allow(non_camel_case_types)]
pub struct vtkCellTypes(*mut core::ffi::c_void);
impl vtkCellTypes {
    /// Creates a new [vtkCellTypes] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellTypes")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellTypes_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellTypes_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellTypes_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellTypes_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellTypes {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellTypes {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellTypes_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellTypes_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellTypes_create_drop() {
    let obj = vtkCellTypes::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellTypes(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implement a specific vtkPointSet::FindCell() strategy based
///
/// on the N closest points
///
/// vtkClosestNPointsStrategy is implements a FindCell() strategy based on
/// locating the closest N points in a dataset, and then searching attached
/// cells. This class extends its superclass vtkClosestPointStrategy by looking
/// at the additional N points.
///
/// @sa
/// vtkFindCellStrategy vtkPointSet
#[allow(non_camel_case_types)]
pub struct vtkClosestNPointsStrategy(*mut core::ffi::c_void);
impl vtkClosestNPointsStrategy {
    /// Creates a new [vtkClosestNPointsStrategy] wrapped inside `vtkNew`
    #[doc(alias = "vtkClosestNPointsStrategy")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkClosestNPointsStrategy_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkClosestNPointsStrategy_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkClosestNPointsStrategy_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkClosestNPointsStrategy_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkClosestNPointsStrategy {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkClosestNPointsStrategy {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkClosestNPointsStrategy_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkClosestNPointsStrategy_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkClosestNPointsStrategy_create_drop() {
    let obj = vtkClosestNPointsStrategy::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkClosestNPointsStrategy(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implement a specific vtkPointSet::FindCell() strategy based
///
/// on closest point
///
/// vtkClosestPointStrategy is implements a FindCell() strategy based on
/// locating the closest point in a dataset, and then searching the attached
/// cells. While relatively fast, it does not always return the correct result
/// (it may not find a cell, since the closest cell may not be connected to the
/// closest point). vtkCellLocatorStrategy or vtkClosestNPointsStrategy will
/// produce better results at the cost of speed.
///
/// @sa
/// vtkFindCellStrategy vtkPointSet vtkCellLocatorStrategy
/// vtkClosestNPointsStrategy
#[allow(non_camel_case_types)]
pub struct vtkClosestPointStrategy(*mut core::ffi::c_void);
impl vtkClosestPointStrategy {
    /// Creates a new [vtkClosestPointStrategy] wrapped inside `vtkNew`
    #[doc(alias = "vtkClosestPointStrategy")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkClosestPointStrategy_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkClosestPointStrategy_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkClosestPointStrategy_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkClosestPointStrategy_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkClosestPointStrategy {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkClosestPointStrategy {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkClosestPointStrategy_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkClosestPointStrategy_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkClosestPointStrategy_create_drop() {
    let obj = vtkClosestPointStrategy::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkClosestPointStrategy(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a cone
///
///
/// vtkCone computes the implicit function and function gradient for a cone.
/// vtkCone is a concrete implementation of vtkImplicitFunction. By default, the cone vertex
/// is located at the origin with axis of rotation coincident with x-axis. You can use
/// the superclass' vtkImplicitFunction transformation matrix to reposition. You can alternatively
/// use the accessors provided by this class, which will cause the transform to be recomputed. to
/// reposition/orient the cone. The angle specifies the angle between the axis of rotation and the
/// side of the cone.
///
/// @warning
/// The cone is infinite in extent (on both sides if IsDoubleCone is set to true). To truncate the
/// cone use the vtkImplicitBoolean in combination with clipping planes.
#[allow(non_camel_case_types)]
pub struct vtkCone(*mut core::ffi::c_void);
impl vtkCone {
    /// Creates a new [vtkCone] wrapped inside `vtkNew`
    #[doc(alias = "vtkCone")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCone_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCone_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCone_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCone_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCone {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCone {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCone_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCone_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCone_create_drop() {
    let obj = vtkCone::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCone(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a 3D cell defined by a set of convex points
///
///
/// vtkConvexPointSet is a concrete implementation that represents a 3D cell
/// defined by a convex set of points. An example of such a cell is an octant
/// (from an octree). vtkConvexPointSet uses the ordered triangulations
/// approach (vtkOrderedTriangulator) to create triangulations guaranteed to
/// be compatible across shared faces. This allows a general approach to
/// processing complex, convex cell types.
///
/// @sa
/// vtkHexahedron vtkPyramid vtkTetra vtkVoxel vtkWedge
#[allow(non_camel_case_types)]
pub struct vtkConvexPointSet(*mut core::ffi::c_void);
impl vtkConvexPointSet {
    /// Creates a new [vtkConvexPointSet] wrapped inside `vtkNew`
    #[doc(alias = "vtkConvexPointSet")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkConvexPointSet_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkConvexPointSet_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkConvexPointSet_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkConvexPointSet_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkConvexPointSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkConvexPointSet {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkConvexPointSet_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkConvexPointSet_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkConvexPointSet_create_drop() {
    let obj = vtkConvexPointSet::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkConvexPointSet(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a right-handed coordinate system
///
///
/// vtkCoordinateFrame computes an implicit function and function gradient
/// for a set of 3 orthogonal planes.
///
/// The function evaluates to a combination of quartic spherical harmonic
/// basis functions:
/// \f$\sqrt(\frac{7}{12})*Y_{4,0} + \sqrt(\frac{5}{12})*Y_{4,4}\f$
/// that – when evaluated on a unit sphere centered at the coordinate frame's
/// origin – form a 6-lobed function with a maximum along each of the
/// 6 axes (3 positive, 3 negative).
/// This function is frequently used in frame-field design.
///
/// See the paper "On Smooth Frame Field Design" by Nicolas Ray and
/// Dmitry Sokolov (2016, hal-01245657,
/// https://hal.inria.fr/hal-01245657/file/framefield.pdf ) for more
/// information.
#[allow(non_camel_case_types)]
pub struct vtkCoordinateFrame(*mut core::ffi::c_void);
impl vtkCoordinateFrame {
    /// Creates a new [vtkCoordinateFrame] wrapped inside `vtkNew`
    #[doc(alias = "vtkCoordinateFrame")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCoordinateFrame_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCoordinateFrame_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCoordinateFrame_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCoordinateFrame_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCoordinateFrame {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCoordinateFrame {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCoordinateFrame_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCoordinateFrame_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCoordinateFrame_create_drop() {
    let obj = vtkCoordinateFrame::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCoordinateFrame(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a cubic , isoparametric 1D line
///
///
/// vtkCubicLine is a concrete implementation of vtkNonLinearCell to represent a 1D Cubic line.
/// The Cubic Line is the 4 nodes isoparametric parabolic line . The
/// interpolation is the standard finite element, cubic isoparametric
/// shape function. The cell includes two mid-edge nodes. The ordering of the
/// four points defining the cell is point ids (0,1,2,3) where id #2 and #3 are the
/// mid-edge nodes. Please note that the parametric coordinates lie between -1 and 1
/// in accordance with most standard documentations.
/// @par Thanks:
/// \verbatim
/// This file has been developed by Oxalya - www.oxalya.com
/// \endverbatim
#[allow(non_camel_case_types)]
pub struct vtkCubicLine(*mut core::ffi::c_void);
impl vtkCubicLine {
    /// Creates a new [vtkCubicLine] wrapped inside `vtkNew`
    #[doc(alias = "vtkCubicLine")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCubicLine_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCubicLine_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCubicLine_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCubicLine_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCubicLine {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCubicLine {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCubicLine_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCubicLine_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCubicLine_create_drop() {
    let obj = vtkCubicLine::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCubicLine(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a cylinder
///
///
/// vtkCylinder computes the implicit function and function gradient
/// for a cylinder using F(r)=r^2-Radius^2. vtkCylinder is a concrete
/// implementation of vtkImplicitFunction. By default the Cylinder is
/// centered at the origin and the axis of rotation is along the
/// y-axis. You can redefine the center and axis of rotation by setting
/// the Center and Axis data members. (Note that it is also possible to
/// use the superclass' vtkImplicitFunction transformation matrix if
/// necessary to reposition by using FunctionValue() and
/// FunctionGradient().)
///
/// @warning
/// The cylinder is infinite in extent. To truncate the cylinder in
/// modeling operations use the vtkImplicitBoolean in combination with
/// clipping planes.
///
/// @sa
/// vtkCylinderSource
#[allow(non_camel_case_types)]
pub struct vtkCylinder(*mut core::ffi::c_void);
impl vtkCylinder {
    /// Creates a new [vtkCylinder] wrapped inside `vtkNew`
    #[doc(alias = "vtkCylinder")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCylinder_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCylinder_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCylinder_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCylinder_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCylinder {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCylinder {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCylinder_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCylinder_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCylinder_create_drop() {
    let obj = vtkCylinder::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCylinder(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// hierarchical representation to use with
///
/// vtkPartitionedDataSetCollection
///
/// vtkDataAssembly is a mechanism to represent hierarchical organization of
/// items (or vtkPartitionedDataSet instances) in a vtkPartitionedDataSetCollection.
/// vtkPartitionedDataSetCollection is similar to a vtkMultiBlockDataSet since it
/// provides a means for putting together multiple non-composite datasets.
/// However, vtkPartitionedDataSetCollection itself doesn't provide any mechanism
/// to define relationships between items in the collections. That is done using
/// vtkDataAssembly.
///
/// @section Overview Overview
///
/// At its core, vtkDataAssembly is simply a tree of nodes starting
/// with the root node. Each node has a unique id and a string name (names need not
/// be unique). On initialization with `vtkDataAssembly::Initialize`, an empty tree
/// with a root node is created. The root node's id and name can be obtained
/// using `vtkDataAssembly::GetRootNode` and `vtkDataAssembly::GetRootNodeName`.
/// The root node's id is fixed (vtkDataAssembly::GetRootNode), however the name
/// can be changed using `vtkDataAssembly::SetRootNodeName`.
///
/// Child nodes can be added using `vtkDataAssembly::AddNode` or
/// `vtkDataAssembly::AddNodes`, each of which returns the ids for every child
/// node added. A non-root node can be removed using `vtkDataAssembly::RemoveNode`.
///
/// Each node in the tree (including the root node) can have associated dataset
/// indices. For a vtkDataAssembly associated with a
/// vtkPartitionedDataSetCollection, these indices refer to the item index, or
/// partitioned-dataset-index for items in the collection. Dataset indices can be
/// specified using `vtkDataAssembly::AddDataSetIndex`,
/// `vtkDataAssembly::AddDataSetIndices` and removed using `vtkDataAssembly::RemoveDataSetIndex`,
/// `vtkDataAssembly::RemoveAllDataSetIndices`.
/// `vtkDataAssembly::GetDataSetIndices` provides a mechanism to get the
/// database indices associated with a node, and optionally, the entire subtree
/// rooted at the chosen node.
///
/// @section Searching Searching
///
/// Each node in the vtkDataAssembly is assigned a unique id.
/// `vtkDataAssembly::FindFirstNodeWithName` and
/// `vtkDataAssembly::FindNodesWithName` can be used to get the id(s) for
/// node(s) with given name.
///
/// `vtkDataAssembly::SelectNodes` provides a
/// more flexible mechanism to find nodes using name-based queries. Section
/// @ref DataAssemblyPathQueries covers supported queries.
///
/// @section Traversal Traversal
///
/// `vtkDataAssemblyVisitor` defines a visitor API. An instance of a concretized
/// `vtkDataAssemblyVisitor` subclass can be passed to `vtkDataAssembly::Visit`
/// to traverse the data-assembly hierarchy either in depth-first or
/// breadth-first order.
///
/// @section DataAssemblyPathQueries Supported Path Queries
///
/// `vtkDataAssembly::SelectNodes` can be used find nodes that match the
/// specified query (or queries) using XPath 1.0 syntax.
///
/// For example:
///
/// * '/' is used as the path separator. If a node name has a `/` it must be
/// escaped using `\\` in the query. Note, escaping is not necessary when using
/// `SetNodeName`/`GetNodeName`.
///
/// * '/' selects the root node.
///
/// * '/nodename' selects all child nodes of the root with the name 'nodename'.
///
/// * '//nodename' selects all nodes with 'nodename' that are descendants of the
/// root; thus, this this will traverse the entire tree.
///
/// * '/nodename/' selects all child nodes of all nodes named 'nodename' under
/// the root; thus, ending a query with '/' selects the children of the found
/// nodes rather than the nodes themselves.
///
/// * '/nodename1/nodename2' selects all nodes named 'nodename2' which are
/// children of nodes with name 'nodename1' that are themselves children of
/// the root node.
///
/// * '//nodename1/nodename2' finds all nodes in the tree named 'nodename1' and
/// then selects all children of these found nodes that are named 'nodename2'.
///
/// @section Applications Applications
///
/// The separation of dataset storage (vtkPartitionedDataSetCollection) and
/// organization (vtkDataAssembly) enables development of algorithms that can
/// expose APIs that are not tightly coupled to dataset storage. Together,
/// vtkPartitionedDataSetCollection and vtkDataAssembly can be thought of as a
/// different way of organizing data that was previously organized as a
/// vtkMultiBlockDataSet. The advantage of the this newer approach is that
/// filters can support specifying parameters using paths or path queries
/// rather than composite indices. The composite indices suffered from the fact
/// that they made no sense except for the specific vtkMultiBlockDataSet they
/// were applied too. Thus, if the filters input was changed, the composite ids
/// rarely made any sense and needed to be updated. Paths and path queries,
/// however, do not suffer from this issue.
#[allow(non_camel_case_types)]
pub struct vtkDataAssembly(*mut core::ffi::c_void);
impl vtkDataAssembly {
    /// Creates a new [vtkDataAssembly] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataAssembly")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataAssembly_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataAssembly_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataAssembly_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataAssembly_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataAssembly {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataAssembly {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataAssembly_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataAssembly_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataAssembly_create_drop() {
    let obj = vtkDataAssembly::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataAssembly(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// collections of utilities for vtkDataAssembly
///
///
/// vtkDataAssemblyUtilities provides useful utilities for working with
/// vtkDataAssembly.
#[allow(non_camel_case_types)]
pub struct vtkDataAssemblyUtilities(*mut core::ffi::c_void);
impl vtkDataAssemblyUtilities {
    /// Creates a new [vtkDataAssemblyUtilities] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataAssemblyUtilities")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataAssemblyUtilities_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataAssemblyUtilities_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataAssemblyUtilities_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataAssemblyUtilities_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataAssemblyUtilities {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataAssemblyUtilities {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataAssemblyUtilities_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataAssemblyUtilities_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataAssemblyUtilities_create_drop() {
    let obj = vtkDataAssemblyUtilities::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataAssemblyUtilities(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// general representation of visualization data
///
///
/// vtkDataObject is an general representation of visualization data. It serves
/// to encapsulate instance variables and methods for visualization network
/// execution, as well as representing data consisting of a field (i.e., just
/// an unstructured pile of data). This is to be compared with a vtkDataSet,
/// which is data with geometric and/or topological structure.
///
/// vtkDataObjects are used to represent arbitrary repositories of data via the
/// vtkFieldData instance variable. These data must be eventually mapped into a
/// concrete subclass of vtkDataSet before they can actually be displayed.
///
/// @sa
/// vtkDataSet vtkFieldData vtkDataObjectToDataSetFilter
/// vtkFieldDataToAttributeDataFilter
#[allow(non_camel_case_types)]
pub struct vtkDataObject(*mut core::ffi::c_void);
impl vtkDataObject {
    /// Creates a new [vtkDataObject] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataObject")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataObject_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataObject_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataObject_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataObject_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataObject {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataObject {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataObject_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataObject_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataObject_create_drop() {
    let obj = vtkDataObject::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataObject(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain an unordered list of data objects
///
///
/// vtkDataObjectCollection is an object that creates and manipulates ordered
/// lists of data objects. See also vtkCollection and subclasses.
#[allow(non_camel_case_types)]
pub struct vtkDataObjectCollection(*mut core::ffi::c_void);
impl vtkDataObjectCollection {
    /// Creates a new [vtkDataObjectCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataObjectCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataObjectCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataObjectCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataObjectCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataObjectCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataObjectCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataObjectCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataObjectCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataObjectCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataObjectCollection_create_drop() {
    let obj = vtkDataObjectCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataObjectCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// superclass for composite data iterators
///
///
/// vtkDataObjectTreeIterator provides an interface for accessing datasets
/// in a collection (vtkDataObjectTreeIterator).
#[allow(non_camel_case_types)]
pub struct vtkDataObjectTreeIterator(*mut core::ffi::c_void);
impl vtkDataObjectTreeIterator {
    /// Creates a new [vtkDataObjectTreeIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataObjectTreeIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataObjectTreeIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataObjectTreeIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataObjectTreeIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataObjectTreeIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataObjectTreeIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataObjectTreeIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataObjectTreeIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataObjectTreeIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataObjectTreeIterator_create_drop() {
    let obj = vtkDataObjectTreeIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataObjectTreeIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkDataObjectTypes(*mut core::ffi::c_void);
impl vtkDataObjectTypes {
    /// Creates a new [vtkDataObjectTypes] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataObjectTypes")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataObjectTypes_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataObjectTypes_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataObjectTypes_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataObjectTypes_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataObjectTypes {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataObjectTypes {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataObjectTypes_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataObjectTypes_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataObjectTypes_create_drop() {
    let obj = vtkDataObjectTypes::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataObjectTypes(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// represent and manipulate attribute data in a dataset
///
///
/// vtkDataSetAttributes is a class that is used to represent and manipulate
/// attribute data (e.g., scalars, vectors, normals, texture coordinates,
/// tensors, global ids, pedigree ids, and field data).
///
/// This adds to vtkFieldData the ability to pick one of the arrays from the
/// field as the currently active array for each attribute type. In other
/// words, you pick one array to be called "THE" Scalars, and then filters down
/// the pipeline will treat that array specially. For example vtkContourFilter
/// will contour "THE" Scalar array unless a different array is asked for.
///
/// Additionally vtkDataSetAttributes provides methods that filters call to
/// pass data through, copy data into, and interpolate from Fields. PassData
/// passes entire arrays from the source to the destination. Copy passes
/// through some subset of the tuples from the source to the destination.
/// Interpolate interpolates from the chosen tuple(s) in the source data, using
/// the provided weights, to produce new tuples in the destination.
/// Each attribute type has pass, copy and interpolate "copy" flags that
/// can be set in the destination to choose which attribute arrays will be
/// transferred from the source to the destination.
///
/// Finally this class provides a mechanism to determine which attributes a
/// group of sources have in common, and to copy tuples from a source into
/// the destination, for only those attributes that are held by all.
///
/// @warning
/// vtkDataSetAttributes is not in general thread safe due to the use of its
/// vtkFieldData::BasicIterator RequiredArrays data member. The class
/// vtkArrayListTemplate augments vtkDataSetAttributes for thread safety.
///
/// @sa vtkArrayListTemplate
#[allow(non_camel_case_types)]
pub struct vtkDataSetAttributes(*mut core::ffi::c_void);
impl vtkDataSetAttributes {
    /// Creates a new [vtkDataSetAttributes] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataSetAttributes")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataSetAttributes_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataSetAttributes_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataSetAttributes_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataSetAttributes_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataSetAttributes {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataSetAttributes {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataSetAttributes_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataSetAttributes_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataSetAttributes_create_drop() {
    let obj = vtkDataSetAttributes::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataSetAttributes(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Implementation of vtkCellIterator using
///
/// vtkDataSet API.
#[allow(non_camel_case_types)]
pub struct vtkDataSetCellIterator(*mut core::ffi::c_void);
impl vtkDataSetCellIterator {
    /// Creates a new [vtkDataSetCellIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataSetCellIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataSetCellIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataSetCellIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataSetCellIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataSetCellIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataSetCellIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataSetCellIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataSetCellIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataSetCellIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataSetCellIterator_create_drop() {
    let obj = vtkDataSetCellIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataSetCellIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain an unordered list of dataset objects
///
///
/// vtkDataSetCollection is an object that creates and manipulates ordered
/// lists of datasets. See also vtkCollection and subclasses.
#[allow(non_camel_case_types)]
pub struct vtkDataSetCollection(*mut core::ffi::c_void);
impl vtkDataSetCollection {
    /// Creates a new [vtkDataSetCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataSetCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataSetCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataSetCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataSetCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataSetCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataSetCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataSetCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataSetCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataSetCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataSetCollection_create_drop() {
    let obj = vtkDataSetCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataSetCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A rooted tree data structure.
///
///
///
/// vtkDirectedAcyclicGraph is a connected directed graph with no cycles. A tree is a type of
/// directed graph, so works with all graph algorithms.
///
/// vtkDirectedAcyclicGraph is a read-only data structure.
/// To construct a tree, create an instance of vtkMutableDirectedGraph.
/// Add vertices and edges with AddVertex() and AddEdge(). You may alternately
/// start by adding a single vertex as the root then call graph->AddChild(parent)
/// which adds a new vertex and connects the parent to the child.
/// The tree MUST have all edges in the proper direction, from parent to child.
/// After building the tree, call tree->CheckedShallowCopy(graph) to copy the
/// structure into a vtkDirectedAcyclicGraph. This method will return false if the graph is
/// an invalid tree.
///
/// vtkDirectedAcyclicGraph provides some convenience methods for obtaining the parent and
/// children of a vertex, for finding the root, and determining if a vertex
/// is a leaf (a vertex with no children).
///
/// @sa
/// vtkDirectedGraph vtkMutableDirectedGraph vtkGraph
#[allow(non_camel_case_types)]
pub struct vtkDirectedAcyclicGraph(*mut core::ffi::c_void);
impl vtkDirectedAcyclicGraph {
    /// Creates a new [vtkDirectedAcyclicGraph] wrapped inside `vtkNew`
    #[doc(alias = "vtkDirectedAcyclicGraph")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDirectedAcyclicGraph_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDirectedAcyclicGraph_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDirectedAcyclicGraph_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDirectedAcyclicGraph_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDirectedAcyclicGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDirectedAcyclicGraph {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDirectedAcyclicGraph_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDirectedAcyclicGraph_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDirectedAcyclicGraph_create_drop() {
    let obj = vtkDirectedAcyclicGraph::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDirectedAcyclicGraph(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A directed graph.
///
///
///
/// vtkDirectedGraph is a collection of vertices along with a collection of
/// directed edges (edges that have a source and target). ShallowCopy()
/// and DeepCopy() (and CheckedShallowCopy(), CheckedDeepCopy())
/// accept instances of vtkTree and vtkMutableDirectedGraph.
///
/// vtkDirectedGraph is read-only. To create an undirected graph,
/// use an instance of vtkMutableDirectedGraph, then you may set the
/// structure to a vtkDirectedGraph using ShallowCopy().
///
/// @sa
/// vtkGraph vtkMutableDirectedGraph
#[allow(non_camel_case_types)]
pub struct vtkDirectedGraph(*mut core::ffi::c_void);
impl vtkDirectedGraph {
    /// Creates a new [vtkDirectedGraph] wrapped inside `vtkNew`
    #[doc(alias = "vtkDirectedGraph")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDirectedGraph_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDirectedGraph_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDirectedGraph_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDirectedGraph_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDirectedGraph {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDirectedGraph_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDirectedGraph_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDirectedGraph_create_drop() {
    let obj = vtkDirectedGraph::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDirectedGraph(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Iterates through all edges in a graph.
///
///
///
/// vtkEdgeListIterator iterates through all the edges in a graph, by traversing
/// the adjacency list for each vertex. You may instantiate this class directly
/// and call SetGraph() to traverse a certain graph. You may also call the graph's
/// GetEdges() method to set up the iterator for a certain graph.
///
/// Note that this class does NOT guarantee that the edges will be processed in
/// order of their ids (i.e. it will not necessarily return edge 0, then edge 1,
/// etc.).
///
/// @sa
/// vtkGraph
#[allow(non_camel_case_types)]
pub struct vtkEdgeListIterator(*mut core::ffi::c_void);
impl vtkEdgeListIterator {
    /// Creates a new [vtkEdgeListIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkEdgeListIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEdgeListIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkEdgeListIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkEdgeListIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkEdgeListIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkEdgeListIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEdgeListIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEdgeListIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEdgeListIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEdgeListIterator_create_drop() {
    let obj = vtkEdgeListIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkEdgeListIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// keep track of edges (edge is pair of integer id's)
///
///
/// vtkEdgeTable is a general object for keeping track of lists of edges. An
/// edge is defined by the pair of point id's (p1,p2). Methods are available
/// to insert edges, check if edges exist, and traverse the list of edges.
/// Also, it's possible to associate attribute information with each edge.
/// The attribute information may take the form of vtkIdType id's, void*
/// pointers, or points. To store attributes, make sure that
/// InitEdgeInsertion() is invoked with the storeAttributes flag set properly.
/// If points are inserted, use the methods InitPointInsertion() and
/// InsertUniquePoint().
#[allow(non_camel_case_types)]
pub struct vtkEdgeTable(*mut core::ffi::c_void);
impl vtkEdgeTable {
    /// Creates a new [vtkEdgeTable] wrapped inside `vtkNew`
    #[doc(alias = "vtkEdgeTable")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEdgeTable_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkEdgeTable_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkEdgeTable_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkEdgeTable_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkEdgeTable {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEdgeTable {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEdgeTable_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEdgeTable_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEdgeTable_create_drop() {
    let obj = vtkEdgeTable::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkEdgeTable(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// an empty cell used as a place-holder during processing
///
///
/// vtkEmptyCell is a concrete implementation of vtkCell. It is used
/// during processing to represented a deleted element.
#[allow(non_camel_case_types)]
pub struct vtkEmptyCell(*mut core::ffi::c_void);
impl vtkEmptyCell {
    /// Creates a new [vtkEmptyCell] wrapped inside `vtkNew`
    #[doc(alias = "vtkEmptyCell")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEmptyCell_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkEmptyCell_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkEmptyCell_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkEmptyCell_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkEmptyCell {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEmptyCell {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEmptyCell_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEmptyCell_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEmptyCell_create_drop() {
    let obj = vtkEmptyCell::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkEmptyCell(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// structured grid with explicit topology and geometry
///
///
/// vtkExplicitStructuredGrid is a data object that is a concrete implementation
/// of vtkDataSet. vtkExplicitStructuredGrid represents a geometric structure
/// that is a topologically regular array of hexahedron. The topology is that of
/// a cube that has been subdivided into a regular array of smaller cubes.
/// Each cell can be addressed with i-j-k indices, however neighbor hexahedrons
/// does not necessarily share a face and hexahedron can be blanked (turned-off).
///
/// Like unstructured grid, vtkExplicitStructuredGrid has explicit point coordinates
/// and cell to point indexing.
/// Unlike unstructured grid, vtkExplicitStructuredGrid does not keep a cell type
/// list as all visible cells are known to be hexahedra.
/// vtkExplicitStructuredGrid can take advantage of its layout to perform operations
/// based on the i, j, k parameters, similar to structured grid. This makes some
/// operations faster on this class, without losing the flexibility of the
/// cell -> points mapping.
/// The most common use of this class would be in situations where you have all
/// hexahedra but the points used by the cells are not exactly defined by the
/// i, j, k parameters. One example of this is a structured grid with a half voxel
/// shift occurring in the middle of it such as with a geologic fault.
///
/// The order and number of points is arbitrary.
/// The order and number of cells must match that specified by the dimensions
/// of the grid minus 1, because in vtk structured datasets the dimensions
/// correspond to the points.
/// The cells order increases in i fastest (from 0 <= i <= dims[0] - 2),
/// then j (0 <= j <= dims[1] - 2), then k ( 0 <= k <= dims[2] - 2) where dims[]
/// are the dimensions of the grid in the i-j-k topological directions.
/// The number of cells is (dims[0] - 1) * (dims[1] - 1) * (dims[2] - 1).
///
/// In order for an ESG to be usable by most other ESG specific filters,
/// it is needed to call the ComputeFacesConnectivityFlagsArray method.
/// It is also recommended to call CheckAndReorderFaces method to fix any
/// faces issues in the dataset.
#[allow(non_camel_case_types)]
pub struct vtkExplicitStructuredGrid(*mut core::ffi::c_void);
impl vtkExplicitStructuredGrid {
    /// Creates a new [vtkExplicitStructuredGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkExplicitStructuredGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExplicitStructuredGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkExplicitStructuredGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkExplicitStructuredGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkExplicitStructuredGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkExplicitStructuredGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExplicitStructuredGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExplicitStructuredGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExplicitStructuredGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExplicitStructuredGrid_create_drop() {
    let obj = vtkExplicitStructuredGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkExplicitStructuredGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// helper for extracting/sub-sampling
///
/// structured datasets.
///
///
/// vtkExtractStructuredGridHelper provides some common functionality that is
/// used by filters that extract and sub-sample structured data. Specifically,
/// it provides functionality for calculating the mapping from the output extent
/// of each process to the input extent.
///
/// @sa
/// vtkExtractGrid vtkExtractVOI vtkExtractRectilinearGrid
#[allow(non_camel_case_types)]
pub struct vtkExtractStructuredGridHelper(*mut core::ffi::c_void);
impl vtkExtractStructuredGridHelper {
    /// Creates a new [vtkExtractStructuredGridHelper] wrapped inside `vtkNew`
    #[doc(alias = "vtkExtractStructuredGridHelper")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExtractStructuredGridHelper_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkExtractStructuredGridHelper_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkExtractStructuredGridHelper_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkExtractStructuredGridHelper_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkExtractStructuredGridHelper {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExtractStructuredGridHelper {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExtractStructuredGridHelper_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExtractStructuredGridHelper_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExtractStructuredGridHelper_create_drop() {
    let obj = vtkExtractStructuredGridHelper::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkExtractStructuredGridHelper(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// represent and manipulate fields of data
///
///
/// vtkFieldData represents and manipulates fields of data. The model of a field
/// is a m x n matrix of data values, where m is the number of tuples, and n
/// is the number of components. (A tuple is a row of n components in the
/// matrix.) The field is assumed to be composed of a set of one or more data
/// arrays, where the data in the arrays are of different types (e.g., int,
/// double, char, etc.), and there may be variable numbers of components in
/// each array. Note that each data array is assumed to be "m" in length
/// (i.e., number of tuples), which typically corresponds to the number of
/// points or cells in a dataset. Also, each data array must have a
/// character-string name. (This is used to manipulate data.)
///
/// There are two ways of manipulating and interfacing to fields. You can do
/// it generically by manipulating components/tuples via a double-type data
/// exchange, or you can do it by grabbing the arrays and manipulating them
/// directly. The former is simpler but performs type conversion, which is bad
/// if your data has non-castable types like (void) pointers, or you lose
/// information as a result of the cast. The more efficient method means
/// managing each array in the field.  Using this method you can create
/// faster, more efficient algorithms that do not lose information.
///
/// @sa
/// vtkAbstractArray vtkDataSetAttributes vtkPointData vtkCellData
#[allow(non_camel_case_types)]
pub struct vtkFieldData(*mut core::ffi::c_void);
impl vtkFieldData {
    /// Creates a new [vtkFieldData] wrapped inside `vtkNew`
    #[doc(alias = "vtkFieldData")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFieldData_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkFieldData_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkFieldData_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkFieldData_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkFieldData {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFieldData {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFieldData_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFieldData_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFieldData_create_drop() {
    let obj = vtkFieldData::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkFieldData(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a collection of attributes
///
///
/// vtkGenericAttributeCollection is a class that collects attributes
/// (represented by vtkGenericAttribute).
#[allow(non_camel_case_types)]
pub struct vtkGenericAttributeCollection(*mut core::ffi::c_void);
impl vtkGenericAttributeCollection {
    /// Creates a new [vtkGenericAttributeCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkGenericAttributeCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGenericAttributeCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGenericAttributeCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGenericAttributeCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGenericAttributeCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGenericAttributeCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGenericAttributeCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGenericAttributeCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGenericAttributeCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGenericAttributeCollection_create_drop() {
    let obj = vtkGenericAttributeCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGenericAttributeCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// provides thread-safe access to cells
///
///
/// vtkGenericCell is a class that provides access to concrete types of cells.
/// It's main purpose is to allow thread-safe access to cells, supporting
/// the vtkDataSet::GetCell(vtkGenericCell *) method. vtkGenericCell acts
/// like any type of cell, it just dereferences an internal representation.
/// The SetCellType() methods use \#define constants; these are defined in
/// the file vtkCellType.h.
///
/// @sa
/// vtkCell vtkDataSet
#[allow(non_camel_case_types)]
pub struct vtkGenericCell(*mut core::ffi::c_void);
impl vtkGenericCell {
    /// Creates a new [vtkGenericCell] wrapped inside `vtkNew`
    #[doc(alias = "vtkGenericCell")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGenericCell_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGenericCell_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGenericCell_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGenericCell_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGenericCell {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGenericCell {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGenericCell_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGenericCell_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGenericCell_create_drop() {
    let obj = vtkGenericCell::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGenericCell(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// keep track of edges (defined by pair of integer id's)
///
///
/// vtkGenericEdgeTable is used to indicate the existence of and hold
/// information about edges. Similar to vtkEdgeTable, this class is
/// more sophisticated in that it uses reference counting to keep track
/// of when information about an edge should be deleted.
///
/// vtkGenericEdgeTable is a helper class used in the adaptor framework.  It
/// is used during the tessellation process to hold information about the
/// error metric on each edge. This avoids recomputing the error metric each
/// time the same edge is visited.
#[allow(non_camel_case_types)]
pub struct vtkGenericEdgeTable(*mut core::ffi::c_void);
impl vtkGenericEdgeTable {
    /// Creates a new [vtkGenericEdgeTable] wrapped inside `vtkNew`
    #[doc(alias = "vtkGenericEdgeTable")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGenericEdgeTable_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGenericEdgeTable_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGenericEdgeTable_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGenericEdgeTable_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGenericEdgeTable {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGenericEdgeTable {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGenericEdgeTable_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGenericEdgeTable_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGenericEdgeTable_create_drop() {
    let obj = vtkGenericEdgeTable::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGenericEdgeTable(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Interface for obtaining
///
/// interpolated velocity values
///
/// vtkGenericInterpolatedVelocityField acts as a continuous velocity field
/// by performing cell interpolation on the underlying vtkDataSet.
/// This is a concrete sub-class of vtkFunctionSet with
/// NumberOfIndependentVariables = 4 (x,y,z,t) and
/// NumberOfFunctions = 3 (u,v,w). Normally, every time an evaluation
/// is performed, the cell which contains the point (x,y,z) has to
/// be found by calling FindCell. This is a computationally expansive
/// operation. In certain cases, the cell search can be avoided or shortened
/// by providing a guess for the cell iterator. For example, in streamline
/// integration, the next evaluation is usually in the same or a neighbour
/// cell. For this reason, vtkGenericInterpolatedVelocityField stores the last
/// cell iterator. If caching is turned on, it uses this iterator as the
/// starting point.
///
/// @warning
/// vtkGenericInterpolatedVelocityField is not thread safe. A new instance
/// should be created by each thread.
///
/// @sa
/// vtkFunctionSet vtkGenericStreamTracer
#[allow(non_camel_case_types)]
pub struct vtkGenericInterpolatedVelocityField(*mut core::ffi::c_void);
impl vtkGenericInterpolatedVelocityField {
    /// Creates a new [vtkGenericInterpolatedVelocityField] wrapped inside `vtkNew`
    #[doc(alias = "vtkGenericInterpolatedVelocityField")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGenericInterpolatedVelocityField_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGenericInterpolatedVelocityField_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGenericInterpolatedVelocityField_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGenericInterpolatedVelocityField_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGenericInterpolatedVelocityField {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGenericInterpolatedVelocityField {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGenericInterpolatedVelocityField_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkGenericInterpolatedVelocityField_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGenericInterpolatedVelocityField_create_drop() {
    let obj = vtkGenericInterpolatedVelocityField::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGenericInterpolatedVelocityField(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects that compute
///
/// geometry-based error during cell tessellation.
///
///
/// It is a concrete error metric, based on a geometric criterium:
/// the variation of the edge from a straight line.
///
/// @sa
/// vtkGenericCellTessellator vtkGenericSubdivisionErrorMetric
#[allow(non_camel_case_types)]
pub struct vtkGeometricErrorMetric(*mut core::ffi::c_void);
impl vtkGeometricErrorMetric {
    /// Creates a new [vtkGeometricErrorMetric] wrapped inside `vtkNew`
    #[doc(alias = "vtkGeometricErrorMetric")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGeometricErrorMetric_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGeometricErrorMetric_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGeometricErrorMetric_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGeometricErrorMetric_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGeometricErrorMetric {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGeometricErrorMetric {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGeometricErrorMetric_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGeometricErrorMetric_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGeometricErrorMetric_create_drop() {
    let obj = vtkGeometricErrorMetric::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGeometricErrorMetric(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Representation of a single graph edge.
///
///
///
/// A heavy-weight (vtkObject subclass) graph edge object that may be used
/// instead of the vtkEdgeType struct, for use with wrappers.
/// The edge contains the source and target vertex ids, and the edge id.
///
/// @sa
/// vtkGraph
#[allow(non_camel_case_types)]
pub struct vtkGraphEdge(*mut core::ffi::c_void);
impl vtkGraphEdge {
    /// Creates a new [vtkGraphEdge] wrapped inside `vtkNew`
    #[doc(alias = "vtkGraphEdge")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGraphEdge_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGraphEdge_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGraphEdge_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGraphEdge_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGraphEdge {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGraphEdge {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGraphEdge_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGraphEdge_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGraphEdge_create_drop() {
    let obj = vtkGraphEdge::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGraphEdge(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Internal representation of vtkGraph
///
///
///
/// This is the internal representation of vtkGraph, used only in rare cases
/// where one must modify that representation.
#[allow(non_camel_case_types)]
pub struct vtkGraphInternals(*mut core::ffi::c_void);
impl vtkGraphInternals {
    /// Creates a new [vtkGraphInternals] wrapped inside `vtkNew`
    #[doc(alias = "vtkGraphInternals")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGraphInternals_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGraphInternals_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGraphInternals_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGraphInternals_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGraphInternals {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGraphInternals {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGraphInternals_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGraphInternals_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGraphInternals_create_drop() {
    let obj = vtkGraphInternals::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGraphInternals(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a 3D cell that represents a prism with
///
/// hexagonal base
///
/// vtkHexagonalPrism is a concrete implementation of vtkCell to represent a
/// linear 3D prism with hexagonal base. Such prism is defined by the twelve points
/// (0-12) where (0,1,2,3,4,5) is the base of the prism which, using the right
/// hand rule, forms a hexagon whose normal points is in the direction of the
/// opposite face (6,7,8,9,10,11).
///
/// @par Thanks:
/// Thanks to Philippe Guerville who developed this class.
/// Thanks to Charles Pignerol (CEA-DAM, France) who ported this class under
/// VTK 4.
/// Thanks to Jean Favre (CSCS, Switzerland) who contributed to integrate this
/// class in VTK.
/// Please address all comments to Jean Favre (jfavre at cscs.ch).
#[allow(non_camel_case_types)]
pub struct vtkHexagonalPrism(*mut core::ffi::c_void);
impl vtkHexagonalPrism {
    /// Creates a new [vtkHexagonalPrism] wrapped inside `vtkNew`
    #[doc(alias = "vtkHexagonalPrism")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHexagonalPrism_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHexagonalPrism_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHexagonalPrism_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHexagonalPrism_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHexagonalPrism {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHexagonalPrism {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHexagonalPrism_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkHexagonalPrism_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHexagonalPrism_create_drop() {
    let obj = vtkHexagonalPrism::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHexagonalPrism(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents a linear 3D hexahedron
///
///
/// vtkHexahedron is a concrete implementation of vtkCell to represent a
/// linear, 3D rectangular hexahedron (e.g., "brick" topology). vtkHexahedron
/// uses the standard isoparametric shape functions for a linear
/// hexahedron. The hexahedron is defined by the eight points (0-7) where
/// (0,1,2,3) is the base of the hexahedron which, using the right hand rule,
/// forms a quadrilaterial whose normal points in the direction of the
/// opposite face (4,5,6,7).
///
/// @sa
/// vtkConvexPointSet vtkPyramid vtkTetra vtkVoxel vtkWedge
#[allow(non_camel_case_types)]
pub struct vtkHexahedron(*mut core::ffi::c_void);
impl vtkHexahedron {
    /// Creates a new [vtkHexahedron] wrapped inside `vtkNew`
    #[doc(alias = "vtkHexahedron")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHexahedron_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHexahedron_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHexahedron_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHexahedron_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHexahedron {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHexahedron {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHexahedron_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkHexahedron_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHexahedron_create_drop() {
    let obj = vtkHexahedron::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHexahedron(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Backwards compatibility class
///
///
///
/// An empty class for backwards compatibility
///
/// @sa
/// vtkUniformGridAM vtkOverlappingAMR vtkNonOverlappingAMR
#[allow(non_camel_case_types)]
pub struct vtkHierarchicalBoxDataSet(*mut core::ffi::c_void);
impl vtkHierarchicalBoxDataSet {
    /// Creates a new [vtkHierarchicalBoxDataSet] wrapped inside `vtkNew`
    #[doc(alias = "vtkHierarchicalBoxDataSet")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHierarchicalBoxDataSet_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHierarchicalBoxDataSet_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHierarchicalBoxDataSet_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHierarchicalBoxDataSet_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHierarchicalBoxDataSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHierarchicalBoxDataSet {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHierarchicalBoxDataSet_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkHierarchicalBoxDataSet_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHierarchicalBoxDataSet_create_drop() {
    let obj = vtkHierarchicalBoxDataSet::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHierarchicalBoxDataSet(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A dataset containing a grid of vtkHyperTree instances
///
/// arranged as a rectilinear grid.
///
///
/// An hypertree grid is a dataset containing a rectilinear grid of root nodes,
/// each of which can be refined as a vtkHyperTree grid. This organization of the
/// root nodes allows for the definition of tree-based AMR grids that do not have
/// uniform geometry.
/// Some filters can be applied on this dataset: contour, outline, geometry.
///
/// The order and number of points must match that specified by the dimensions
/// of the grid. The point order increases in i fastest (from 0<=i<dims[0]),
/// then j (0<=j<dims[1]), then k (0<=k<dims[2]) where dims[] are the
/// dimensions of the grid in the i-j-k topological directions. The number of
/// points is dims[0]*dims[1]*dims[2]. The same is true for the cells of the
/// grid. The order and number of cells must match that specified by the
/// dimensions of the grid. The cell order increases in i fastest (from
/// 0<=i<(dims[0]-1)), then j (0<=j<(dims[1]-1)), then k (0<=k<(dims[2]-1))
/// The number of cells is (dims[0]-1)*(dims[1]-1)*(dims[2]-1).
///
/// Dimensions : number of points by direction of rectilinear grid
/// CellDims : number of cells by directions of rectilinear grid
/// (1 for each dimensions 1)
///
/// @warning
/// It is not a spatial search object. If you are looking for this kind of
/// octree see vtkCellLocator instead.
/// Extent support is not finished yet.
///
/// @sa
/// vtkHyperTree vtkRectilinearGrid
///
/// @par Thanks:
/// This class was written by Philippe Pebay, Joachim Pouderoux, and Charles Law, Kitware 2013
/// This class was modified by Guenole Harel and Jacques-Bernard Lekien 2014
/// This class was rewritten by Philippe Pebay, 2016
/// This class was modified by Jacques-Bernard Lekien 2018
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGrid(*mut core::ffi::c_void);
impl vtkHyperTreeGrid {
    /// Creates a new [vtkHyperTreeGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkHyperTreeGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGrid_create_drop() {
    let obj = vtkHyperTreeGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// class that implements accelerated searches through HyperTree Grids (HTGs) using geometric
///
/// information
///
/// The goal of this class is to implement a geometric locator search through the HTG structure. Its
/// main feature should be to expose a generic interface to finding the HTG cells that contain a
/// given geometric object. The search through the HTG is implemented using a
/// vtkHyperTreeGridNonOrientedGeometricCursor. The arborescent structure of the HTG should be
/// sufficient to accelerate the search and achieve good performance in general.
///
/// All methods in this class should be thread safe since it is meant to be used in a multi-threaded
/// environment out of the box (except SetHTG which should be called outside any multi-threaded
/// setting).
///
/// @sa
/// vtkHyperTreeGridLocator, vtkHyperTreeGrid, vtkHyperTree, vtkHyperTreeGridOrientedCursor,
/// vtkHyperTreeGridNonOrientedCursor
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridGeometricLocator(*mut core::ffi::c_void);
impl vtkHyperTreeGridGeometricLocator {
    /// Creates a new [vtkHyperTreeGridGeometricLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridGeometricLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridGeometricLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridGeometricLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridGeometricLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridGeometricLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridGeometricLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridGeometricLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridGeometricLocator_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHyperTreeGridGeometricLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridGeometricLocator_create_drop() {
    let obj = vtkHyperTreeGridGeometricLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridGeometricLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// Non-oriented cursors have the ability to come back to their parents
/// and to go to the root.
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written for more optimisation by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridNonOrientedCursor(*mut core::ffi::c_void);
impl vtkHyperTreeGridNonOrientedCursor {
    /// Creates a new [vtkHyperTreeGridNonOrientedCursor] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridNonOrientedCursor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedCursor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridNonOrientedCursor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedCursor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridNonOrientedCursor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridNonOrientedCursor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridNonOrientedCursor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedCursor_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHyperTreeGridNonOrientedCursor_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridNonOrientedCursor_create_drop() {
    let obj = vtkHyperTreeGridNonOrientedCursor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridNonOrientedCursor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// NonOriented ne peut pas remonter plus haut qu'a sa creation.
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// Geometry cursors allow to retrieve origin, size, bounds
/// and central points
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written for more optimisation by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridNonOrientedGeometryCursor(*mut core::ffi::c_void);
impl vtkHyperTreeGridNonOrientedGeometryCursor {
    /// Creates a new [vtkHyperTreeGridNonOrientedGeometryCursor] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridNonOrientedGeometryCursor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedGeometryCursor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridNonOrientedGeometryCursor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedGeometryCursor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridNonOrientedGeometryCursor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridNonOrientedGeometryCursor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridNonOrientedGeometryCursor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedGeometryCursor_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHyperTreeGridNonOrientedGeometryCursor_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridNonOrientedGeometryCursor_create_drop() {
    let obj = vtkHyperTreeGridNonOrientedGeometryCursor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridNonOrientedGeometryCursor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// This supercursor allows to visit all neighbors including diagonal ones.
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written and optimized by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridNonOrientedMooreSuperCursor(*mut core::ffi::c_void);
impl vtkHyperTreeGridNonOrientedMooreSuperCursor {
    /// Creates a new [vtkHyperTreeGridNonOrientedMooreSuperCursor] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridNonOrientedMooreSuperCursor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedMooreSuperCursor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridNonOrientedMooreSuperCursor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedMooreSuperCursor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridNonOrientedMooreSuperCursor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridNonOrientedMooreSuperCursor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridNonOrientedMooreSuperCursor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedMooreSuperCursor_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHyperTreeGridNonOrientedMooreSuperCursor_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridNonOrientedMooreSuperCursor_create_drop() {
    let obj = vtkHyperTreeGridNonOrientedMooreSuperCursor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridNonOrientedMooreSuperCursor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written and optimized by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridNonOrientedMooreSuperCursorLight(*mut core::ffi::c_void);
impl vtkHyperTreeGridNonOrientedMooreSuperCursorLight {
    /// Creates a new [vtkHyperTreeGridNonOrientedMooreSuperCursorLight] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridNonOrientedMooreSuperCursorLight")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedMooreSuperCursorLight_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridNonOrientedMooreSuperCursorLight_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedMooreSuperCursorLight_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridNonOrientedMooreSuperCursorLight_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridNonOrientedMooreSuperCursorLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridNonOrientedMooreSuperCursorLight {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedMooreSuperCursorLight_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHyperTreeGridNonOrientedMooreSuperCursorLight_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridNonOrientedMooreSuperCursorLight_create_drop() {
    let obj = vtkHyperTreeGridNonOrientedMooreSuperCursorLight::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridNonOrientedMooreSuperCursorLight(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// NonOriented ne peut pas remonter plus haut qu'a sa creation.
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// Geometry cursors allow to retrieve origin, size, bounds
/// and central points
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written for more optimisation by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor(*mut core::ffi::c_void);
impl vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor {
    /// Creates a new [vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor_create_drop() {
    let obj = vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridNonOrientedUnlimitedGeometryCursor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Specific Moore super cursor that can subdivied neighborhood
///
///
/// This supercursor behave like the Moore supercursor but relies on the
/// vtkHyperTreeGridNonOrientedUnlimitedSuperCursor so the neighborhood
/// can be refined to reach the level of the current cell in any case.
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid vtkHyperTreeGridNonOrientedMooreSuperCursor
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor(*mut core::ffi::c_void);
impl vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor {
    /// Creates a new [vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_destructor(self.0)
        }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor_create_drop() {
    let obj = vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridNonOrientedUnlimitedMooreSuperCursor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// This supercursor allows to traverse neighbors attached to coface of
/// the current position.
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written and optimized by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridNonOrientedVonNeumannSuperCursor(*mut core::ffi::c_void);
impl vtkHyperTreeGridNonOrientedVonNeumannSuperCursor {
    /// Creates a new [vtkHyperTreeGridNonOrientedVonNeumannSuperCursor] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridNonOrientedVonNeumannSuperCursor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridNonOrientedVonNeumannSuperCursor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridNonOrientedVonNeumannSuperCursor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridNonOrientedVonNeumannSuperCursor_create_drop() {
    let obj = vtkHyperTreeGridNonOrientedVonNeumannSuperCursor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridNonOrientedVonNeumannSuperCursor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written and optimized by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight(*mut core::ffi::c_void);
impl vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight {
    /// Creates a new [vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe {
            &mut *vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_new()
        })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_destructor(self.0)
        }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight_create_drop() {
    let obj = vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridNonOrientedVonNeumannSuperCursorLight(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// oriented cursors are used for simple recursive DFS. A cursor has no
/// knowledge of its parent, only its children.
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written for more optimisation by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridOrientedCursor(*mut core::ffi::c_void);
impl vtkHyperTreeGridOrientedCursor {
    /// Creates a new [vtkHyperTreeGridOrientedCursor] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridOrientedCursor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridOrientedCursor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridOrientedCursor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridOrientedCursor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridOrientedCursor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridOrientedCursor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridOrientedCursor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridOrientedCursor_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkHyperTreeGridOrientedCursor_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridOrientedCursor_create_drop() {
    let obj = vtkHyperTreeGridOrientedCursor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridOrientedCursor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects for traversal a HyperTreeGrid.
///
///
/// NonOriented ne peut pas remonter plus haut qu'a sa creation.
/// Objects that can perform depth traversal of a hyper tree grid,
/// take into account more parameters (related to the grid structure) than
/// the compact hyper tree cursor implemented in vtkHyperTree can.
/// This is an abstract class.
/// Cursors are created by the HyperTreeGrid implementation.
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid
///
/// @par Thanks:
/// This class was written by Guenole Harel and Jacques-Bernard Lekien, 2014.
/// This class was re-written by Philippe Pebay, 2016.
/// This class was re-written for more optimisation by Jacques-Bernard Lekien,
/// Guenole Harel and Jerome Dubois, 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridOrientedGeometryCursor(*mut core::ffi::c_void);
impl vtkHyperTreeGridOrientedGeometryCursor {
    /// Creates a new [vtkHyperTreeGridOrientedGeometryCursor] wrapped inside `vtkNew`
    #[doc(alias = "vtkHyperTreeGridOrientedGeometryCursor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridOrientedGeometryCursor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHyperTreeGridOrientedGeometryCursor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHyperTreeGridOrientedGeometryCursor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHyperTreeGridOrientedGeometryCursor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHyperTreeGridOrientedGeometryCursor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridOrientedGeometryCursor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridOrientedGeometryCursor_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHyperTreeGridOrientedGeometryCursor_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridOrientedGeometryCursor_create_drop() {
    let obj = vtkHyperTreeGridOrientedGeometryCursor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHyperTreeGridOrientedGeometryCursor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// topologically and geometrically regular array of data
///
///
/// vtkImageData is a data object that is a concrete implementation of
/// vtkDataSet. vtkImageData represents a geometric structure that is
/// a topological and geometrical regular array of points. Examples include
/// volumes (voxel data) and pixmaps. This representation supports images
/// up to three dimensions. The image may also be oriented (see the
/// DirectionMatrices and related transformation methods). Note however,
/// that not all filters support oriented images.
///
/// @sa
/// vtkImageTransform
#[allow(non_camel_case_types)]
pub struct vtkImageData(*mut core::ffi::c_void);
impl vtkImageData {
    /// Creates a new [vtkImageData] wrapped inside `vtkNew`
    #[doc(alias = "vtkImageData")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImageData_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImageData_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImageData_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImageData_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImageData {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImageData {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImageData_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImageData_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImageData_create_drop() {
    let obj = vtkImageData::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImageData(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// helper class to transform output of non-axis-aligned images
///
///
/// vtkImageTransform is a helper class to transform the output of
/// image filters (i.e., filter that input vtkImageData) by applying the
/// Index to Physical transformation from the input image, which can
/// include origin, spacing, direction. The transformation process is
/// threaded with vtkSMPTools for performance.
///
/// Typically in application the single method TransformPointSet() is
/// invoked to transform the output of an image algorithm (assuming
/// that the image's direction/orientation matrix is non-identity).
/// Note that vtkPointSets encompass vtkPolyData as well
/// as vtkUnstructuredGrids. In the future other output types may be
/// added. Note that specific methods for transforming points, normals,
/// and vectors is also provided by this class in case additional
/// output data arrays need to be transformed (since
/// TransformPointSet() only processes data arrays labeled as points,
/// normals, and vectors).
///
/// @warning
/// This class assumes that any vectors are gradients, and vector arrays
/// will therefore be transformed by first dividing by the spacing and
/// then applying the inverse transpose of the direction matrix.
///
/// @warning
/// This class has been threaded with vtkSMPTools. Using TBB or other
/// non-sequential type (set in the CMake variable
/// VTK_SMP_IMPLEMENTATION_TYPE) may improve performance significantly.
#[allow(non_camel_case_types)]
pub struct vtkImageTransform(*mut core::ffi::c_void);
impl vtkImageTransform {
    /// Creates a new [vtkImageTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkImageTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImageTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImageTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImageTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImageTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImageTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImageTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImageTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImageTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImageTransform_create_drop() {
    let obj = vtkImageTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImageTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function consisting of boolean combinations of implicit functions
///
///
/// vtkImplicitBoolean is an implicit function consisting of boolean
/// combinations of implicit functions. The class has a list of functions
/// (FunctionList) that are combined according to a specified operator
/// (VTK_UNION or VTK_INTERSECTION or VTK_DIFFERENCE). You can use nested
/// combinations of vtkImplicitFunction's (and/or vtkImplicitBoolean) to create
/// elaborate implicit functions.  vtkImplicitBoolean is a concrete
/// implementation of vtkImplicitFunction.
///
/// The operators work as follows. The VTK_UNION operator takes the minimum
/// value of all implicit functions. The VTK_INTERSECTION operator takes the
/// maximum value of all implicit functions. The VTK_DIFFERENCE operator
/// subtracts the 2nd through last implicit functions from the first. The
/// VTK_UNION_OF_MAGNITUDES takes the minimum absolute value of the
/// implicit functions.
#[allow(non_camel_case_types)]
pub struct vtkImplicitBoolean(*mut core::ffi::c_void);
impl vtkImplicitBoolean {
    /// Creates a new [vtkImplicitBoolean] wrapped inside `vtkNew`
    #[doc(alias = "vtkImplicitBoolean")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImplicitBoolean_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImplicitBoolean_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImplicitBoolean_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImplicitBoolean_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImplicitBoolean {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImplicitBoolean {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImplicitBoolean_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImplicitBoolean_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImplicitBoolean_create_drop() {
    let obj = vtkImplicitBoolean::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImplicitBoolean(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// treat a dataset as if it were an implicit function
///
///
/// vtkImplicitDataSet treats any type of dataset as if it were an
/// implicit function. This means it computes a function value and
/// gradient. vtkImplicitDataSet is a concrete implementation of
/// vtkImplicitFunction.
///
/// vtkImplicitDataSet computes the function (at the point x) by performing
/// cell interpolation. That is, it finds the cell containing x, and then
/// uses the cell's interpolation functions to compute an interpolated
/// scalar value at x. (A similar approach is used to find the
/// gradient, if requested.) Points outside of the dataset are assigned
/// the value of the ivar OutValue, and the gradient value OutGradient.
///
/// @warning
/// Any type of dataset can be used as an implicit function as long as it
/// has scalar data associated with it.
///
/// @sa
/// vtkImplicitFunction vtkImplicitVolume vtkClipPolyData vtkCutter
/// vtkImplicitWindowFunction
#[allow(non_camel_case_types)]
pub struct vtkImplicitDataSet(*mut core::ffi::c_void);
impl vtkImplicitDataSet {
    /// Creates a new [vtkImplicitDataSet] wrapped inside `vtkNew`
    #[doc(alias = "vtkImplicitDataSet")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImplicitDataSet_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImplicitDataSet_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImplicitDataSet_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImplicitDataSet_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImplicitDataSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImplicitDataSet {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImplicitDataSet_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImplicitDataSet_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImplicitDataSet_create_drop() {
    let obj = vtkImplicitDataSet::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImplicitDataSet(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain a list of implicit functions
///
///
/// vtkImplicitFunctionCollection is an object that creates and manipulates
/// lists of objects of type vtkImplicitFunction.
/// @sa
/// vtkCollection vtkPlaneCollection
#[allow(non_camel_case_types)]
pub struct vtkImplicitFunctionCollection(*mut core::ffi::c_void);
impl vtkImplicitFunctionCollection {
    /// Creates a new [vtkImplicitFunctionCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkImplicitFunctionCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImplicitFunctionCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImplicitFunctionCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImplicitFunctionCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImplicitFunctionCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImplicitFunctionCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImplicitFunctionCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImplicitFunctionCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImplicitFunctionCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImplicitFunctionCollection_create_drop() {
    let obj = vtkImplicitFunctionCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImplicitFunctionCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for an halo
///
///
/// vtkImplicitHalo evaluates to 1.0 for each position in the sphere of a
/// given center and radius Radius*(1-FadeOut). It evaluates to 0.0 for each
/// position out the sphere of a given Center and radius Radius. It fades out
/// linearly from 1.0 to 0.0 for points in a radius from Radius*(1-FadeOut) to
/// Radius.
/// vtkImplicitHalo is a concrete implementation of vtkImplicitFunction.
/// It is useful as an input to
/// vtkSampleFunction to generate an 2D image of an halo. It is used this way by
/// vtkShadowMapPass.
/// @warning
/// It does not implement the gradient.
#[allow(non_camel_case_types)]
pub struct vtkImplicitHalo(*mut core::ffi::c_void);
impl vtkImplicitHalo {
    /// Creates a new [vtkImplicitHalo] wrapped inside `vtkNew`
    #[doc(alias = "vtkImplicitHalo")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImplicitHalo_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImplicitHalo_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImplicitHalo_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImplicitHalo_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImplicitHalo {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImplicitHalo {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImplicitHalo_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImplicitHalo_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImplicitHalo_create_drop() {
    let obj = vtkImplicitHalo::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImplicitHalo(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a selection loop
///
///
/// vtkImplicitSelectionLoop computes the implicit function value and
/// function gradient for an irregular, cylinder-like object whose cross
/// section is defined by a set of points forming a loop. The loop need
/// not be convex nor its points coplanar. However, the loop must be
/// non-self-intersecting when projected onto the plane defined by the
/// accumulated cross product around the loop (i.e., the axis of the
/// loop). (Alternatively, you can specify the normal to use.)
///
/// The following procedure is used to compute the implicit function
/// value for a point x. Each point of the loop is first projected onto
/// the plane defined by the loop normal. This forms a polygon. Then,
/// to evaluate the implicit function value, inside/outside tests are
/// used to determine if x is inside the polygon, and the distance to
/// the loop boundary is computed (negative values are inside the
/// loop).
///
/// One example application of this implicit function class is to draw a
/// loop on the surface of a mesh, and use the loop to clip or extract
/// cells from within the loop. Remember, the selection loop is "infinite"
/// in length, you can use a plane (in boolean combination) to cap the extent
/// of the selection loop. Another trick is to use a connectivity filter to
/// extract the closest region to a given point (i.e., one of the points used
/// to define the selection loop).
///
/// @sa
/// vtkImplicitFunction vtkImplicitBoolean vtkExtractGeometry vtkClipPolyData
/// vtkConnectivityFilter vtkPolyDataConnectivityFilter
#[allow(non_camel_case_types)]
pub struct vtkImplicitSelectionLoop(*mut core::ffi::c_void);
impl vtkImplicitSelectionLoop {
    /// Creates a new [vtkImplicitSelectionLoop] wrapped inside `vtkNew`
    #[doc(alias = "vtkImplicitSelectionLoop")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImplicitSelectionLoop_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImplicitSelectionLoop_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImplicitSelectionLoop_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImplicitSelectionLoop_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImplicitSelectionLoop {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImplicitSelectionLoop {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImplicitSelectionLoop_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImplicitSelectionLoop_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImplicitSelectionLoop_create_drop() {
    let obj = vtkImplicitSelectionLoop::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImplicitSelectionLoop(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit sum of other implicit functions
///
///
/// vtkImplicitSum produces a linear combination of other implicit functions.
/// The contribution of each function is weighted by a scalar coefficient.
/// The NormalizeByWeight option normalizes the output so that the
/// scalar weights add up to 1. Note that this function gives accurate
/// sums and gradients only if the input functions are linear.
#[allow(non_camel_case_types)]
pub struct vtkImplicitSum(*mut core::ffi::c_void);
impl vtkImplicitSum {
    /// Creates a new [vtkImplicitSum] wrapped inside `vtkNew`
    #[doc(alias = "vtkImplicitSum")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImplicitSum_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImplicitSum_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImplicitSum_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImplicitSum_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImplicitSum {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImplicitSum {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImplicitSum_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImplicitSum_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImplicitSum_create_drop() {
    let obj = vtkImplicitSum::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImplicitSum(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// treat a volume as if it were an implicit function
///
///
/// vtkImplicitVolume treats a volume (e.g., structured point dataset)
/// as if it were an implicit function. This means it computes a function
/// value and gradient. vtkImplicitVolume is a concrete implementation of
/// vtkImplicitFunction.
///
/// vtkImplicitDataSet computes the function (at the point x) by performing
/// cell interpolation. That is, it finds the cell containing x, and then
/// uses the cell's interpolation functions to compute an interpolated
/// scalar value at x. (A similar approach is used to find the
/// gradient, if requested.) Points outside of the dataset are assigned
/// the value of the ivar OutValue, and the gradient value OutGradient.
///
/// @warning
/// The input volume data is only updated when GetMTime() is called.
/// Works for 3D structured points datasets, 0D-2D datasets won't work properly.
///
/// @sa
/// vtkImplicitFunction vtkImplicitDataSet vtkClipPolyData vtkCutter
/// vtkImplicitWindowFunction
#[allow(non_camel_case_types)]
pub struct vtkImplicitVolume(*mut core::ffi::c_void);
impl vtkImplicitVolume {
    /// Creates a new [vtkImplicitVolume] wrapped inside `vtkNew`
    #[doc(alias = "vtkImplicitVolume")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImplicitVolume_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImplicitVolume_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImplicitVolume_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImplicitVolume_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImplicitVolume {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImplicitVolume {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImplicitVolume_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImplicitVolume_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImplicitVolume_create_drop() {
    let obj = vtkImplicitVolume::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImplicitVolume(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function maps another implicit function to lie within a specified range
///
///
/// vtkImplicitWindowFunction is used to modify the output of another
/// implicit function to lie within a specified "window", or function
/// range. This can be used to add "thickness" to cutting or clipping
/// functions.
///
/// This class works as follows. First, it evaluates the function value of the
/// user-specified implicit function. Then, based on the window range specified,
/// it maps the function value into the window values specified.
///
///
/// @sa
/// vtkImplicitFunction
#[allow(non_camel_case_types)]
pub struct vtkImplicitWindowFunction(*mut core::ffi::c_void);
impl vtkImplicitWindowFunction {
    /// Creates a new [vtkImplicitWindowFunction] wrapped inside `vtkNew`
    #[doc(alias = "vtkImplicitWindowFunction")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImplicitWindowFunction_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImplicitWindowFunction_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImplicitWindowFunction_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImplicitWindowFunction_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImplicitWindowFunction {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImplicitWindowFunction {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImplicitWindowFunction_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImplicitWindowFunction_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImplicitWindowFunction_create_drop() {
    let obj = vtkImplicitWindowFunction::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImplicitWindowFunction(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Iterates through all incoming edges to a vertex.
///
///
///
/// vtkInEdgeIterator iterates through all edges whose target is a particular
/// vertex. Instantiate this class directly and call Initialize() to traverse
/// the vertex of a graph. Alternately, use GetInEdges() on the graph to
/// initialize the iterator. it->Next() returns a vtkInEdgeType structure,
/// which contains Id, the edge's id, and Source, the edge's source vertex.
///
/// @sa
/// vtkGraph vtkOutEdgeIterator
#[allow(non_camel_case_types)]
pub struct vtkInEdgeIterator(*mut core::ffi::c_void);
impl vtkInEdgeIterator {
    /// Creates a new [vtkInEdgeIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkInEdgeIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInEdgeIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkInEdgeIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkInEdgeIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkInEdgeIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkInEdgeIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInEdgeIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInEdgeIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInEdgeIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInEdgeIterator_create_drop() {
    let obj = vtkInEdgeIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkInEdgeIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Octree node constituting incremental
///
/// octree (in support of both point location and point insertion)
///
///
/// Octree nodes serve as spatial sub-division primitives to build the search
/// structure of an incremental octree in a recursive top-down manner. The
/// hierarchy takes the form of a tree-like representation by which a parent
/// node contains eight mutually non-overlapping child nodes. Each child is
/// assigned with an axis-aligned rectangular volume (Spatial Bounding Box)
/// and the eight children together cover exactly the same region as governed
/// by their parent. The eight child nodes / octants are ordered as
///
/// { (xBBoxMin, xBBoxMid] & (yBBoxMin, yBBoxMid] & (zBBoxMin, zBBoxMid] },
/// { (xBBoxMid, xBBoxMax] & (yBBoxMin, yBBoxMid] & (zBBoxMin, zBBoxMid] },
/// { (xBBoxMin, xBBoxMid] & (yBBoxMid, yBBoxMax] & (zBBoxMin, zBBoxMid] },
/// { (xBBoxMid, xBBoxMax] & (yBBoxMid, yBBoxMax] & (zBBoxMin, zBBoxMid] },
/// { (xBBoxMin, xBBoxMid] & (yBBoxMin, yBBoxMid] & (zBBoxMid, zBBoxMax] },
/// { (xBBoxMid, xBBoxMax] & (yBBoxMin, yBBoxMid] & (zBBoxMid, zBBoxMax] },
/// { (xBBoxMin, xBBoxMid] & (yBBoxMid, yBBoxMax] & (zBBoxMid, zBBoxMax] },
/// { (xBBoxMid, xBBoxMax] & (yBBoxMid, yBBoxMax] & (zBBoxMid, zBBoxMax] },
///
/// where { xrange & yRange & zRange } defines the region of each 3D octant.
/// In addition, the points falling within and registered, by means of point
/// indices, in the parent node are distributed to the child nodes for delegated
/// maintenance. In fact, only leaf nodes, i.e., those without any descendants,
/// actually store point indices while each node, regardless of a leaf or non-
/// leaf node, keeps a dynamically updated Data Bounding Box of the inhabitant
/// points, if any. Given a maximum number of points per leaf node, an octree
/// is initialized with an empty leaf node that is then recursively sub-divided,
/// but only on demand as points are incrementally inserted, to construct a
/// populated tree.
///
/// Please note that this octree node class is able to handle a large number
/// of EXACTLY duplicate points that is greater than the specified maximum
/// number of points per leaf node. In other words, as an exception, a leaf
/// node may maintain an arbitrary number of exactly duplicate points to deal
/// with possible extreme cases.
///
/// @sa
/// vtkIncrementalOctreePointLocator
#[allow(non_camel_case_types)]
pub struct vtkIncrementalOctreeNode(*mut core::ffi::c_void);
impl vtkIncrementalOctreeNode {
    /// Creates a new [vtkIncrementalOctreeNode] wrapped inside `vtkNew`
    #[doc(alias = "vtkIncrementalOctreeNode")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIncrementalOctreeNode_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkIncrementalOctreeNode_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkIncrementalOctreeNode_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkIncrementalOctreeNode_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkIncrementalOctreeNode {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIncrementalOctreeNode {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIncrementalOctreeNode_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIncrementalOctreeNode_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIncrementalOctreeNode_create_drop() {
    let obj = vtkIncrementalOctreeNode::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkIncrementalOctreeNode(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Incremental octree in support
///
/// of both point location and point insertion.
///
///
/// As opposed to the uniform bin-based search structure (adopted in class
/// vtkPointLocator) with a fixed spatial resolution, an octree mechanism
/// employs a hierarchy of tree-like sub-division of the 3D data domain. Thus
/// it enables data-aware multi-resolution and accordingly accelerated point
/// location as well as insertion, particularly when handling a radically
/// imbalanced layout of points as not uncommon in datasets defined on
/// adaptive meshes. Compared to a static point locator supporting pure
/// location functionalities through some search structure established from
/// a fixed set of points, an incremental point locator allows for, in addition,
/// point insertion capabilities, with the search structure maintaining a
/// dynamically increasing number of points.
/// Class vtkIncrementalOctreePointLocator is an octree-based accelerated
/// implementation of the functionalities of the uniform bin-based incremental
/// point locator vtkPointLocator. For point location, an octree is built by
/// accessing a vtkDataSet, specifically a vtkPointSet. For point insertion,
/// an empty octree is inited and then incrementally populated as points are
/// inserted. Three increasingly complex point insertion modes, i.e., direct
/// check-free insertion, zero tolerance insertion, and non-zero tolerance
/// insertion, are supported. In fact, the octree used in the point location
/// mode is actually constructed via direct check-free point insertion. This
/// class also provides a polygonal representation of the octree boundary.
///
/// @sa
/// vtkAbstractPointLocator, vtkIncrementalPointLocator, vtkPointLocator,
/// vtkMergePoints
#[allow(non_camel_case_types)]
pub struct vtkIncrementalOctreePointLocator(*mut core::ffi::c_void);
impl vtkIncrementalOctreePointLocator {
    /// Creates a new [vtkIncrementalOctreePointLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkIncrementalOctreePointLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIncrementalOctreePointLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkIncrementalOctreePointLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkIncrementalOctreePointLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkIncrementalOctreePointLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkIncrementalOctreePointLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIncrementalOctreePointLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIncrementalOctreePointLocator_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkIncrementalOctreePointLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIncrementalOctreePointLocator_create_drop() {
    let obj = vtkIncrementalOctreePointLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkIncrementalOctreePointLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Implementation of the ICP algorithm.
///
///
/// Match two surfaces using the iterative closest point (ICP) algorithm.
/// The core of the algorithm is to match each vertex in one surface with
/// the closest surface point on the other, then apply the transformation
/// that modify one surface to best match the other (in a least square sense).
/// This has to be iterated to get proper convergence of the surfaces.
/// @attention
/// Use vtkTransformPolyDataFilter to apply the resulting ICP transform to
/// your data. You might also set it to your actor's user transform.
/// @attention
/// This class makes use of vtkLandmarkTransform internally to compute the
/// best fit. Use the GetLandmarkTransform member to get a pointer to that
/// transform and set its parameters. You might, for example, constrain the
/// number of degrees of freedom of the solution (i.e. rigid body, similarity,
/// etc.) by checking the vtkLandmarkTransform documentation for its SetMode
/// member.
/// @sa
/// vtkLandmarkTransform
#[allow(non_camel_case_types)]
pub struct vtkIterativeClosestPointTransform(*mut core::ffi::c_void);
impl vtkIterativeClosestPointTransform {
    /// Creates a new [vtkIterativeClosestPointTransform] wrapped inside `vtkNew`
    #[doc(alias = "vtkIterativeClosestPointTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIterativeClosestPointTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkIterativeClosestPointTransform_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkIterativeClosestPointTransform_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkIterativeClosestPointTransform_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkIterativeClosestPointTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIterativeClosestPointTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIterativeClosestPointTransform_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkIterativeClosestPointTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIterativeClosestPointTransform_create_drop() {
    let obj = vtkIterativeClosestPointTransform::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkIterativeClosestPointTransform(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// This class represents a single spatial region
///
/// in an 3D axis aligned binary spatial partitioning.  It is assumed
/// the region bounds some set of points.  Regions are represented
/// as nodes in a binary tree.
///
///
///
/// @sa
/// vtkKdTree vtkOBSPCuts
#[allow(non_camel_case_types)]
pub struct vtkKdNode(*mut core::ffi::c_void);
impl vtkKdNode {
    /// Creates a new [vtkKdNode] wrapped inside `vtkNew`
    #[doc(alias = "vtkKdNode")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkKdNode_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkKdNode_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkKdNode_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkKdNode_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkKdNode {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkKdNode {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkKdNode_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkKdNode_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkKdNode_create_drop() {
    let obj = vtkKdNode::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkKdNode(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a Kd-tree spatial decomposition of a set of points
///
///
///
/// Given one or more vtkDataSets, create a load balancing
/// k-d tree decomposition of the points at the center of the cells.
/// Or, create a k-d tree point locator from a list of points.
///
/// This class can also generate a PolyData representation of
/// the boundaries of the spatial regions in the decomposition.
///
/// It can sort the regions with respect to a viewing direction,
/// and it can decompose a list of regions into subsets, each
/// of which represent a convex spatial region (since many algorithms
/// require a convex region).
///
/// If the points were derived from cells, vtkKdTree
/// can create a list of cell Ids for each region for each data set.
/// Two lists are available - all cells with centroid in the region,
/// and all cells that intersect the region but whose centroid lies
/// in another region.
///
/// For the purpose of removing duplicate points quickly from large
/// data sets, or for finding nearby points, we added another mode for
/// building the locator.  BuildLocatorFromPoints will build a k-d tree
/// from one or more vtkPoints objects.  This can be followed by
/// BuildMapForDuplicatePoints which returns a mapping from the original
/// ids to a subset of the ids that is unique within a supplied
/// tolerance, or you can use FindPoint and FindClosestPoint to
/// locate points in the original set that the tree was built from.
///
/// @sa
/// vtkLocator vtkCellLocator vtkPKdTree
#[allow(non_camel_case_types)]
pub struct vtkKdTree(*mut core::ffi::c_void);
impl vtkKdTree {
    /// Creates a new [vtkKdTree] wrapped inside `vtkNew`
    #[doc(alias = "vtkKdTree")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkKdTree_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkKdTree_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkKdTree_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkKdTree_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkKdTree {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkKdTree {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkKdTree_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkKdTree_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkKdTree_create_drop() {
    let obj = vtkKdTree::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkKdTree(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// class to quickly locate points in 3-space
///
///
/// vtkKdTreePointLocator is a wrapper class that derives from
/// vtkAbstractPointLocator and calls the search functions in vtkKdTree.
///
/// @sa
/// vtkKdTree
#[allow(non_camel_case_types)]
pub struct vtkKdTreePointLocator(*mut core::ffi::c_void);
impl vtkKdTreePointLocator {
    /// Creates a new [vtkKdTreePointLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkKdTreePointLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkKdTreePointLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkKdTreePointLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkKdTreePointLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkKdTreePointLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkKdTreePointLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkKdTreePointLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkKdTreePointLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkKdTreePointLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkKdTreePointLocator_create_drop() {
    let obj = vtkKdTreePointLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkKdTreePointLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkLagrangeCurve(*mut core::ffi::c_void);
impl vtkLagrangeCurve {
    /// Creates a new [vtkLagrangeCurve] wrapped inside `vtkNew`
    #[doc(alias = "vtkLagrangeCurve")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLagrangeCurve_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLagrangeCurve_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLagrangeCurve_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLagrangeCurve_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLagrangeCurve {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLagrangeCurve {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLagrangeCurve_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLagrangeCurve_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLagrangeCurve_create_drop() {
    let obj = vtkLagrangeCurve::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLagrangeCurve(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 3D cell that represents an arbitrary order Lagrange hex
///
///
/// vtkLagrangeHexahedron is a concrete implementation of vtkCell to represent a
/// 3D hexahedron using Lagrange shape functions of user specified order.
///
/// @sa
/// vtkHexahedron
#[allow(non_camel_case_types)]
pub struct vtkLagrangeHexahedron(*mut core::ffi::c_void);
impl vtkLagrangeHexahedron {
    /// Creates a new [vtkLagrangeHexahedron] wrapped inside `vtkNew`
    #[doc(alias = "vtkLagrangeHexahedron")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLagrangeHexahedron_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLagrangeHexahedron_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLagrangeHexahedron_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLagrangeHexahedron_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLagrangeHexahedron {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLagrangeHexahedron {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLagrangeHexahedron_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLagrangeHexahedron_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLagrangeHexahedron_create_drop() {
    let obj = vtkLagrangeHexahedron::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLagrangeHexahedron(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkLagrangeInterpolation(*mut core::ffi::c_void);
impl vtkLagrangeInterpolation {
    /// Creates a new [vtkLagrangeInterpolation] wrapped inside `vtkNew`
    #[doc(alias = "vtkLagrangeInterpolation")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLagrangeInterpolation_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLagrangeInterpolation_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLagrangeInterpolation_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLagrangeInterpolation_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLagrangeInterpolation {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLagrangeInterpolation {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLagrangeInterpolation_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLagrangeInterpolation_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLagrangeInterpolation_create_drop() {
    let obj = vtkLagrangeInterpolation::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLagrangeInterpolation(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkLagrangeQuadrilateral(*mut core::ffi::c_void);
impl vtkLagrangeQuadrilateral {
    /// Creates a new [vtkLagrangeQuadrilateral] wrapped inside `vtkNew`
    #[doc(alias = "vtkLagrangeQuadrilateral")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLagrangeQuadrilateral_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLagrangeQuadrilateral_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLagrangeQuadrilateral_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLagrangeQuadrilateral_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLagrangeQuadrilateral {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLagrangeQuadrilateral {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLagrangeQuadrilateral_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLagrangeQuadrilateral_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLagrangeQuadrilateral_create_drop() {
    let obj = vtkLagrangeQuadrilateral::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLagrangeQuadrilateral(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 3D cell that represents an arbitrary order Lagrange tetrahedron
///
///
/// vtkLagrangeTetra is a concrete implementation of vtkCell to represent a
/// 3D tetrahedron using Lagrange shape functions of user specified order.
///
/// The number of points in a Lagrange cell determines the order over which they
/// are iterated relative to the parametric coordinate system of the cell. The
/// first points that are reported are vertices. They appear in the same order in
/// which they would appear in linear cells. Mid-edge points are reported next.
/// They are reported in sequence. For two- and three-dimensional (3D) cells, the
/// following set of points to be reported are face points. Finally, 3D cells
/// report points interior to their volume.
#[allow(non_camel_case_types)]
pub struct vtkLagrangeTetra(*mut core::ffi::c_void);
impl vtkLagrangeTetra {
    /// Creates a new [vtkLagrangeTetra] wrapped inside `vtkNew`
    #[doc(alias = "vtkLagrangeTetra")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLagrangeTetra_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLagrangeTetra_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLagrangeTetra_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLagrangeTetra_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLagrangeTetra {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLagrangeTetra {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLagrangeTetra_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLagrangeTetra_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLagrangeTetra_create_drop() {
    let obj = vtkLagrangeTetra::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLagrangeTetra(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 2D cell that represents an arbitrary order Lagrange triangle
///
///
/// vtkLagrangeTriangle is a concrete implementation of vtkCell to represent a
/// 2D triangle using Lagrange shape functions of user specified order.
///
/// The number of points in a Lagrange cell determines the order over which they
/// are iterated relative to the parametric coordinate system of the cell. The
/// first points that are reported are vertices. They appear in the same order in
/// which they would appear in linear cells. Mid-edge points are reported next.
/// They are reported in sequence. For two- and three-dimensional (3D) cells, the
/// following set of points to be reported are face points. Finally, 3D cells
/// report points interior to their volume.
#[allow(non_camel_case_types)]
pub struct vtkLagrangeTriangle(*mut core::ffi::c_void);
impl vtkLagrangeTriangle {
    /// Creates a new [vtkLagrangeTriangle] wrapped inside `vtkNew`
    #[doc(alias = "vtkLagrangeTriangle")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLagrangeTriangle_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLagrangeTriangle_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLagrangeTriangle_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLagrangeTriangle_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLagrangeTriangle {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLagrangeTriangle {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLagrangeTriangle_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLagrangeTriangle_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLagrangeTriangle_create_drop() {
    let obj = vtkLagrangeTriangle::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLagrangeTriangle(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 3D cell that represents an arbitrary order Lagrange wedge
///
///
/// vtkLagrangeWedge is a concrete implementation of vtkCell to represent a
/// 3D wedge using Lagrange shape functions of user specified order.
/// A wedge consists of two triangular and three quadrilateral faces.
/// The first six points of the wedge (0-5) are the "corner" points
/// where the first three points are the base of the wedge. This wedge
/// point ordering is opposite the vtkWedge ordering though in that
/// the base of the wedge defined by the first three points (0,1,2) form
/// a triangle whose normal points inward (toward the triangular face (3,4,5)).
/// While this is opposite the vtkWedge convention it is consistent with
/// every other cell type in VTK. The first 2 parametric coordinates of the
/// Lagrange wedge or for the triangular base and vary between 0 and 1. The
/// third parametric coordinate is between the two triangular faces and goes
/// from 0 to 1 as well.
#[allow(non_camel_case_types)]
pub struct vtkLagrangeWedge(*mut core::ffi::c_void);
impl vtkLagrangeWedge {
    /// Creates a new [vtkLagrangeWedge] wrapped inside `vtkNew`
    #[doc(alias = "vtkLagrangeWedge")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLagrangeWedge_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLagrangeWedge_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLagrangeWedge_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLagrangeWedge_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLagrangeWedge {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLagrangeWedge {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLagrangeWedge_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLagrangeWedge_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLagrangeWedge_create_drop() {
    let obj = vtkLagrangeWedge::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLagrangeWedge(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a 1D line
///
///
/// vtkLine is a concrete implementation of vtkCell to represent a 1D line.
#[allow(non_camel_case_types)]
pub struct vtkLine(*mut core::ffi::c_void);
impl vtkLine {
    /// Creates a new [vtkLine] wrapped inside `vtkNew`
    #[doc(alias = "vtkLine")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLine_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLine_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLine_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLine_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLine {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLine {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLine_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLine_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLine_create_drop() {
    let obj = vtkLine::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLine(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// compute interpolation computes
///
/// for closed triangular mesh
///
/// vtkMeanValueCoordinatesInterpolator computes interpolation weights for a
/// closed, manifold polyhedron mesh.  Once computed, the interpolation
/// weights can be used to interpolate data anywhere interior or exterior to
/// the mesh. This work implements two MVC algorithms. The first one is for
/// triangular meshes which is documented in the Siggraph 2005 paper by Tao Ju,
/// Scot Schaefer and Joe Warren from Rice University "Mean Value Coordinates
/// for Closed Triangular Meshes". The second one is for general polyhedron
/// mesh which is documented in the Eurographics Symposium on Geometry Processing
/// 2006 paper by Torsten Langer, Alexander Belyaev and Hans-Peter Seidel from
/// MPI Informatik "Spherical Barycentric Coordinates".
/// The filter will automatically choose which algorithm to use based on whether
/// the input mesh is triangulated or not.
///
/// In VTK this class was initially created to interpolate data across
/// polyhedral cells. In addition, the class can be used to interpolate
/// data values from a polyhedron mesh, and to smoothly deform a mesh from
/// an associated control mesh.
///
/// @sa
/// vtkPolyhedralCell
#[allow(non_camel_case_types)]
pub struct vtkMeanValueCoordinatesInterpolator(*mut core::ffi::c_void);
impl vtkMeanValueCoordinatesInterpolator {
    /// Creates a new [vtkMeanValueCoordinatesInterpolator] wrapped inside `vtkNew`
    #[doc(alias = "vtkMeanValueCoordinatesInterpolator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMeanValueCoordinatesInterpolator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMeanValueCoordinatesInterpolator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMeanValueCoordinatesInterpolator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMeanValueCoordinatesInterpolator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMeanValueCoordinatesInterpolator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMeanValueCoordinatesInterpolator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMeanValueCoordinatesInterpolator_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkMeanValueCoordinatesInterpolator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMeanValueCoordinatesInterpolator_create_drop() {
    let obj = vtkMeanValueCoordinatesInterpolator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMeanValueCoordinatesInterpolator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// merge exactly coincident points
///
///
/// vtkMergePoints is a locator object to quickly locate points in 3D.
/// The primary difference between vtkMergePoints and its superclass
/// vtkPointLocator is that vtkMergePoints merges precisely coincident points
/// and is therefore much faster.
/// @sa
/// vtkCleanPolyData
#[allow(non_camel_case_types)]
pub struct vtkMergePoints(*mut core::ffi::c_void);
impl vtkMergePoints {
    /// Creates a new [vtkMergePoints] wrapped inside `vtkNew`
    #[doc(alias = "vtkMergePoints")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMergePoints_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMergePoints_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMergePoints_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMergePoints_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMergePoints {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMergePoints {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMergePoints_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMergePoints_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMergePoints_create_drop() {
    let obj = vtkMergePoints::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMergePoints(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// class describing a molecule
///
///
///
///
/// vtkMolecule and the convenience classes vtkAtom and vtkBond
/// describe the geometry and connectivity of a molecule. The molecule
/// can be constructed using the AppendAtom() and AppendBond() methods in one
/// of two ways; either by fully specifying the atom/bond in a single
/// call, or by incrementally setting the various attributes using the
/// convenience vtkAtom and vtkBond classes:
///
/// Single call:
/// \code
/// vtkMolecule *mol = vtkMolecule::New();
/// vtkAtom h1 = mol->AppendAtom(1, 0.0, 0.0, -0.5);
/// vtkAtom h2 = mol->AppendAtom(1, 0.0, 0.0,  0.5);
/// vtkBond b  = mol->AppendBond(h1, h2, 1);
/// \endcode
///
/// Incremental:
/// \code
/// vtkMolecule *mol = vtkMolecule::New();
///
/// vtkAtom h1 = mol->AppendAtom();
/// h1.SetAtomicNumber(1);
/// h1.SetPosition(0.0, 0.0, -0.5);
///
/// vtkAtom h2 = mol->AppendAtom();
/// h2.SetAtomicNumber(1);
/// vtkVector3d displacement (0.0, 0.0, 1.0);
/// h2.SetPosition(h1.GetPositionAsVector3d() + displacement);
///
/// vtkBond b  = mol->AppendBond(h1, h2, 1);
/// \endcode
///
/// Both of the above methods will produce the same molecule, two
/// hydrogens connected with a 1.0 Angstrom single bond, aligned to the
/// z-axis. The second example also demonstrates the use of VTK's
/// vtkVector class, which is fully supported by the Chemistry kit.
///
/// The vtkMolecule object is intended to be used with the
/// vtkMoleculeMapper class for visualizing molecular structure using
/// common rendering techniques.
///
/// \warning While direct use of the underlying vtkUndirectedGraph
/// structure is possible due to vtkMolecule's public inheritance, this
/// should not be relied upon and may change in the future.
///
/// @sa
/// vtkAtom vtkBond vtkMoleculeMapper vtkPeriodicTable
#[allow(non_camel_case_types)]
pub struct vtkMolecule(*mut core::ffi::c_void);
impl vtkMolecule {
    /// Creates a new [vtkMolecule] wrapped inside `vtkNew`
    #[doc(alias = "vtkMolecule")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMolecule_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMolecule_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMolecule_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMolecule_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMolecule {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMolecule {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMolecule_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMolecule_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMolecule_create_drop() {
    let obj = vtkMolecule::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMolecule(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Composite dataset that organizes datasets into
///
/// blocks.
///
/// vtkMultiBlockDataSet is a vtkCompositeDataSet that stores
/// a hierarchy of datasets. The dataset collection consists of
/// multiple blocks. Each block can itself be a vtkMultiBlockDataSet, thus
/// providing for a full tree structure.
/// Sub-blocks are usually used to distribute blocks across processors.
/// For example, a 1 block dataset can be distributed as following:
/// @verbatim
/// proc 0:
/// Block 0:
/// * ds 0
/// * (null)
///
/// proc 1:
/// Block 0:
/// * (null)
/// * ds 1
/// @endverbatim
#[allow(non_camel_case_types)]
pub struct vtkMultiBlockDataSet(*mut core::ffi::c_void);
impl vtkMultiBlockDataSet {
    /// Creates a new [vtkMultiBlockDataSet] wrapped inside `vtkNew`
    #[doc(alias = "vtkMultiBlockDataSet")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMultiBlockDataSet_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMultiBlockDataSet_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMultiBlockDataSet_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMultiBlockDataSet_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMultiBlockDataSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMultiBlockDataSet {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMultiBlockDataSet_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMultiBlockDataSet_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMultiBlockDataSet_create_drop() {
    let obj = vtkMultiBlockDataSet::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMultiBlockDataSet(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// composite dataset to encapsulates pieces of
///
/// dataset.
///
/// A vtkMultiPieceDataSet dataset groups multiple data pieces together.
/// For example, say that a simulation broke a volume into 16 piece so that
/// each piece can be processed with 1 process in parallel. We want to load
/// this volume in a visualization cluster of 4 nodes. Each node will get 4
/// pieces, not necessarily forming a whole rectangular piece. In this case,
/// it is not possible to append the 4 pieces together into a vtkImageData.
/// In this case, these 4 pieces can be collected together using a
/// vtkMultiPieceDataSet.
/// Note that vtkMultiPieceDataSet is intended to be included in other composite
/// datasets eg. vtkMultiBlockDataSet, vtkHierarchicalBoxDataSet. Hence the lack
/// of algorithms producing vtkMultiPieceDataSet.
#[allow(non_camel_case_types)]
pub struct vtkMultiPieceDataSet(*mut core::ffi::c_void);
impl vtkMultiPieceDataSet {
    /// Creates a new [vtkMultiPieceDataSet] wrapped inside `vtkNew`
    #[doc(alias = "vtkMultiPieceDataSet")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMultiPieceDataSet_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMultiPieceDataSet_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMultiPieceDataSet_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMultiPieceDataSet_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMultiPieceDataSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMultiPieceDataSet {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMultiPieceDataSet_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMultiPieceDataSet_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMultiPieceDataSet_create_drop() {
    let obj = vtkMultiPieceDataSet::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMultiPieceDataSet(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// An editable directed graph.
///
///
///
/// vtkMutableDirectedGraph is a directed graph which has additional methods
/// for adding edges and vertices. AddChild() is a convenience method for
/// constructing trees. ShallowCopy(), DeepCopy(), CheckedShallowCopy() and
/// CheckedDeepCopy() will succeed for instances of vtkDirectedGraph,
/// vtkMutableDirectedGraph and vtkTree.
///
/// @sa
/// vtkDirectedGraph vtkGraph vtkTree
#[allow(non_camel_case_types)]
pub struct vtkMutableDirectedGraph(*mut core::ffi::c_void);
impl vtkMutableDirectedGraph {
    /// Creates a new [vtkMutableDirectedGraph] wrapped inside `vtkNew`
    #[doc(alias = "vtkMutableDirectedGraph")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMutableDirectedGraph_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMutableDirectedGraph_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMutableDirectedGraph_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMutableDirectedGraph_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMutableDirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMutableDirectedGraph {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMutableDirectedGraph_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMutableDirectedGraph_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMutableDirectedGraph_create_drop() {
    let obj = vtkMutableDirectedGraph::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMutableDirectedGraph(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// An editable undirected graph.
///
///
///
/// vtkMutableUndirectedGraph is an undirected graph with additional functions
/// for adding vertices and edges. ShallowCopy(), DeepCopy(), CheckedShallowCopy(),
/// and CheckedDeepCopy() will succeed when the argument is a vtkUndirectedGraph
/// or vtkMutableUndirectedGraph.
///
/// @sa
/// vtkUndirectedGraph vtkGraph
#[allow(non_camel_case_types)]
pub struct vtkMutableUndirectedGraph(*mut core::ffi::c_void);
impl vtkMutableUndirectedGraph {
    /// Creates a new [vtkMutableUndirectedGraph] wrapped inside `vtkNew`
    #[doc(alias = "vtkMutableUndirectedGraph")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMutableUndirectedGraph_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMutableUndirectedGraph_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMutableUndirectedGraph_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMutableUndirectedGraph_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMutableUndirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMutableUndirectedGraph {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMutableUndirectedGraph_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMutableUndirectedGraph_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMutableUndirectedGraph_create_drop() {
    let obj = vtkMutableUndirectedGraph::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMutableUndirectedGraph(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// direct / check-free point insertion.
///
///
///
/// As a special sub-class of vtkPointLocator, vtkNonMergingPointLocator is
/// intended for direct / check-free insertion of points into a vtkPoints
/// object. In other words, any given point is always directly inserted.
/// The name emphasizes the difference between this class and its sibling
/// class vtkMergePoints in that the latter class performs check-based zero
/// tolerance point insertion (or to 'merge' exactly duplicate / coincident
/// points) by exploiting the uniform bin mechanism employed by the parent
/// class vtkPointLocator. vtkPointLocator allows for generic (zero and non-
/// zero) tolerance point insertion as well as point location.
///
/// @sa
/// vtkIncrementalPointLocator vtkPointLocator vtkMergePoints
#[allow(non_camel_case_types)]
pub struct vtkNonMergingPointLocator(*mut core::ffi::c_void);
impl vtkNonMergingPointLocator {
    /// Creates a new [vtkNonMergingPointLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkNonMergingPointLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkNonMergingPointLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkNonMergingPointLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkNonMergingPointLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkNonMergingPointLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkNonMergingPointLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkNonMergingPointLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkNonMergingPointLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkNonMergingPointLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkNonMergingPointLocator_create_drop() {
    let obj = vtkNonMergingPointLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkNonMergingPointLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A concrete instance of vtkUniformGridAMR to store uniform grids at different
///
/// levels of resolution that do not overlap with each other.
///
/// @sa
/// vtkUniformGridAMR
#[allow(non_camel_case_types)]
pub struct vtkNonOverlappingAMR(*mut core::ffi::c_void);
impl vtkNonOverlappingAMR {
    /// Creates a new [vtkNonOverlappingAMR] wrapped inside `vtkNew`
    #[doc(alias = "vtkNonOverlappingAMR")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkNonOverlappingAMR_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkNonOverlappingAMR_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkNonOverlappingAMR_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkNonOverlappingAMR_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkNonOverlappingAMR {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkNonOverlappingAMR {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkNonOverlappingAMR_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkNonOverlappingAMR_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkNonOverlappingAMR_create_drop() {
    let obj = vtkNonOverlappingAMR::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkNonOverlappingAMR(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// an octree spatial decomposition of a set of points
///
///
///
/// Given a vtkDataSet, create an octree that is locally refined
/// such that all leaf octants contain less than a certain
/// amount of points.  Note that there is no size constraint that
/// a leaf octant in relation to any of its neighbors.
///
/// This class can also generate a PolyData representation of
/// the boundaries of the spatial regions in the decomposition.
///
/// @sa
/// vtkLocator vtkPointLocator vtkOctreePointLocatorNode
#[allow(non_camel_case_types)]
pub struct vtkOctreePointLocator(*mut core::ffi::c_void);
impl vtkOctreePointLocator {
    /// Creates a new [vtkOctreePointLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkOctreePointLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOctreePointLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOctreePointLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOctreePointLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOctreePointLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOctreePointLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOctreePointLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOctreePointLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOctreePointLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOctreePointLocator_create_drop() {
    let obj = vtkOctreePointLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOctreePointLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Octree node that has 8 children each of equal size
///
///
///
/// This class represents a single spatial region in a 3D axis octant
/// partitioning.  It is intended to work efficiently with the
/// vtkOctreePointLocator and is not meant for general use.  It is assumed
/// the region bounds some set of points.  The ordering of the children is
/// (-x,-y,-z),(+x,-y,-z),(-x,+y,-z),(+x,+y,-z),(-x,-y,+z),(+x,-y,+z),
/// (-x,+y,+z),(+x,+y,+z).  The portion of the domain assigned to an
/// octant is Min < x <= Max.
///
/// @sa
/// vtkOctreePointLocator
#[allow(non_camel_case_types)]
pub struct vtkOctreePointLocatorNode(*mut core::ffi::c_void);
impl vtkOctreePointLocatorNode {
    /// Creates a new [vtkOctreePointLocatorNode] wrapped inside `vtkNew`
    #[doc(alias = "vtkOctreePointLocatorNode")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOctreePointLocatorNode_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOctreePointLocatorNode_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOctreePointLocatorNode_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOctreePointLocatorNode_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOctreePointLocatorNode {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOctreePointLocatorNode {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOctreePointLocatorNode_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOctreePointLocatorNode_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOctreePointLocatorNode_create_drop() {
    let obj = vtkOctreePointLocatorNode::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOctreePointLocatorNode(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// helper class to generate triangulations
///
///
/// This class is used to generate unique triangulations of points. The
/// uniqueness of the triangulation is controlled by the id of the inserted
/// points in combination with a Delaunay criterion. The class is designed to
/// be as fast as possible (since the algorithm can be slow) and uses block
/// memory allocations to support rapid triangulation generation. Also, the
/// assumption behind the class is that a maximum of hundreds of points are to
/// be triangulated. If you desire more robust triangulation methods use
/// vtkPolygon::Triangulate(), vtkDelaunay2D, or vtkDelaunay3D.
///
/// @par Background:
/// This work is documented in the technical paper: W.J. Schroeder, B. Geveci,
/// M. Malaterre. Compatible Triangulations of Spatial Decompositions. In
/// Proceedings of Visualization 2004, IEEE Press October 2004.
///
/// @par Background:
/// Delaunay triangulations are unique assuming a random distribution of input
/// points. The 3D Delaunay criterion is as follows: the circumsphere of each
/// tetrahedron contains no other points of the triangulation except for the
/// four points defining the tetrahedron.  In application this property is
/// hard to satisfy because objects like cubes are defined by eight points all
/// sharing the same circumsphere (center and radius); hence the Delaunay
/// triangulation is not unique.  These so-called degenerate situations are
/// typically resolved by arbitrary selecting a triangulation. This code does
/// something different: it resolves degenerate triangulations by modifying
/// the "InCircumsphere" method to use a slightly smaller radius. Hence,
/// degenerate points are always considered "out" of the circumsphere. This,
/// in combination with an ordering (based on id) of the input points,
/// guarantees a unique triangulation.
///
/// @par Background:
/// There is another related characteristic of Delaunay triangulations. Given
/// a N-dimensional Delaunay triangulation, points laying on a (N-1) dimensional
/// plane also form a (N-1) Delaunay triangulation. This means for example,
/// that if a 3D cell is defined by a set of (2D) planar faces, then the
/// face triangulations are Delaunay. Combining this with the method to
/// generate unique triangulations described previously, the triangulations
/// on the face are guaranteed unique. This fact can be used to triangulate
/// 3D objects in such a way to guarantee compatible face triangulations.
/// This is a very useful fact for parallel processing, or performing
/// operations like clipping that require compatible triangulations across
/// 3D cell faces. (See vtkClipVolume for an example.)
///
/// @par Background:
/// A special feature of this class is that it can generate triangulation
/// templates on the fly. If template triangulation is enabled, then the
/// ordered triangulator will first triangulate the cell using the slower
/// ordered Delaunay approach, and then store the result as a template.
/// Later, if the same cell type and cell configuration is encountered,
/// then the template is reused which greatly speeds the triangulation.
///
/// @warning
/// Duplicate vertices will be ignored, i.e., if two points have the same
/// coordinates the second one is discarded. The implications are that the
/// user of this class must prevent duplicate points. Because the precision
/// of this algorithm is double, it's also a good idea to merge points
/// that are within some epsilon of one another.
///
/// @warning
/// The triangulation is performed using the parametric coordinates of the
/// inserted points. Therefore the bounds (see InitTriangulation()) should
/// represent the range of the parametric coordinates of the inserted points.
///
/// @sa
/// vtkDelaunay2D vtkDelaunay3D vtkPolygon
#[allow(non_camel_case_types)]
pub struct vtkOrderedTriangulator(*mut core::ffi::c_void);
impl vtkOrderedTriangulator {
    /// Creates a new [vtkOrderedTriangulator] wrapped inside `vtkNew`
    #[doc(alias = "vtkOrderedTriangulator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOrderedTriangulator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOrderedTriangulator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOrderedTriangulator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOrderedTriangulator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOrderedTriangulator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOrderedTriangulator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOrderedTriangulator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOrderedTriangulator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOrderedTriangulator_create_drop() {
    let obj = vtkOrderedTriangulator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOrderedTriangulator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Iterates through all outgoing edges from a vertex.
///
///
///
/// vtkOutEdgeIterator iterates through all edges whose source is a particular
/// vertex. Instantiate this class directly and call Initialize() to traverse
/// the vertex of a graph. Alternately, use GetInEdges() on the graph to
/// initialize the iterator. it->Next() returns a vtkOutEdgeType structure,
/// which contains Id, the edge's id, and Target, the edge's target vertex.
///
/// @sa
/// vtkGraph vtkInEdgeIterator
#[allow(non_camel_case_types)]
pub struct vtkOutEdgeIterator(*mut core::ffi::c_void);
impl vtkOutEdgeIterator {
    /// Creates a new [vtkOutEdgeIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkOutEdgeIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOutEdgeIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOutEdgeIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOutEdgeIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOutEdgeIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOutEdgeIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOutEdgeIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOutEdgeIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOutEdgeIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOutEdgeIterator_create_drop() {
    let obj = vtkOutEdgeIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOutEdgeIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// hierarchical dataset of vtkUniformGrids
///
///
///
/// vtkOverlappingAMR extends vtkUniformGridAMR by exposing access to the
/// amr meta data, which stores all structural information represented
/// by an vtkAMRInformation object
///
/// @sa
/// vtkAMRInformation
#[allow(non_camel_case_types)]
pub struct vtkOverlappingAMR(*mut core::ffi::c_void);
impl vtkOverlappingAMR {
    /// Creates a new [vtkOverlappingAMR] wrapped inside `vtkNew`
    #[doc(alias = "vtkOverlappingAMR")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOverlappingAMR_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOverlappingAMR_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOverlappingAMR_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOverlappingAMR_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOverlappingAMR {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOverlappingAMR {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOverlappingAMR_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOverlappingAMR_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOverlappingAMR_create_drop() {
    let obj = vtkOverlappingAMR::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOverlappingAMR(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// composite dataset to encapsulates a dataset consisting of
///
/// partitions.
///
/// A vtkPartitionedDataSet dataset groups multiple datasets together.
/// For example, say a simulation running in parallel on 16 processes
/// generated 16 datasets that when considering together form a whole
/// dataset. These are referred to as the partitions of the whole dataset.
/// Now imagine that we want to load a volume of 16 partitions in a
/// visualization cluster of 4 nodes. Each node could get 4 partitions,
/// not necessarily forming a whole rectangular region. In this case,
/// it is not possible to append the 4 partitions together into a vtkImageData.
/// We can then collect these 4 partitions together using a
/// vtkPartitionedDataSet.
///
/// It is required that all non-empty partitions have the same arrays
/// and that they can be processed together as a whole by the same kind of
/// filter. However, it is not required that they are of the same type.
/// For example, it is possible to have structured datasets together with
/// unstructured datasets as long as they are compatible meshes (i.e. can
/// be processed together for the same kind of filter).
#[allow(non_camel_case_types)]
pub struct vtkPartitionedDataSet(*mut core::ffi::c_void);
impl vtkPartitionedDataSet {
    /// Creates a new [vtkPartitionedDataSet] wrapped inside `vtkNew`
    #[doc(alias = "vtkPartitionedDataSet")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPartitionedDataSet_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPartitionedDataSet_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPartitionedDataSet_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPartitionedDataSet_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPartitionedDataSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPartitionedDataSet {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPartitionedDataSet_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPartitionedDataSet_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPartitionedDataSet_create_drop() {
    let obj = vtkPartitionedDataSet::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPartitionedDataSet(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Composite dataset that groups datasets as a collection.
///
///
/// vtkPartitionedDataSetCollection is a vtkCompositeDataSet that stores
/// a collection of non-null vtkPartitionedDataSets. These items can represent
/// different concepts depending on the context. For example, they can
/// represent region of different materials in a simulation or parts in
/// an assembly. It is not requires that items have anything in common.
/// For example, they can have completely different point or cell arrays.
#[allow(non_camel_case_types)]
pub struct vtkPartitionedDataSetCollection(*mut core::ffi::c_void);
impl vtkPartitionedDataSetCollection {
    /// Creates a new [vtkPartitionedDataSetCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkPartitionedDataSetCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPartitionedDataSetCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPartitionedDataSetCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPartitionedDataSetCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPartitionedDataSetCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPartitionedDataSetCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPartitionedDataSetCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPartitionedDataSetCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPartitionedDataSetCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPartitionedDataSetCollection_create_drop() {
    let obj = vtkPartitionedDataSetCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPartitionedDataSetCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// concrete dataset representing a path defined by Bezier
///
/// curves.
///
/// vtkPath provides a container for paths composed of line segments,
/// 2nd-order (quadratic) and 3rd-order (cubic) Bezier curves.
#[allow(non_camel_case_types)]
pub struct vtkPath(*mut core::ffi::c_void);
impl vtkPath {
    /// Creates a new [vtkPath] wrapped inside `vtkNew`
    #[doc(alias = "vtkPath")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPath_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPath_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPath_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPath_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPath {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPath {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPath_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPath_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPath_create_drop() {
    let obj = vtkPath::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPath(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a 3D cell that represents a convex prism with
///
/// pentagonal base
///
/// vtkPentagonalPrism is a concrete implementation of vtkCell to represent a
/// linear convex 3D prism with pentagonal base. Such prism is defined by the
/// ten points (0-9), where (0,1,2,3,4) is the base of the prism which, using
/// the right hand rule, forms a pentagon whose normal points is in the direction
/// of the opposite face (5,6,7,8,9).
///
/// @par Thanks:
/// Thanks to Philippe Guerville who developed this class.
/// Thanks to Charles Pignerol (CEA-DAM, France) who ported this class under
/// VTK 4. <br>
/// Thanks to Jean Favre (CSCS, Switzerland) who contributed to integrate this
/// class in VTK. <br>
/// Please address all comments to Jean Favre (jfavre at cscs.ch).
///
/// @par Thanks:
/// The Interpolation functions and derivatives were changed in June
/// 2015 by Bill Lorensen. These changes follow the formulation in:
/// http://dilbert.engr.ucdavis.edu/~suku/nem/papers/polyelas.pdf
/// NOTE: An additional copy of this paper is located at:
/// http://www.vtk.org/Wiki/File:ApplicationOfPolygonalFiniteElementsInLinearElasticity.pdf
#[allow(non_camel_case_types)]
pub struct vtkPentagonalPrism(*mut core::ffi::c_void);
impl vtkPentagonalPrism {
    /// Creates a new [vtkPentagonalPrism] wrapped inside `vtkNew`
    #[doc(alias = "vtkPentagonalPrism")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPentagonalPrism_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPentagonalPrism_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPentagonalPrism_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPentagonalPrism_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPentagonalPrism {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPentagonalPrism {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPentagonalPrism_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPentagonalPrism_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPentagonalPrism_create_drop() {
    let obj = vtkPentagonalPrism::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPentagonalPrism(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// an implicit function that implements Perlin noise
///
///
/// vtkPerlinNoise computes a Perlin noise field as an implicit function.
/// vtkPerlinNoise is a concrete implementation of vtkImplicitFunction.
/// Perlin noise, originally described by Ken Perlin, is a non-periodic and
/// continuous noise function useful for modeling real-world objects.
///
/// The amplitude and frequency of the noise pattern are adjustable.  This
/// implementation of Perlin noise is derived closely from Greg Ward's version
/// in Graphics Gems II.
///
/// @sa
/// vtkImplicitFunction
#[allow(non_camel_case_types)]
pub struct vtkPerlinNoise(*mut core::ffi::c_void);
impl vtkPerlinNoise {
    /// Creates a new [vtkPerlinNoise] wrapped inside `vtkNew`
    #[doc(alias = "vtkPerlinNoise")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPerlinNoise_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPerlinNoise_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPerlinNoise_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPerlinNoise_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPerlinNoise {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPerlinNoise {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPerlinNoise_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPerlinNoise_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPerlinNoise_create_drop() {
    let obj = vtkPerlinNoise::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPerlinNoise(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Defines a 1D piecewise function.
///
///
///
/// Defines a piecewise function mapping. This mapping allows the addition
/// of control points, and allows the user to control the function between
/// the control points. A piecewise hermite curve is used between control
/// points, based on the sharpness and midpoint parameters. A sharpness of
/// 0 yields a piecewise linear function and a sharpness of 1 yields a
/// piecewise constant function. The midpoint is the normalized distance
/// between control points at which the curve reaches the median Y value.
/// The midpoint and sharpness values specified when adding a node are used
/// to control the transition to the next node (the last node's values are
/// ignored) Outside the range of nodes, the values are 0 if Clamping is off,
/// or the nearest node point if Clamping is on. Using the legacy methods for
/// adding points  (which do not have Sharpness and Midpoint parameters)
/// will default to Midpoint = 0.5 (halfway between the control points) and
/// Sharpness = 0.0 (linear).
#[allow(non_camel_case_types)]
pub struct vtkPiecewiseFunction(*mut core::ffi::c_void);
impl vtkPiecewiseFunction {
    /// Creates a new [vtkPiecewiseFunction] wrapped inside `vtkNew`
    #[doc(alias = "vtkPiecewiseFunction")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPiecewiseFunction_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPiecewiseFunction_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPiecewiseFunction_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPiecewiseFunction_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPiecewiseFunction {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPiecewiseFunction {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPiecewiseFunction_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPiecewiseFunction_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPiecewiseFunction_create_drop() {
    let obj = vtkPiecewiseFunction::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPiecewiseFunction(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents an orthogonal quadrilateral
///
///
/// vtkPixel is a concrete implementation of vtkCell to represent a 2D
/// orthogonal quadrilateral. Unlike vtkQuad, the corners are at right angles,
/// and aligned along x-y-z coordinate axes leading to large increases in
/// computational efficiency.
#[allow(non_camel_case_types)]
pub struct vtkPixel(*mut core::ffi::c_void);
impl vtkPixel {
    /// Creates a new [vtkPixel] wrapped inside `vtkNew`
    #[doc(alias = "vtkPixel")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPixel_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPixel_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPixel_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPixel_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPixel {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPixel {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPixel_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPixel_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPixel_create_drop() {
    let obj = vtkPixel::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPixel(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// perform various plane computations
///
///
/// vtkPlane provides methods for various plane computations. These include
/// projecting points onto a plane, evaluating the plane equation, and
/// returning plane normal. vtkPlane is a concrete implementation of the
/// abstract class vtkImplicitFunction.
#[allow(non_camel_case_types)]
pub struct vtkPlane(*mut core::ffi::c_void);
impl vtkPlane {
    /// Creates a new [vtkPlane] wrapped inside `vtkNew`
    #[doc(alias = "vtkPlane")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPlane_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPlane_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPlane_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPlane_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPlane {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPlane {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPlane_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPlane_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPlane_create_drop() {
    let obj = vtkPlane::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPlane(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain a list of planes
///
///
/// vtkPlaneCollection is an object that creates and manipulates
/// lists of objects of type vtkPlane.
/// @sa
/// vtkCollection
#[allow(non_camel_case_types)]
pub struct vtkPlaneCollection(*mut core::ffi::c_void);
impl vtkPlaneCollection {
    /// Creates a new [vtkPlaneCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkPlaneCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPlaneCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPlaneCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPlaneCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPlaneCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPlaneCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPlaneCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPlaneCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPlaneCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPlaneCollection_create_drop() {
    let obj = vtkPlaneCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPlaneCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for convex set of planes
///
///
/// vtkPlanes computes the implicit function and function gradient for a set
/// of planes. The planes must define a convex space.
///
/// The function value is the intersection (i.e., maximum value) obtained by
/// evaluating the each of the supplied planes. Hence the value is the maximum
/// distance of a point to the convex region defined by the planes. The
/// function gradient is the plane normal at the function value.  Note that
/// the normals must point outside of the convex region. Thus, a negative
/// function value means that a point is inside the convex region.
///
/// There are several methods to define the set of planes. The most general is
/// to supply an instance of vtkPoints and an instance of vtkDataArray. (The
/// points define a point on the plane, and the normals corresponding plane
/// normals.) Two other specialized ways are to 1) supply six planes defining
/// the view frustrum of a camera, and 2) provide a bounding box.
///
/// @sa
/// vtkImplicitBoolean vtkSpheres vtkFrustrumSource vtkCamera
#[allow(non_camel_case_types)]
pub struct vtkPlanes(*mut core::ffi::c_void);
impl vtkPlanes {
    /// Creates a new [vtkPlanes] wrapped inside `vtkNew`
    #[doc(alias = "vtkPlanes")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPlanes_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPlanes_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPlanes_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPlanes_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPlanes {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPlanes {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPlanes_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPlanes_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPlanes_create_drop() {
    let obj = vtkPlanes::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPlanes(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A vtkPlanesIntersection object is a
///
/// vtkPlanes object that can compute whether the arbitrary convex region
/// bounded by it's planes intersects an axis-aligned box.
///
///
/// A subclass of vtkPlanes, this class determines whether it
/// intersects an axis aligned box.   This is motivated by the
/// need to intersect the axis aligned region of a spatial
/// decomposition of volume data with various other regions.
/// It uses the algorithm from Graphics Gems IV, page 81.
///
/// @par Caveat:
/// An instance of vtkPlanes can be redefined by changing the planes,
/// but this subclass then will not know if the region vertices are
/// up to date.  (Region vertices can be specified in SetRegionVertices
/// or computed by the subclass.)  So Delete and recreate if you want
/// to change the set of planes.
#[allow(non_camel_case_types)]
pub struct vtkPlanesIntersection(*mut core::ffi::c_void);
impl vtkPlanesIntersection {
    /// Creates a new [vtkPlanesIntersection] wrapped inside `vtkNew`
    #[doc(alias = "vtkPlanesIntersection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPlanesIntersection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPlanesIntersection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPlanesIntersection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPlanesIntersection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPlanesIntersection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPlanesIntersection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPlanesIntersection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPlanesIntersection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPlanesIntersection_create_drop() {
    let obj = vtkPlanesIntersection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPlanesIntersection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// represent and manipulate point attribute data
///
///
/// vtkPointData is a class that is used to represent and manipulate
/// point attribute data (e.g., scalars, vectors, normals, texture
/// coordinates, etc.) Most of the functionality is handled by
/// vtkDataSetAttributes.
///
/// By default, `GhostTypesToSkip` is set to `DUPLICATEPOINT | HIDDENPOINT`.
/// See `vtkDataSetAttributes` for the definition of those constants.
#[allow(non_camel_case_types)]
pub struct vtkPointData(*mut core::ffi::c_void);
impl vtkPointData {
    /// Creates a new [vtkPointData] wrapped inside `vtkNew`
    #[doc(alias = "vtkPointData")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointData_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPointData_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPointData_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPointData_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPointData {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointData {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointData_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointData_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointData_create_drop() {
    let obj = vtkPointData::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPointData(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// quickly locate points in 3-space
///
///
/// vtkPointLocator is a spatial search object to quickly locate points in 3D.
/// vtkPointLocator works by dividing a specified region of space into a regular
/// array of "rectangular" buckets, and then keeping a list of points that
/// lie in each bucket. Typical operation involves giving a position in 3D
/// and finding the closest point.
///
/// vtkPointLocator has two distinct methods of interaction. In the first
/// method, you supply it with a dataset, and it operates on the points in
/// the dataset. In the second method, you supply it with an array of points,
/// and the object operates on the array.
///
/// @warning
/// Many other types of spatial locators have been developed such as
/// octrees and kd-trees. These are often more efficient for the
/// operations described here.
///
/// @warning
/// Frequently vtkStaticPointLocator is used in lieu of vtkPointLocator.
/// They are very similar in terms of algorithmic approach, however
/// vtkStaticCellLocator is threaded and is typically much faster for
/// a large number of points (on the order of 3-5x faster). For small numbers
/// of points, vtkPointLocator is just as fast as vtkStaticPointLocator.
///
/// @sa
/// vtkCellPicker vtkPointPicker vtkStaticPointLocator
#[allow(non_camel_case_types)]
pub struct vtkPointLocator(*mut core::ffi::c_void);
impl vtkPointLocator {
    /// Creates a new [vtkPointLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkPointLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPointLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPointLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPointLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPointLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointLocator_create_drop() {
    let obj = vtkPointLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPointLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// concrete class for storing a set of points
///
///
/// vtkPointSet is an concrete class representing a set of points
/// that specifies the interface for
/// datasets that explicitly use "point" arrays to represent geometry.
/// For example, vtkPolyData, vtkUnstructuredGrid, and vtkStructuredGrid
/// require point arrays to specify point positions, while vtkImageData
/// represents point positions implicitly (and hence is not a subclass
/// of vtkImageData).
///
/// Note: The vtkPolyData and vtkUnstructuredGrid datasets (derived classes of
/// vtkPointSet) are often used in geometric computation (e.g.,
/// vtkDelaunay2D).  In most cases during filter execution the output geometry
/// and/or topology is created once and provided as output; however in a very
/// few cases the underlying geometry/topology may be created and then
/// incrementally modified. This has implications on the use of supporting
/// classes like locators and cell links topological structures which may be
/// required to support incremental editing operations. Consequently, there is
/// a flag, Editable, that controls whether the dataset can be incrementally
/// edited after it is initially created. By default, and for performance
/// reasons, vtkPointSet derived classes are created as non-editable.  The few
/// methods that require incremental editing capabilities are documented in
/// derived classes.
///
/// Another important feature of vtkPointSet classes is the use of an internal
/// locator to speed up certain operations like FindCell(). Depending on the
/// application and desired performance, different locators (either a cell or
/// point locator) of different locator types may be used, along with different
/// strategies for using the locators to perform various operations. See
/// the class vtkFindCellStrategy for more information
///
/// @sa
/// vtkPolyData vtkStructuredGrid vtkUnstructuredGrid vtkFindCellStrategy
#[allow(non_camel_case_types)]
pub struct vtkPointSet(*mut core::ffi::c_void);
impl vtkPointSet {
    /// Creates a new [vtkPointSet] wrapped inside `vtkNew`
    #[doc(alias = "vtkPointSet")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointSet_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPointSet_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPointSet_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPointSet_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPointSet {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointSet {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointSet_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointSet_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointSet_create_drop() {
    let obj = vtkPointSet::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPointSet(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Implementation of vtkCellIterator using
///
/// vtkPointSet API.
#[allow(non_camel_case_types)]
pub struct vtkPointSetCellIterator(*mut core::ffi::c_void);
impl vtkPointSetCellIterator {
    /// Creates a new [vtkPointSetCellIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkPointSetCellIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointSetCellIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPointSetCellIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPointSetCellIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPointSetCellIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPointSetCellIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointSetCellIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointSetCellIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointSetCellIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointSetCellIterator_create_drop() {
    let obj = vtkPointSetCellIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPointSetCellIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// the convex hull of the orthogonal
///
/// projection of the vtkPoints in the 3 coordinate directions
///
/// a subclass of vtkPoints, it maintains the counter clockwise
/// convex hull of the points (projected orthogonally in the
/// three coordinate directions) and has a method to
/// test for intersection of that hull with an axis aligned
/// rectangle.  This is used for intersection tests of 3D volumes.
#[allow(non_camel_case_types)]
pub struct vtkPointsProjectedHull(*mut core::ffi::c_void);
impl vtkPointsProjectedHull {
    /// Creates a new [vtkPointsProjectedHull] wrapped inside `vtkNew`
    #[doc(alias = "vtkPointsProjectedHull")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointsProjectedHull_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPointsProjectedHull_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPointsProjectedHull_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPointsProjectedHull_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPointsProjectedHull {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointsProjectedHull {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointsProjectedHull_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointsProjectedHull_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointsProjectedHull_create_drop() {
    let obj = vtkPointsProjectedHull::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPointsProjectedHull(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// concrete dataset represents vertices, lines, polygons, and triangle strips
///
///
/// vtkPolyData is a data object that is a concrete implementation of
/// vtkDataSet. vtkPolyData represents a geometric structure consisting of
/// vertices, lines, polygons, and/or triangle strips. Point and cell
/// attribute values (e.g., scalars, vectors, etc.) also are represented.
///
/// The actual cell types (vtkCellType.h) supported by vtkPolyData are:
/// vtkVertex, vtkPolyVertex, vtkLine, vtkPolyLine, vtkTriangle, vtkQuad,
/// vtkPolygon, and vtkTriangleStrip.
///
/// One important feature of vtkPolyData objects is that special traversal and
/// data manipulation methods are available to process data. These methods are
/// generally more efficient than vtkDataSet methods and should be used
/// whenever possible. For example, traversing the cells in a dataset we would
/// use GetCell(). To traverse cells with vtkPolyData we would retrieve the
/// cell array object representing polygons (for example using GetPolys()) and
/// then use vtkCellArray's InitTraversal() and GetNextCell() methods.
///
/// @warning
/// Because vtkPolyData is implemented with four separate instances of
/// vtkCellArray to represent 0D vertices, 1D lines, 2D polygons, and 2D
/// triangle strips, it is possible to create vtkPolyData instances that
/// consist of a mixture of cell types. Because of the design of the class,
/// there are certain limitations on how mixed cell types are inserted into
/// the vtkPolyData, and in turn the order in which they are processed and
/// rendered. To preserve the consistency of cell ids, and to ensure that
/// cells with cell data are rendered properly, users must insert mixed cells
/// in the order of vertices (vtkVertex and vtkPolyVertex), lines (vtkLine and
/// vtkPolyLine), polygons (vtkTriangle, vtkQuad, vtkPolygon), and triangle
/// strips (vtkTriangleStrip).
///
/// @warning
/// Some filters when processing vtkPolyData with mixed cell types may process
/// the cells in differing ways. Some will convert one type into another
/// (e.g., vtkTriangleStrip into vtkTriangles) or expect a certain type
/// (vtkDecimatePro expects triangles or triangle strips; vtkTubeFilter
/// expects lines). Read the documentation for each filter carefully to
/// understand how each part of vtkPolyData is processed.
///
/// @warning
/// Some of the methods specified here function properly only when the dataset
/// has been specified as "Editable". They are documented as such.
#[allow(non_camel_case_types)]
pub struct vtkPolyData(*mut core::ffi::c_void);
impl vtkPolyData {
    /// Creates a new [vtkPolyData] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolyData")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyData_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolyData_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolyData_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolyData_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolyData {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyData {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyData_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyData_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyData_create_drop() {
    let obj = vtkPolyData::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolyData(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain a list of polygonal data objects
///
///
/// vtkPolyDataCollection is an object that creates and manipulates ordered
/// lists of datasets of type vtkPolyData.
///
/// @sa
/// vtkDataSetCollection vtkCollection
#[allow(non_camel_case_types)]
pub struct vtkPolyDataCollection(*mut core::ffi::c_void);
impl vtkPolyDataCollection {
    /// Creates a new [vtkPolyDataCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolyDataCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyDataCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolyDataCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolyDataCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolyDataCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolyDataCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyDataCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyDataCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyDataCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyDataCollection_create_drop() {
    let obj = vtkPolyDataCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolyDataCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a set of 1D lines
///
///
/// vtkPolyLine is a concrete implementation of vtkCell to represent a set
/// of 1D lines.
#[allow(non_camel_case_types)]
pub struct vtkPolyLine(*mut core::ffi::c_void);
impl vtkPolyLine {
    /// Creates a new [vtkPolyLine] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolyLine")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyLine_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolyLine_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolyLine_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolyLine_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolyLine {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyLine {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyLine_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyLine_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyLine_create_drop() {
    let obj = vtkPolyLine::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolyLine(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Implicit function that is generated by extrusion of a polyline along the Z axis
///
///
/// vtkPolyPlane is, as the name suggests, an extrusion of a vtkPolyLine.
/// The extrusion direction is assumed to be the Z vector. It can be used in
/// combination with a vtkCutter to cut a dataset with a polyplane.
/// vtkPolyPlane is a concrete implementation of the abstract class
/// vtkImplicitFunction.
///
/// @todo
/// Generalize to extrusions along arbitrary directions.
#[allow(non_camel_case_types)]
pub struct vtkPolyPlane(*mut core::ffi::c_void);
impl vtkPolyPlane {
    /// Creates a new [vtkPolyPlane] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolyPlane")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyPlane_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolyPlane_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolyPlane_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolyPlane_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolyPlane {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyPlane {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyPlane_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyPlane_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyPlane_create_drop() {
    let obj = vtkPolyPlane::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolyPlane(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a set of 0D vertices
///
///
/// vtkPolyVertex is a concrete implementation of vtkCell to represent a
/// set of 3D vertices.
#[allow(non_camel_case_types)]
pub struct vtkPolyVertex(*mut core::ffi::c_void);
impl vtkPolyVertex {
    /// Creates a new [vtkPolyVertex] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolyVertex")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyVertex_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolyVertex_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolyVertex_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolyVertex_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolyVertex {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyVertex {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyVertex_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyVertex_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyVertex_create_drop() {
    let obj = vtkPolyVertex::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolyVertex(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents an n-sided polygon
///
///
/// vtkPolygon is a concrete implementation of vtkCell to represent a 2D
/// n-sided polygon. The polygons cannot have any internal holes, and cannot
/// self-intersect. Define the polygon with n-points ordered in the counter-
/// clockwise direction; do not repeat the last point.
#[allow(non_camel_case_types)]
pub struct vtkPolygon(*mut core::ffi::c_void);
impl vtkPolygon {
    /// Creates a new [vtkPolygon] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolygon")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolygon_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolygon_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolygon_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolygon_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolygon {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolygon {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolygon_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolygon_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolygon_create_drop() {
    let obj = vtkPolygon::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolygon(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A 3D cell defined by a set of polygonal faces
///
///
/// @section Instantiation Instantiation
///
/// vtkPolyhedron is a concrete implementation of vtkCell that represents a 3D cell
/// defined by a set of polygonal faces.
///
/// To instantiate a vtkPolyhedron, like any vtkCell, one needs to define the
/// following structures:
/// - A list of point coordinates
/// - A list of global point IDs
///
/// Note that the ordering of points coordinates or IDs is not important.
/// However, it MUST be consistent between the two lists.
///
/// Unlike other kinds of cells (e.g. vtkVoxel), the topology is not directly deduced from points
/// coordinates or point IDs ordering; it must be explicitly defined by providing a list of faces
/// (see the SetFaces() method). Each face is represented as a sequence of global point Ids.
///
/// Once point coordinates, point IDs and faces are defined, the Initialize() method should be called
/// in order to setup the internal structures and finalize the construction of the polyhedron.
///
/// Here is an example of vtkPolyhedron instantiation:
/// @code{.cpp}
///
/// //  9 +------+.11
/// //    |`.    | `.
/// //    |13`+--+---+ 15
/// //    |   |  |   |
/// //  8 +---+--+.10|
/// //     `. |    `.|
/// //     12`+------+ 14
/// //
/// // (Global IDs are arbitrarily chosen between 8 and 15)
///
/// // Insert point coordinates
/// polyhedron->GetPoints()->SetNumberOfPoints(8);
/// polyhedron->GetPoints()->SetPoint(0, 0., 0., 0.); // 8
/// polyhedron->GetPoints()->SetPoint(1, 0., 1., 0.); // 9
/// polyhedron->GetPoints()->SetPoint(2, 1., 0., 0.); // 10
/// polyhedron->GetPoints()->SetPoint(3, 1., 1., 0.); // 11
/// polyhedron->GetPoints()->SetPoint(4, 0., 0., 1.); // 12
/// polyhedron->GetPoints()->SetPoint(5, 0., 1., 1.); // 13
/// polyhedron->GetPoints()->SetPoint(6, 1., 0., 1.); // 14
/// polyhedron->GetPoints()->SetPoint(7, 1., 1., 1.); // 15
///
/// // Insert point IDs (global IDs)
/// // Note that the canonical ordering (0, 1, ..., 8) is used
/// // to correlate point Ids and coordinates
/// polyhedron->GetPointIds()->Allocate(8);
/// for (int i = 8; i < 16; ++i)
/// {
/// polyhedron->GetPointIds()->InsertNextId(i);
/// }
///
/// // Describe faces, indexed on global IDs
/// vtkIdType faces[31] = { 6, // Number of faces
/// 4, 9 , 11, 10, 8 , // Number of points in the face + point IDs
/// 4, 11, 15, 14, 10, // Faces are described counter-clockwise
/// 4, 15, 13, 12, 14, // looking from the "outside" of the cell
/// 4, 13, 9 , 8 , 12,
/// 4, 14, 12, 8 , 10,
/// 4, 13, 15, 11, 9 };
///
/// polyhedron->SetFaces(faces);
///
/// // Initialize the polyhedron
/// // This will build internal structures and should be done before the proper
/// // use of the cell.
/// polyhedron->Initialize();
/// @endcode
///
/// @section Specifications Specifications
///
/// Polyhedrons described by this class must conform to some criteria in order to avoid errors and
/// guarantee good results in terms of visualization and processing.
///
/// These specifications are described as follows. Polyhedrons must:
/// - be watertight : the faces describing the polyhedron should define an enclosed volume
/// i.e. define the “inside” and the “outside” of the cell
/// - have planar faces : all points defining a face should be in the same 2D plane
/// - not be self-intersecting : for example, a face of the polyhedron can’t intersect other ones
/// - not contain zero-thickness portions : adjacent faces should not overlap each other even
/// partially
/// - not contain disconnected elements : detached vertice(s), edge(s) or face(s)
/// - be simply connected : vtkPolyhedron must describe a single polyhedron
/// - not contain duplicate elements : each point index and each face description should be unique
/// - not contain “internal” or “external” faces : for each face, one side should be “inside” the
/// cell, the other side “outside”
///
/// In a more global perspective, polyhedrons must be watertight and manifold.
/// In particular, each edge of the polyhedron must be adjacent to exactly two faces.
/// Several algorithms like contour, clip or slice will assume that each edge of the polyhedron
/// is adjacent to exactly two faces and will definitely lead to bad results (and generate numerous
/// warnings) if this criterion is not fulfilled.
///
/// @section Limitations Limitations
///
/// The class does not require the polyhedron to be convex. However, the support of concave
/// polyhedrons is currently limited. Concavity can lead to bad results with some filters,
/// including:
/// - Contour: the contour (surface) can be constructed outside of the cell,
/// - Triangulate: the current tetrahedralization algorithm can modify the initial
/// shape of the polygon (created tetrahedrons can change concave portions of the shape
/// to convex ones).
///
/// @section OtherDetails Other details
///
/// Interpolation functions and weights are defined / computed using the method of Mean Value
/// Coordinates (MVC). See the VTK class vtkMeanValueCoordinatesInterpolator for more information.
///
/// @sa
/// vtkCell3D vtkConvexPointSet vtkMeanValueCoordinatesInterpolator vtkPolyhedronUtilities
#[allow(non_camel_case_types)]
pub struct vtkPolyhedron(*mut core::ffi::c_void);
impl vtkPolyhedron {
    /// Creates a new [vtkPolyhedron] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolyhedron")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyhedron_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolyhedron_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolyhedron_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolyhedron_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolyhedron {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyhedron {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyhedron_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyhedron_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyhedron_create_drop() {
    let obj = vtkPolyhedron::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolyhedron(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a 3D cell that represents a linear pyramid
///
///
/// vtkPyramid is a concrete implementation of vtkCell to represent a 3D
/// pyramid. A pyramid consists of a rectangular base with four triangular
/// faces. vtkPyramid uses the standard isoparametric shape functions for
/// a linear pyramid. The pyramid is defined by the five points (0-4) where
/// (0,1,2,3) is the base of the pyramid which, using the right hand rule,
/// forms a quadrilaterial whose normal points in the direction of the
/// pyramid apex at vertex #4. The parametric location of vertex #4 is [0, 0, 1].
///
/// @sa
/// vtkConvexPointSet vtkHexahedron vtkTetra vtkVoxel vtkWedge
#[allow(non_camel_case_types)]
pub struct vtkPyramid(*mut core::ffi::c_void);
impl vtkPyramid {
    /// Creates a new [vtkPyramid] wrapped inside `vtkNew`
    #[doc(alias = "vtkPyramid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPyramid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPyramid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPyramid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPyramid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPyramid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPyramid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPyramid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPyramid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPyramid_create_drop() {
    let obj = vtkPyramid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPyramid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents a 2D quadrilateral
///
///
/// vtkQuad is a concrete implementation of vtkCell to represent a 2D
/// quadrilateral. vtkQuad is defined by the four points (0,1,2,3) in
/// counterclockwise order. vtkQuad uses the standard isoparametric
/// interpolation functions for a linear quadrilateral.
#[allow(non_camel_case_types)]
pub struct vtkQuad(*mut core::ffi::c_void);
impl vtkQuad {
    /// Creates a new [vtkQuad] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuad")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuad_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuad_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuad_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuad_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuad {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuad {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuad_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuad_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuad_create_drop() {
    let obj = vtkQuad::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuad(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, isoparametric edge
///
///
/// vtkQuadraticEdge is a concrete implementation of vtkNonLinearCell to
/// represent a one-dimensional, 3-nodes, isoparametric parabolic line. The
/// interpolation is the standard finite element, quadratic isoparametric
/// shape function. The cell includes a mid-edge node. The ordering of the
/// three points defining the cell is point ids (0,1,2) where id #2 is the
/// midedge node.
///
/// @sa
/// vtkQuadraticTriangle vtkQuadraticTetra vtkQuadraticWedge
/// vtkQuadraticQuad vtkQuadraticHexahedron vtkQuadraticPyramid
#[allow(non_camel_case_types)]
pub struct vtkQuadraticEdge(*mut core::ffi::c_void);
impl vtkQuadraticEdge {
    /// Creates a new [vtkQuadraticEdge] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticEdge")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticEdge_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticEdge_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticEdge_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticEdge_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticEdge {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticEdge {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticEdge_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticEdge_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticEdge_create_drop() {
    let obj = vtkQuadraticEdge::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticEdge(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 20-node isoparametric hexahedron
///
///
/// vtkQuadraticHexahedron is a concrete implementation of vtkNonLinearCell to
/// represent a three-dimensional, 20-node isoparametric parabolic
/// hexahedron. The interpolation is the standard finite element, quadratic
/// isoparametric shape function. The cell includes a mid-edge node. The
/// ordering of the twenty points defining the cell is point ids (0-7,8-19)
/// where point ids 0-7 are the eight corner vertices of the cube; followed by
/// twelve midedge nodes (8-19). Note that these midedge nodes correspond lie
/// on the edges defined by (0,1), (1,2), (2,3), (3,0), (4,5), (5,6), (6,7),
/// (7,4), (0,4), (1,5), (2,6), (3,7).
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticQuad vtkQuadraticPyramid vtkQuadraticWedge
#[allow(non_camel_case_types)]
pub struct vtkQuadraticHexahedron(*mut core::ffi::c_void);
impl vtkQuadraticHexahedron {
    /// Creates a new [vtkQuadraticHexahedron] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticHexahedron")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticHexahedron_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticHexahedron_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticHexahedron_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticHexahedron_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticHexahedron {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticHexahedron {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticHexahedron_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticHexahedron_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticHexahedron_create_drop() {
    let obj = vtkQuadraticHexahedron::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticHexahedron(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a quadratic-linear, 6-node isoparametric quad
///
///
/// vtkQuadraticQuad is a concrete implementation of vtkNonLinearCell to
/// represent a two-dimensional, 6-node isoparametric quadratic-linear quadrilateral
/// element. The interpolation is the standard finite element, quadratic-linear
/// isoparametric shape function. The cell includes a mid-edge node for two
/// of the four edges. The ordering of the six points defining
/// the cell are point ids (0-3,4-5) where ids 0-3 define the four corner
/// vertices of the quad; ids 4-7 define the midedge nodes (0,1) and (2,3) .
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra vtkQuadraticQuad
/// vtkQuadraticHexahedron vtkQuadraticWedge vtkQuadraticPyramid
///
/// @par Thanks:
/// Thanks to Soeren Gebbert  who developed this class and
/// integrated it into VTK 5.0.
#[allow(non_camel_case_types)]
pub struct vtkQuadraticLinearQuad(*mut core::ffi::c_void);
impl vtkQuadraticLinearQuad {
    /// Creates a new [vtkQuadraticLinearQuad] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticLinearQuad")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticLinearQuad_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticLinearQuad_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticLinearQuad_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticLinearQuad_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticLinearQuad {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticLinearQuad {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticLinearQuad_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticLinearQuad_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticLinearQuad_create_drop() {
    let obj = vtkQuadraticLinearQuad::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticLinearQuad(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a, 12-node isoparametric wedge
///
///
/// vtkQuadraticLinearWedge is a concrete implementation of vtkNonLinearCell to
/// represent a three-dimensional, 12-node isoparametric linear quadratic
/// wedge. The interpolation is the standard finite element, quadratic
/// isoparametric shape function in xy - layer and the linear functions in z - direction.
/// The cell includes mid-edge node in the triangle edges. The
/// ordering of the 12 points defining the cell is point ids (0-5,6-12)
/// where point ids 0-5 are the six corner vertices of the wedge; followed by
/// six midedge nodes (6-12). Note that these midedge nodes correspond lie
/// on the edges defined by (0,1), (1,2), (2,0), (3,4), (4,5), (5,3).
/// The Edges (0,3), (1,4), (2,5) don't have midedge nodes.
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticHexahedron vtkQuadraticQuad vtkQuadraticPyramid
///
/// @par Thanks:
/// Thanks to Soeren Gebbert who developed this class and
/// integrated it into VTK 5.0.
#[allow(non_camel_case_types)]
pub struct vtkQuadraticLinearWedge(*mut core::ffi::c_void);
impl vtkQuadraticLinearWedge {
    /// Creates a new [vtkQuadraticLinearWedge] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticLinearWedge")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticLinearWedge_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticLinearWedge_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticLinearWedge_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticLinearWedge_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticLinearWedge {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticLinearWedge {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticLinearWedge_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticLinearWedge_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticLinearWedge_create_drop() {
    let obj = vtkQuadraticLinearWedge::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticLinearWedge(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents a parabolic n-sided polygon
///
///
/// vtkQuadraticPolygon is a concrete implementation of vtkNonLinearCell to
/// represent a 2D n-sided (2*n nodes) parabolic polygon. The polygon cannot
/// have any internal holes, and cannot self-intersect. The cell includes a
/// mid-edge node for each of the n edges of the cell. The ordering of the
/// 2*n points defining the cell are point ids (0..n-1 and n..2*n-1) where ids
/// 0..n-1 define the corner vertices of the polygon; ids n..2*n-1 define the
/// midedge nodes. Define the polygon with points ordered in the counter-
/// clockwise direction; do not repeat the last point.
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticHexahedron vtkQuadraticWedge vtkQuadraticPyramid
#[allow(non_camel_case_types)]
pub struct vtkQuadraticPolygon(*mut core::ffi::c_void);
impl vtkQuadraticPolygon {
    /// Creates a new [vtkQuadraticPolygon] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticPolygon")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticPolygon_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticPolygon_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticPolygon_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticPolygon_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticPolygon {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticPolygon {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticPolygon_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticPolygon_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticPolygon_create_drop() {
    let obj = vtkQuadraticPolygon::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticPolygon(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 13-node isoparametric pyramid
///
///
/// vtkQuadraticPyramid is a concrete implementation of vtkNonLinearCell to
/// represent a three-dimensional, 13-node isoparametric parabolic
/// pyramid. The interpolation is the standard finite element, quadratic
/// isoparametric shape function. The cell includes a mid-edge node. The
/// ordering of the thirteen points defining the cell is point ids (0-4,5-12)
/// where point ids 0-4 are the five corner vertices of the pyramid; followed
/// by eight midedge nodes (5-12). Note that these midedge nodes lie
/// on the edges defined by (0,1), (1,2), (2,3), (3,0), (0,4), (1,4), (2,4),
/// (3,4), respectively. The parametric location of vertex #4 is [0, 0, 1].
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticHexahedron vtkQuadraticQuad vtkQuadraticWedge
///
/// @par Thanks:
/// The shape functions and derivatives could be implemented thanks to
/// the report Pyramid Solid Elements Linear and Quadratic Iso-P Models
/// From Center For Aerospace Structures
#[allow(non_camel_case_types)]
pub struct vtkQuadraticPyramid(*mut core::ffi::c_void);
impl vtkQuadraticPyramid {
    /// Creates a new [vtkQuadraticPyramid] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticPyramid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticPyramid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticPyramid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticPyramid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticPyramid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticPyramid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticPyramid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticPyramid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticPyramid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticPyramid_create_drop() {
    let obj = vtkQuadraticPyramid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticPyramid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 8-node isoparametric quad
///
///
/// vtkQuadraticQuad is a concrete implementation of vtkNonLinearCell to
/// represent a two-dimensional, 8-node isoparametric parabolic quadrilateral
/// element. The interpolation is the standard finite element, quadratic
/// isoparametric shape function. The cell includes a mid-edge node for each
/// of the four edges of the cell. The ordering of the eight points defining
/// the cell are point ids (0-3,4-7) where ids 0-3 define the four corner
/// vertices of the quad; ids 4-7 define the midedge nodes (0,1), (1,2),
/// (2,3), (3,0).
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticHexahedron vtkQuadraticWedge vtkQuadraticPyramid
#[allow(non_camel_case_types)]
pub struct vtkQuadraticQuad(*mut core::ffi::c_void);
impl vtkQuadraticQuad {
    /// Creates a new [vtkQuadraticQuad] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticQuad")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticQuad_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticQuad_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticQuad_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticQuad_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticQuad {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticQuad {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticQuad_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticQuad_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticQuad_create_drop() {
    let obj = vtkQuadraticQuad::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticQuad(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 10-node isoparametric tetrahedron
///
///
/// vtkQuadraticTetra is a concrete implementation of vtkNonLinearCell to
/// represent a three-dimensional, 10-node, isoparametric parabolic
/// tetrahedron. The interpolation is the standard finite element, quadratic
/// isoparametric shape function. The cell includes a mid-edge node on each of
/// the size edges of the tetrahedron. The ordering of the ten points defining
/// the cell is point ids (0-3,4-9) where ids 0-3 are the four tetra
/// vertices; and point ids 4-9 are the midedge nodes between (0,1), (1,2),
/// (2,0), (0,3), (1,3), and (2,3).
///
/// Note that this class uses an internal linear tessellation for some internal operations
/// (e.g., clipping and contouring). This means that some artifacts may appear trying to
/// represent a non-linear interpolation function with linear tets.
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticWedge
/// vtkQuadraticQuad vtkQuadraticHexahedron vtkQuadraticPyramid
#[allow(non_camel_case_types)]
pub struct vtkQuadraticTetra(*mut core::ffi::c_void);
impl vtkQuadraticTetra {
    /// Creates a new [vtkQuadraticTetra] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticTetra")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticTetra_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticTetra_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticTetra_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticTetra_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticTetra {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticTetra {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticTetra_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticTetra_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticTetra_create_drop() {
    let obj = vtkQuadraticTetra::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticTetra(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, isoparametric triangle
///
///
/// vtkQuadraticTriangle is a concrete implementation of vtkNonLinearCell to
/// represent a two-dimensional, 6-node, isoparametric parabolic triangle. The
/// interpolation is the standard finite element, quadratic isoparametric
/// shape function. The cell includes three mid-edge nodes besides the three
/// triangle vertices. The ordering of the three points defining the cell is
/// point ids (0-2,3-5) where id #3 is the midedge node between points
/// (0,1); id #4 is the midedge node between points (1,2); and id #5 is the
/// midedge node between points (2,0).
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTetra vtkQuadraticPyramid
/// vtkQuadraticQuad vtkQuadraticHexahedron vtkQuadraticWedge
#[allow(non_camel_case_types)]
pub struct vtkQuadraticTriangle(*mut core::ffi::c_void);
impl vtkQuadraticTriangle {
    /// Creates a new [vtkQuadraticTriangle] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticTriangle")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticTriangle_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticTriangle_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticTriangle_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticTriangle_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticTriangle {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticTriangle {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticTriangle_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticTriangle_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticTriangle_create_drop() {
    let obj = vtkQuadraticTriangle::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticTriangle(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 15-node isoparametric wedge
///
///
/// vtkQuadraticWedge is a concrete implementation of vtkNonLinearCell to
/// represent a three-dimensional, 15-node isoparametric parabolic
/// wedge. The interpolation is the standard finite element, quadratic
/// isoparametric shape function. The cell includes a mid-edge node. The
/// ordering of the fifteen points defining the cell is point ids (0-5,6-14)
/// where point ids 0-5 are the six corner vertices of the wedge, defined
/// analogously to the six points in vtkWedge (points (0,1,2) form the base of
/// the wedge which, using the right hand rule, forms a triangle whose normal
/// points away from the triangular face (3,4,5)); followed by nine midedge
/// nodes (6-14). Note that these midedge nodes correspond lie on the edges
/// defined by (0,1), (1,2), (2,0), (3,4), (4,5), (5,3), (0,3), (1,4), (2,5).
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticHexahedron vtkQuadraticQuad vtkQuadraticPyramid
#[allow(non_camel_case_types)]
pub struct vtkQuadraticWedge(*mut core::ffi::c_void);
impl vtkQuadraticWedge {
    /// Creates a new [vtkQuadraticWedge] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadraticWedge")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadraticWedge_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadraticWedge_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadraticWedge_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadraticWedge_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadraticWedge {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadraticWedge {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadraticWedge_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadraticWedge_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadraticWedge_create_drop() {
    let obj = vtkQuadraticWedge::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadraticWedge(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
/// An Elemental data type that holds a definition of a
/// numerical quadrature scheme. The definition contains
/// the requisite information to interpolate to the so called
/// quadrature points of the specific scheme. namely:
///
/// <pre>
/// 1)
/// A matrix of shape function weights(shape functions evaluated
/// at parametric coordinates of the quadrature points).
///
/// 2)
/// The number of quadrature points and cell nodes. These parameters
/// size the matrix, and allow for convenient evaluation by users
/// of the definition.
/// </pre>
#[allow(non_camel_case_types)]
pub struct vtkQuadratureSchemeDefinition(*mut core::ffi::c_void);
impl vtkQuadratureSchemeDefinition {
    /// Creates a new [vtkQuadratureSchemeDefinition] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadratureSchemeDefinition")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadratureSchemeDefinition_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadratureSchemeDefinition_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadratureSchemeDefinition_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadratureSchemeDefinition_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadratureSchemeDefinition {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadratureSchemeDefinition {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadratureSchemeDefinition_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadratureSchemeDefinition_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadratureSchemeDefinition_create_drop() {
    let obj = vtkQuadratureSchemeDefinition::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadratureSchemeDefinition(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// evaluate implicit quadric function
///
///
/// vtkQuadric evaluates the quadric function F(x,y,z) = a0*x^2 + a1*y^2 +
/// a2*z^2 + a3*x*y + a4*y*z + a5*x*z + a6*x + a7*y + a8*z + a9. vtkQuadric is
/// a concrete implementation of vtkImplicitFunction.
#[allow(non_camel_case_types)]
pub struct vtkQuadric(*mut core::ffi::c_void);
impl vtkQuadric {
    /// Creates a new [vtkQuadric] wrapped inside `vtkNew`
    #[doc(alias = "vtkQuadric")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuadric_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkQuadric_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkQuadric_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkQuadric_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkQuadric {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuadric {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuadric_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuadric_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuadric_create_drop() {
    let obj = vtkQuadric::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkQuadric(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a dataset that is topologically regular with variable spacing in the three coordinate
///
/// directions
///
/// vtkRectilinearGrid is a data object that is a concrete implementation of
/// vtkDataSet. vtkRectilinearGrid represents a geometric structure that is
/// topologically regular with variable spacing in the three coordinate
/// directions x-y-z.
///
/// To define a vtkRectilinearGrid, you must specify the dimensions of the
/// data and provide three arrays of values specifying the coordinates
/// along the x-y-z axes. The coordinate arrays are specified using three
/// vtkDataArray objects (one for x, one for y, one for z).
///
/// @warning
/// Make sure that the dimensions of the grid match the number of coordinates
/// in the x-y-z directions. If not, unpredictable results (including
/// program failure) may result. Also, you must supply coordinates in all
/// three directions, even if the dataset topology is 2D, 1D, or 0D.
#[allow(non_camel_case_types)]
pub struct vtkRectilinearGrid(*mut core::ffi::c_void);
impl vtkRectilinearGrid {
    /// Creates a new [vtkRectilinearGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkRectilinearGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRectilinearGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkRectilinearGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkRectilinearGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkRectilinearGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkRectilinearGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRectilinearGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRectilinearGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRectilinearGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRectilinearGrid_create_drop() {
    let obj = vtkRectilinearGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkRectilinearGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Reeb graph computation for PL scalar fields.
///
///
///
/// vtkReebGraph is a class that computes a Reeb graph given a PL scalar
/// field (vtkDataArray) defined on a simplicial mesh.
/// A Reeb graph is a concise representation of the connectivity evolution of
/// the level sets of a scalar function.
///
/// It is particularly useful in visualization (optimal seed set computation,
/// fast flexible isosurface extraction, automated transfer function design,
/// feature-driven visualization, etc.) and computer graphics (shape
/// deformation, shape matching, shape compression, etc.).
///
/// Reference:
/// "Sur les points singuliers d'une forme de Pfaff completement integrable ou
/// d'une fonction numerique",
/// G. Reeb,
/// Comptes-rendus de l'Academie des Sciences, 222:847-849, 1946.
///
/// vtkReebGraph implements one of the latest and most robust Reeb graph
/// computation algorithms.
///
/// Reference:
/// "Robust on-line computation of Reeb graphs: simplicity and speed",
/// V. Pascucci, G. Scorzelli, P.-T. Bremer, and A. Mascarenhas,
/// ACM Transactions on Graphics, Proc. of SIGGRAPH 2007.
///
/// vtkReebGraph provides methods for computing multi-resolution topological
/// hierarchies through topological simplification.
/// Topoligical simplification can be either driven by persistence homology
/// concepts (default behavior) or by application specific metrics (see
/// vtkReebGraphSimplificationMetric).
/// In the latter case, designing customized simplification metric evaluation
/// algorithms enables the user to control the definition of what should be
/// considered as noise or signal in the topological filtering process.
///
/// References:
/// "Topological persistence and simplification",
/// H. Edelsbrunner, D. Letscher, and A. Zomorodian,
/// Discrete Computational Geometry, 28:511-533, 2002.
///
/// "Extreme elevation on a 2-manifold",
/// P.K. Agarwal, H. Edelsbrunner, J. Harer, and Y. Wang,
/// ACM Symposium on Computational Geometry, pp. 357-365, 2004.
///
/// "Simplifying flexible isosurfaces using local geometric measures",
/// H. Carr, J. Snoeyink, M van de Panne,
/// IEEE Visualization, 497-504, 2004
///
/// "Loop surgery for volumetric meshes: Reeb graphs reduced to contour trees",
/// J. Tierny, A. Gyulassy, E. Simon, V. Pascucci,
/// IEEE Trans. on Vis. and Comp. Graph. (Proc of IEEE VIS), 15:1177-1184, 2009.
///
///
///
/// Reeb graphs can be computed from 2D data (vtkPolyData, with triangles only)
/// or 3D data (vtkUnstructuredGrid, with tetrahedra only), sequentially (see
/// the "Build" calls) or in streaming (see the "StreamTriangle" and
/// "StreamTetrahedron" calls).
///
/// vtkReebGraph inherits from vtkMutableDirectedGraph.
///
/// Each vertex of a vtkReebGraph object represents a critical point of the
/// scalar field where the connectivity of the related level set changes
/// (creation, deletion, split or merge of connected components).
/// A vtkIdTypeArray (called "Vertex Ids") is associated with the VertexData of
/// a vtkReebGraph object, in order to retrieve if necessary the exact Ids of
/// the corresponding vertices in the input mesh.
///
/// The edges of a vtkReebGraph object represent the regions of the input mesh
/// separated by the critical contours of the field, and where the connectivity
/// of the input field does not change.
/// A vtkVariantArray is associated with the EdgeDta of a vtkReebGraph object and
/// each entry of this array is a vtkAbstractArray containing the Ids of the
/// vertices of those regions, sorted by function value (useful for flexible
/// isosurface extraction or level set signature computation, for instance).
///
/// See Graphics/Testing/Cxx/TestReebGraph.cxx for examples of traversals and
/// typical usages (customized simplification, skeletonization, contour spectra,
/// etc.) of a vtkReebGraph object.
///
///
/// @sa
/// vtkReebGraphSimplificationMetric
/// vtkPolyDataToReebGraphFilter
/// vtkUnstructuredGridToReebGraphFilter
/// vtkReebGraphSimplificationFilter
/// vtkReebGraphSurfaceSkeletonFilter
/// vtkReebGraphVolumeSkeletonFilter
/// vtkAreaContourSpectrumFilter
/// vtkVolumeContourSpectrumFilter
///
/// @par Tests:
/// Graphics/Testing/Cxx/TestReebGraph.cxx
#[allow(non_camel_case_types)]
pub struct vtkReebGraph(*mut core::ffi::c_void);
impl vtkReebGraph {
    /// Creates a new [vtkReebGraph] wrapped inside `vtkNew`
    #[doc(alias = "vtkReebGraph")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkReebGraph_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkReebGraph_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkReebGraph_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkReebGraph_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkReebGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkReebGraph {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkReebGraph_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkReebGraph_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkReebGraph_create_drop() {
    let obj = vtkReebGraph::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkReebGraph(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// abstract class for custom Reeb graph
///
/// simplification metric design.
///
/// This class makes it possible to design customized simplification metric
/// evaluation algorithms, enabling the user to control the definition of what
/// should be considered as noise or signal in the topological filtering process.
///
/// References:
/// "Topological persistence and simplification",
/// H. Edelsbrunner, D. Letscher, and A. Zomorodian,
/// Discrete Computational Geometry, 28:511-533, 2002.
///
/// "Extreme elevation on a 2-manifold",
/// P.K. Agarwal, H. Edelsbrunner, J. Harer, and Y. Wang,
/// ACM Symposium on Computational Geometry, pp. 357-365, 2004.
///
/// "Simplifying flexible isosurfaces using local geometric measures",
/// H. Carr, J. Snoeyink, M van de Panne,
/// IEEE Visualization, 497-504, 2004
///
/// "Loop surgery for volumetric meshes: Reeb graphs reduced to contour trees",
/// J. Tierny, A. Gyulassy, E. Simon, V. Pascucci,
/// IEEE Trans. on Vis. and Comp. Graph. (Proc of IEEE VIS), 15:1177-1184, 2009.
///
///
/// See Graphics/Testing/Cxx/TestReebGraph.cxx for an example of concrete
/// implementation.
#[allow(non_camel_case_types)]
pub struct vtkReebGraphSimplificationMetric(*mut core::ffi::c_void);
impl vtkReebGraphSimplificationMetric {
    /// Creates a new [vtkReebGraphSimplificationMetric] wrapped inside `vtkNew`
    #[doc(alias = "vtkReebGraphSimplificationMetric")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkReebGraphSimplificationMetric_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkReebGraphSimplificationMetric_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkReebGraphSimplificationMetric_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkReebGraphSimplificationMetric_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkReebGraphSimplificationMetric {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkReebGraphSimplificationMetric {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkReebGraphSimplificationMetric_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkReebGraphSimplificationMetric_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkReebGraphSimplificationMetric_create_drop() {
    let obj = vtkReebGraphSimplificationMetric::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkReebGraphSimplificationMetric(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// data object that represents a "selection" in VTK.
///
///
/// vtkSelection defines a selection. A selection is a data-object that defines
/// which entities from another data-object are to treated as "selected". Filters like
/// `vtkExtractSelection` or `vtkExtractDataArraysOverTime` can then be used to
/// extract these selected entities from the *other* data-object.
///
/// vtkSelection comprises of `vtkSelectionNode`s and optionally, an expression
/// specified using `vtkSelection::SetExpression`. If non-empty, the expression
/// is a boolean expression that defines now the selection nodes present in the
/// selection are to be combined together to form the selection. If no expression
/// is specified and there are multiple selection nodes, then the default
/// expression simply combines all the selection nodes using an `or` operator.
///
/// Each vtkSelectionNode is used to define the selection criteria.
/// vtkSelectionNode API lets one select what kind of entities are being selected
/// (vtkSelectionNode::FieldType) and how they are being selected
/// (vtkSelectionNode::ContentType).
///
/// @sa
/// vtkSelectionNode
#[allow(non_camel_case_types)]
pub struct vtkSelection(*mut core::ffi::c_void);
impl vtkSelection {
    /// Creates a new [vtkSelection] wrapped inside `vtkNew`
    #[doc(alias = "vtkSelection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSelection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSelection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSelection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSelection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSelection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSelection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSelection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSelection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSelection_create_drop() {
    let obj = vtkSelection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSelection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a node in a vtkSelection the defines the selection criteria.
///
///
/// vtkSelectionNode helps define the selection criteria in a vtkSelection.
/// vtkSelection can comprise of multiple such vtkSelectionNode instances that
/// help define the selection.
///
/// vtkSelectionNode has two components: a list of properties (stored in a
/// vtkInformation) and a selection list (a vtkAbstractArray subclass). The
/// properties help indicate how to interpret the values specified in a
/// selection-list.
///
/// The properties can be broadly classified into three categories: core,
/// qualifiers, and information. The core properties must be specified other wise the
/// vtkSelectionNode is not considered valid. These are `FIELD_TYPE` and
/// `CONTENT_TYPE`. `FIELD_TYPE` defines what kinds of entities are being selected.
/// Since selections are used to select items in a data-object, these correspond to
/// things like cells, points, nodes, edges, rows, etc. Supported FIELD_TYPE
/// values are defined in `vtkSelectionNode::SelectionField`. `CONTENT_TYPE`
/// defines the how the selection is described. Supported values are
/// `vtkSelectionNode::SelectionContent`. For example, if CONTENT_TYPE is
/// `INDICES`, then selection is described as ids for entities being selected.
/// The ids themselves will be specified using the selection list. Thus, the
/// CONTENT_TYPE dictates what the selection list comprises.
///
/// Qualifiers are used to further qualify the selection criteria. All qualifiers
/// are optional. When present they restrict the selection. For example, when
/// selecting elements from a composite-dataset, the `COMPOSITE_INDEX` qualifier
/// can be specified of limit the selection to a specific dataset (or subtree).
///
/// Information properties are simply for informative purposes and generally used
/// to provide information about the selection when the selection is created. For
/// example, `PIXEL_COUNT` is used to indicate how many screen pixels resulted in
/// creation of this selection node.
///
/// @section SelectionTypes Selection Types
///
/// `CONTENT_TYPE` property is one of the required properties that helps
/// describe how the selection is defined. To set the content type, one can
/// access the properties store using `vtkSelectionNode::GetProperties()` and
/// then set the `CONTENT_TYPE` using the `vtkSelectionNode::CONTENT_TYPE()` key
/// or simply use `vtkSelectionNode::SetContentType`.
///
/// * `vtkSelectionNode::GLOBALIDS`: indicates that the selection is defined
/// using global ids. In VTK data-object, global ids are specified an `vtkIdTypeArray`
/// added to a appropriate `vtkDataSetAttributes` and marked as global-ids
/// using vtkDataSetAttributes API. Since global ids are expected to be unique
/// for that element type over the entire dataset, it's a convenient way of
/// defining selections. For this content-type, the selection list must be
/// a single-component, `vtkIdTypeArray` that lists all the globals ids for
/// the selected elements.
///
/// * `vtkSelectionNode::PEDIGREEIDS`: similar to `GLOBALIDS` except uses
/// pedigree ids instead of global ids.
///
/// * `vtkSelectionNode::VALUES`: this type is used to define a selection based
/// on array values. The selection list specifies the values to be selected.
/// All elements with array values in the selection list are treated as
/// selected. The qualifier COMPONENT_NUMBER is used to indicate which
/// component to use for the checks. Use `-1` for magnitude. Current
/// implementation does not support checking multiple-components or non-exact
/// matches although support for both is conceivable in future.
/// The selection list array name is used to specify the name of the array from
/// the dataset to use for the checks. Thus, for defining a selection for a
/// dataset where all `TEMP` values match those specified in the selection
/// list, ensure that the selection list array's name is set to `TEMP` as well.
///
/// * `vtkSelectionNode::INDICES`: this is similar to global ids except in this
/// case the selection list is simply the VTK element id which is 0-based
/// index of that element in the particular dataset. Often with this type of
/// selection, additional qualifiers such as `COMPOSITE_INDEX`,
/// `BLOCK_SELECTORS`, `PROCESS_ID` etc. are needed to correctly identify the
/// chosen element(s) in case of composite or distributed datasets.
///
/// * `vtkSelectionNode::FRUSTUM`: this type is used to define a frustum in world
/// coordinates that identifies the selected elements. In this case, the
/// selection list is a vtkDoubleArray with 32 values specifying the 8 frustum
/// corners in homogeneous world coordinates.
///
/// * `vtkSelectionNode::LOCATIONS`: this is used to select points (or cells)
/// near (or containing) specified locations. The selection list is a
/// 3-component vtkDoubleArray with coordinates for locations of interest.
///
/// * `vtkSelectionNode::THRESHOLDS`: this type is used to define a selection based
/// on array value ranges. This is akin to thresholding. All elements with values in
/// the specified ranges are to be treated as selected. For this content-type,
/// the selection-list is a 2-component `vtkDataArray`-subclass that specifies
/// where each tuple defines the min and max values for a range. The selection
/// list can have multiple tuples to define multiple ranges. Elements with
/// values in any of the specified ranges are treated as selected. The
/// selection list array name is used to specify the name of the array from
/// the dataset to use for the checks. Thus, for defining a selection for a
/// dataset where all `TEMP` values are within a range, ensure that the
/// selection list array's name is set to `TEMP` as well.
///
/// * `vtkSelectionNode::BLOCKS`: this type is used to select blocks in a
/// composite dataset. The term blocks is used loosely here and can correspond
/// to a block in a multiblock dataset or a partition in a partitioned-dataset.
/// The selection list is an integral type vtkDataArray subclass that can be 1-
/// or 2- component. If 1-component, it's interpreted as the composite-index
/// (also called flat index) and can be applied to any composite-dataset to
/// choose specific datasets. If 2-component, it's typically associated with
/// vtkUniformGridAMR or vtkPartitionedDataSetCollection which support 2-tuple
/// indexing to locate a dataset.
///
/// * `vtkSelectionNode::BLOCK_SELECTORS`: this is similar to BLOCKS, however
/// instead of using indices to select datasets, here, the selection list is a
/// vtkStringArray which lists selector expressions to select blocks in the
/// composite dataset. By default, the selector expressions are applied to a
/// vtkDataAssembly generated from the composite dataset that represents its
/// hierarchy (see `vtkDataAssembly::GenerateHierarchy`). However, in case of
/// vtkPartitionedDataSetCollection, one can select any other data assembly
/// associated with the vtkPartitionedDataSetCollection by naming the array
/// with the name of the assembly.
///
/// @note, currently vtkPartitionedDataSetCollection only supports a single
/// vtkDataAssembly but this may change in the future.
///
/// * `vtkSelectionNode::QUERY`: this type is primarily added for ParaView where
/// selection expression is specified as a query string. This is likely to
/// change in the future and hence applications are discouraged from using this
/// type.
///
/// @section Properties Properties
///
/// Following a properties that can be used to qualify the selection.
///
/// * `vtkSelectionNode::EPSILON()`: this is a qualifier that can be used to
/// indicate a fuzz-factor when comparing values for equality. Currently, this
/// is only used with content-type LOCATIONS, however, it can be expanded to
/// other selection types in the future.
///
/// * `vtkSelectionNode::CONTAINING_CELLS()`: this qualifier is intended to be
/// used with field-type `POINT`. When present, it indicates that while the
/// selection criteria selects a collection of points the selection should be
/// formed using cell containing the chosen points.
///
/// * `vtkSelectionNode::CONNECTED_LAYERS()`: a qualifier used to expand the
/// definition of selected elements to connected elements for the specified
/// number of layers. Layers can only be positive to grow the selection.
///
/// * `vtkSelectionNode::CONNECTED_LAYERS_REMOVE_SEED()`: this qualifier indicates
/// that when using a number of CONNECTED_LAYERS >= 1, the initial selection will
/// not be kept.
///
/// * `vtkSelectionNode::CONNECTED_LAYERS_REMOVE_INTERMEDIATE_LAYERS()`: this qualifier
/// indicates that when using a number of CONNECTED_LAYERS >= 2, the intermediate layers
/// will not be kept.
///
/// * `vtkSelectionNode::INVERSE()`: a qualifier that causes the selection to be
/// inverted i.e. all elements not chosen by the criteria are to be treated
/// as selected.
///
/// * `vtkSelectionNode::COMPONENT_NUMBER()`: for VALUES and THRESHOLDS selection
/// types, this qualifier identifies the array component of interest. -1
/// indicates magnitude.
///
/// * `vtkSelectionNode::PROCESS_ID()`: limits the selection to a particular
/// rank in a distributed environment.
///
/// * `vtkSelectionNode::COMPOSITE_INDEX()`: a qualifier used to limit the
/// selection to a specific composite index for a composite-dataset.
///
/// * `vtkSelectionNode::ASSEMBLY_NAME()`, `vtkSelectionNode::SELECTORS()`:
/// similar to composite index, except uses data-assembly and selectors to
/// limit the selection to a subset of nodes in a composite-dataset.
///
/// * `vtkSelectionNode::HIERARCHICAL_LEVEL()`,
/// `vtkSelectionNode::HIERARCHICAL_INDEX()`: similar to composite index, except
/// uses level and index for an AMR dataset so limit the selection to a
/// specific AMR level or dataset.
///
/// Following for properties that are primarily intended to provide additional
/// information when the selection is created.
///
/// * `vtkSelectionNode::ZBUFFER_VALUE()`: an information qualifier representing
/// the z-depth for a particular selection when it was created.
///
/// * `vtkSelectionNode::PIXEL_COUNT()`: a qualifier used to provide a count for
/// the number of pixels that resulted in this selection.
///
/// * `vtkSelectionNode::SOURCE()`, `vtkSelectionNode::SOURCE_ID()`: provides
/// information about data producer or selection originator. The interpretation
/// is very specific to the creator creating the selection and varies greatly
/// with VTK.
///
/// * `vtkSelectionNode::PROP(), `vtkSelectionNode::PROP_ID()`: similar to
/// SOURCE/SOURCE_ID except is used to represent a rendering prop from which
/// the selection was created.
///
/// @warning
/// No SelectionList is created by default. It should be assigned.
///
/// @section SelectionFieldMismatch vtkSelectionNode::SelectionField and
/// vtkDataSetAttribute::AttributeTypes
///
/// Strictly speaking, vtkSelectionNode::SelectionField maps directly to
/// vtkDataSetAttribute::AttributeTypes. However, the two enum values are not
/// identical for historical reasons. Use
/// `vtkSelectionNode::ConvertSelectionFieldToAttributeType` and
/// `vtkSelectionNode::ConvertAttributeTypeToSelectionField` to convert between
/// the two.
#[allow(non_camel_case_types)]
pub struct vtkSelectionNode(*mut core::ffi::c_void);
impl vtkSelectionNode {
    /// Creates a new [vtkSelectionNode] wrapped inside `vtkNew`
    #[doc(alias = "vtkSelectionNode")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSelectionNode_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSelectionNode_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSelectionNode_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSelectionNode_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSelectionNode {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSelectionNode {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSelectionNode_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSelectionNode_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSelectionNode_create_drop() {
    let obj = vtkSelectionNode::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSelectionNode(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// helper class to perform cell tessellation
///
///
/// vtkSimpleCellTessellator is a helper class to perform adaptive tessellation
/// of particular cell topologies. The major purpose for this class is to
/// transform higher-order cell types (e.g., higher-order finite elements)
/// into linear cells that can then be easily visualized by VTK. This class
/// works in conjunction with the vtkGenericDataSet and vtkGenericAdaptorCell
/// classes.
///
/// This algorithm is based on edge subdivision. An error metric along each
/// edge is evaluated, and if the error is greater than some tolerance, the
/// edge is subdivided (as well as all connected 2D and 3D cells). The process
/// repeats until the error metric is satisfied. Since the algorithm is based
/// on edge subdivision it inherently avoid T-junctions.
///
/// A significant issue addressed by this algorithm is to ensure face
/// compatibility across neighboring cells. That is, diagonals due to face
/// triangulation must match to ensure that the mesh is compatible. The
/// algorithm employs a precomputed table to accelerate the tessellation
/// process. The table was generated with the help of vtkOrderedTriangulator
/// the basic idea is that the choice of diagonal is made only by considering the
/// relative value of the point ids.
///
/// @sa
/// vtkGenericCellTessellator vtkGenericSubdivisionErrorMetric vtkAttributesErrorMetric
/// vtkGeometricErrorMetric vtkViewDependentErrorMetric
#[allow(non_camel_case_types)]
pub struct vtkSimpleCellTessellator(*mut core::ffi::c_void);
impl vtkSimpleCellTessellator {
    /// Creates a new [vtkSimpleCellTessellator] wrapped inside `vtkNew`
    #[doc(alias = "vtkSimpleCellTessellator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSimpleCellTessellator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSimpleCellTessellator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSimpleCellTessellator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSimpleCellTessellator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSimpleCellTessellator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSimpleCellTessellator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSimpleCellTessellator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSimpleCellTessellator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSimpleCellTessellator_create_drop() {
    let obj = vtkSimpleCellTessellator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSimpleCellTessellator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Objects that compute
///
/// geometry-based error during cell tessellation according to
/// some max angle.
///
///
/// It is a concrete error metric, based on a geometric criterium:
/// a max angle between the chord passing through the midpoint and one of the
/// endpoints and the other chord passing through the midpoint and the other
/// endpoint of the edge. It is related to the flatness of an edge.
///
/// @sa
/// vtkGenericCellTessellator vtkGenericSubdivisionErrorMetric
#[allow(non_camel_case_types)]
pub struct vtkSmoothErrorMetric(*mut core::ffi::c_void);
impl vtkSmoothErrorMetric {
    /// Creates a new [vtkSmoothErrorMetric] wrapped inside `vtkNew`
    #[doc(alias = "vtkSmoothErrorMetric")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSmoothErrorMetric_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSmoothErrorMetric_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSmoothErrorMetric_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSmoothErrorMetric_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSmoothErrorMetric {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSmoothErrorMetric {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSmoothErrorMetric_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSmoothErrorMetric_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSmoothErrorMetric_create_drop() {
    let obj = vtkSmoothErrorMetric::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSmoothErrorMetric(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// provides a method for sorting field data
///
///
///
/// vtkSortFieldData is used to sort data, based on its value, or with an
/// associated key, into either ascending or descending order. This is useful
/// for operations like selection, or analysis, when evaluating and processing
/// data.
///
/// This class, which extends the base functionality of vtkSortDataArray,
/// is used to sort field data and its various subclasses (vtkFieldData,
/// vtkDataSetAttributes, vtkPointData, vtkCellData, etc.)
///
/// @warning
/// This class has been threaded with vtkSMPTools. Using TBB or other
/// non-sequential type (set in the CMake variable
/// VTK_SMP_IMPLEMENTATION_TYPE) may improve performance significantly on
/// multi-core machines.
///
/// @warning
/// The sort methods below are static, hence the sorting methods can be
/// used without instantiating the class. All methods are thread safe.
///
/// @sa
/// vtkSortDataArray
#[allow(non_camel_case_types)]
pub struct vtkSortFieldData(*mut core::ffi::c_void);
impl vtkSortFieldData {
    /// Creates a new [vtkSortFieldData] wrapped inside `vtkNew`
    #[doc(alias = "vtkSortFieldData")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSortFieldData_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSortFieldData_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSortFieldData_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSortFieldData_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSortFieldData {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSortFieldData {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSortFieldData_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSortFieldData_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSortFieldData_create_drop() {
    let obj = vtkSortFieldData::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSortFieldData(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a sphere
///
///
/// vtkSphere computes the implicit function and/or gradient for a sphere.
/// vtkSphere is a concrete implementation of vtkImplicitFunction. Additional
/// methods are available for sphere-related computations, such as computing
/// bounding spheres for a set of points, or set of spheres.
#[allow(non_camel_case_types)]
pub struct vtkSphere(*mut core::ffi::c_void);
impl vtkSphere {
    /// Creates a new [vtkSphere] wrapped inside `vtkNew`
    #[doc(alias = "vtkSphere")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSphere_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSphere_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSphere_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSphere_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSphere {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSphere {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSphere_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSphere_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSphere_create_drop() {
    let obj = vtkSphere::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSphere(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a set of spheres
///
///
/// vtkSpheres computes the implicit function and function gradient for a set
/// of spheres. The spheres are combined via a union operation (i.e., the
/// minimum value from the evaluation of all spheres is taken).
///
/// The function value is the distance of a point to the closest sphere, with
/// negative values interior to the spheres, positive outside the spheres, and
/// distance=0 on the spheres surface.  The function gradient is the sphere
/// normal at the function value.
///
/// @sa
/// vtkPlanes vtkImplicitBoolean
#[allow(non_camel_case_types)]
pub struct vtkSpheres(*mut core::ffi::c_void);
impl vtkSpheres {
    /// Creates a new [vtkSpheres] wrapped inside `vtkNew`
    #[doc(alias = "vtkSpheres")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSpheres_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSpheres_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSpheres_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSpheres_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSpheres {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSpheres {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSpheres_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSpheres_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSpheres_create_drop() {
    let obj = vtkSpheres::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSpheres(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Traverse a collection of points in spherical ordering.
///
///
///
/// vtkSphericalPointIterator is a state-based iterator for traversing a set
/// of points (i.e., a neighborhood of points) in a dataset, providing a point
/// traversal order across user-defined "axes" which span a 2D or 3D space
/// (typically a circle or sphere). The points along each axes may be sorted
/// in increasing radial order. To define the points, specify a dataset (i.e.,
/// its associated points, whether the points are represented implicitly or
/// explicitly) and an associated neighborhood over which to iterate. Methods
/// for iterating over the points are provided.
///
/// For example, consider the axes of iteration to be the four rays emanating
/// from the center of a square and passing through the center of each of the
/// four edges of the square. Points to be iterated over are associated (using
/// a dot product) with each of the four axes, and then can be sorted along
/// each axis. Then the order of iteration is then: (axis0,pt0), (axis1,pt0),
/// (axis2,pt0), (axis3,pt0), (axis0,pt1), (axis1,pt1), (axis2,pt1),
/// (axis3,pt1), (axis0,pt2), (axis1,pt2), (axis2,pt2), (axis3,pt2), and so on
/// in a "spiraling" fashion until all points are visited. Thus the order of
/// visitation is: iteration i visits all N axes in order, returning the jth
/// point sorted along each of the N axes (i.e., i increases the fastest).
/// Alternatively, methods exist to randomly access points, or points
/// associated with an axes, so that custom iteration methods can be defined.
///
/// The iterator can be defined with any number of axes (defined by 3D
/// vectors). The axes must not be coincident, and typically are equally
/// spaced from one another. The order which the axes are defined determines
/// the order in which the axes (and hence the points) are traversed. So for
/// example, in a 2D sphere, four axes in the (-x,+x,-y,+y) directions would
/// provide a "ping pong" iteration, while four axes ordered in the
/// (+x,+y,-x,-y) directions would provide a counterclockwise rotation
/// iteration.
///
/// The iterator provides thread-safe iteration of dataset points. It supports
/// both random and forward iteration.
///
/// @warning
/// The behavior of the iterator depends on the ordering of the iteration
/// axes. It is possible to obtain a wide variety of iteration patterns
/// depending on these axes. For example, if only one axis is defined, then a
/// "linear" pattern is possible (i.e., visiting points in the half space
/// defined by the vector); if two axes, then a "diagonal" iteration pattern;
/// and so on. Note that points are sorted along the iteration axes depending
/// on the their projection onto them (e.g., using the dot product). Because
/// only points with positive projection are associated with an axis, it is
/// possible that some points in the neighborhood will not be processed (i.e.,
/// if a point in the neighborhood does not positively project onto any of the
/// axes, then it will not be iterated over). Thus if all points are to be
/// iterated over, then the axes must form a basis which covers all points
/// using positive projections.
///
/// @sa
/// vtkVoronoi2D vtkVoronoi3D vtkStaticPointLocator vtkPointLocator
#[allow(non_camel_case_types)]
pub struct vtkSphericalPointIterator(*mut core::ffi::c_void);
impl vtkSphericalPointIterator {
    /// Creates a new [vtkSphericalPointIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkSphericalPointIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSphericalPointIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSphericalPointIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSphericalPointIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSphericalPointIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSphericalPointIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSphericalPointIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSphericalPointIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSphericalPointIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSphericalPointIterator_create_drop() {
    let obj = vtkSphericalPointIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSphericalPointIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// object represents upward pointers from points
///
/// to list of cells using each point
///
/// vtkStaticCellLinks is a supplemental object to vtkCellArray and
/// vtkCellTypes, enabling access from points to the cells using the
/// points. vtkStaticCellLinks is an array of links, each link represents a
/// list of cell ids using a particular point. The information provided by
/// this object can be used to determine cell neighbors and construct other
/// local topological information. This class is a faster implementation of
/// vtkCellLinks. However, it cannot be incrementally constructed; it is meant
/// to be constructed once (statically) and must be rebuilt if the cells
/// change.
///
/// @warning
/// This is a drop-in replacement for vtkCellLinks using static link
/// construction. It uses the templated vtkStaticCellLinksTemplate class,
/// instantiating vtkStaticCellLinksTemplate with a vtkIdType template
/// parameter. Note that for best performance, the vtkStaticCellLinksTemplate
/// class may be used directly, instantiating it with the appropriate id
/// type. This class is also wrappable and can be used from an interpreted
/// language such as Python.
///
/// @sa
/// vtkCellLinks vtkStaticCellLinksTemplate
#[allow(non_camel_case_types)]
pub struct vtkStaticCellLinks(*mut core::ffi::c_void);
impl vtkStaticCellLinks {
    /// Creates a new [vtkStaticCellLinks] wrapped inside `vtkNew`
    #[doc(alias = "vtkStaticCellLinks")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStaticCellLinks_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStaticCellLinks_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStaticCellLinks_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStaticCellLinks_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStaticCellLinks {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStaticCellLinks {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStaticCellLinks_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStaticCellLinks_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStaticCellLinks_create_drop() {
    let obj = vtkStaticCellLinks::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStaticCellLinks(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// perform fast cell location operations
///
///
/// vtkStaticCellLocator is a type of vtkAbstractCellLocator that accelerates
/// certain operations when performing spatial operations on cells. These
/// operations include finding a point that contains a cell, and intersecting
/// cells with a line.
///
/// vtkStaticCellLocator is an accelerated version of vtkCellLocator. It is
/// threaded (via vtkSMPTools), and supports one-time static construction
/// (i.e., incremental cell insertion is not supported).
///
/// @warning
/// vtkStaticCellLocator utilizes the following parent class parameters:
/// - Automatic                   (default true)
/// - NumberOfCellsPerNode        (default 10)
/// - UseExistingSearchStructure  (default false)
///
/// vtkStaticCellLocator does NOT utilize the following parameters:
/// - CacheCellBounds             (always cached)
/// - Tolerance
/// - Level
/// - MaxLevel
/// - RetainCellLists
///
/// @warning
/// This class is templated. It may run slower than serial execution if the code
/// is not optimized during compilation. Build in Release or ReleaseWithDebugInfo.
///
/// @sa
/// vtkAbstractCellLocator vtkCellLocator vtkCellTreeLocator vtkModifiedBSPTree vtkOBBTree
#[allow(non_camel_case_types)]
pub struct vtkStaticCellLocator(*mut core::ffi::c_void);
impl vtkStaticCellLocator {
    /// Creates a new [vtkStaticCellLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkStaticCellLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStaticCellLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStaticCellLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStaticCellLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStaticCellLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStaticCellLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStaticCellLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStaticCellLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStaticCellLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStaticCellLocator_create_drop() {
    let obj = vtkStaticCellLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStaticCellLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// quickly locate points in 3-space
///
///
/// vtkStaticPointLocator is a spatial search object to quickly locate points
/// in 3D.  vtkStaticPointLocator works by dividing a specified region of
/// space into a regular array of cuboid buckets, and then keeping a
/// list of points that lie in each bucket. Typical operation involves giving
/// a position in 3D and finding the closest point; or finding the N closest
/// points.
///
/// vtkStaticPointLocator is an accelerated version of vtkPointLocator. It is
/// threaded (via vtkSMPTools), and supports one-time static construction
/// (i.e., incremental point insertion is not supported). If you need to
/// incrementally insert points, use the vtkPointLocator or its kin to do so.
///
/// @warning
/// This class is templated. It may run slower than serial execution if the code
/// is not optimized during compilation. Build in Release or ReleaseWithDebugInfo.
///
/// @warning
/// Make sure that you review the documentation for the superclasses
/// vtkAbstactPointLocator and vtkLocator. In particular the Automatic
/// data member can be used to automatically determine divisions based
/// on the average number of points per bucket.
///
/// @warning
/// Other types of spatial locators have been developed such as octrees and
/// kd-trees. These are often more efficient for the operations described
/// here.
///
/// @warning
/// Frequently vtkStaticPointLocator is used in lieu of vtkPointLocator.
/// They are very similar in terms of algorithmic approach, however
/// vtkStaticCellLocator is threaded and is typically much faster for
/// a large number of points (on the order of 3-5x faster). For small numbers
/// of points, vtkPointLocator is just as fast as vtkStaticPointLocator.
///
/// @sa
/// vtkPointLocator vtkCellLocator vtkLocator vtkAbstractPointLocator
#[allow(non_camel_case_types)]
pub struct vtkStaticPointLocator(*mut core::ffi::c_void);
impl vtkStaticPointLocator {
    /// Creates a new [vtkStaticPointLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkStaticPointLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStaticPointLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStaticPointLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStaticPointLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStaticPointLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStaticPointLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStaticPointLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStaticPointLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStaticPointLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStaticPointLocator_create_drop() {
    let obj = vtkStaticPointLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStaticPointLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// quickly locate points in 2-space
///
///
/// vtkStaticPointLocator2D is a spatial search object to quickly locate points
/// in 2D.  vtkStaticPointLocator2D works by dividing a specified region of
/// space into a regular array of rectilinear buckets, and then keeping a
/// list of points that lie in each bucket. Typical operation involves giving
/// a position in 2D and finding the closest point; or finding the N closest
/// points. (Note that the more general vtkStaticPointLocator is available
/// for 3D operations.) Other specialized methods for 2D have also been provided.
///
/// vtkStaticPointLocator2D is an accelerated version of vtkPointLocator. It is
/// threaded (via vtkSMPTools), and supports one-time static construction
/// (i.e., incremental point insertion is not supported). If you need to
/// incrementally insert points, use the vtkPointLocator or its kin to do so.
///
/// Note that to satisfy the superclass's API, methods often assume a 3D point
/// is provided. However, only the x,y values are used for processing. The
/// z-value is only used to define location of the 2D plane.
///
/// @warning
/// This class is templated. It may run slower than serial execution if the code
/// is not optimized during compilation. Build in Release or ReleaseWithDebugInfo.
///
/// @warning
/// Make sure that you review the documentation for the superclasses
/// vtkAbstactPointLocator and vtkLocator. In particular the Automatic
/// data member can be used to automatically determine divisions based
/// on the average number of points per bucket.
///
/// @warning
/// Other types of spatial locators have been developed such as octrees and
/// kd-trees. These are often more efficient for the operations described
/// here.
///
/// @sa
/// vtkStaticPointLocator vtkPointLocator vtkCellLocator vtkLocator
/// vtkAbstractPointLocator
#[allow(non_camel_case_types)]
pub struct vtkStaticPointLocator2D(*mut core::ffi::c_void);
impl vtkStaticPointLocator2D {
    /// Creates a new [vtkStaticPointLocator2D] wrapped inside `vtkNew`
    #[doc(alias = "vtkStaticPointLocator2D")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStaticPointLocator2D_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStaticPointLocator2D_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStaticPointLocator2D_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStaticPointLocator2D_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStaticPointLocator2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStaticPointLocator2D {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStaticPointLocator2D_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStaticPointLocator2D_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStaticPointLocator2D_create_drop() {
    let obj = vtkStaticPointLocator2D::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStaticPointLocator2D(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit object to represent cell connectivity
///
///
/// vtkStructuredCellArray stores dataset topologies as an structured connectivity table
/// listing the point ids that make up each cell.
///
/// Internally, the connectivity is stored as a vtkImplicitArray that is constructed
/// using the SetData function by providing the dimensions of the dataset and a flag
/// indicating whether the data should use voxel/pixel orientation.
///
/// This class was designed as a more performant alternative to vtkStructuredData::GetCellPoints.
///
/// @sa
/// vtkCellArray vtkAbstractCellArray
#[allow(non_camel_case_types)]
pub struct vtkStructuredCellArray(*mut core::ffi::c_void);
impl vtkStructuredCellArray {
    /// Creates a new [vtkStructuredCellArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkStructuredCellArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStructuredCellArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStructuredCellArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStructuredCellArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStructuredCellArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStructuredCellArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStructuredCellArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStructuredCellArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStructuredCellArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStructuredCellArray_create_drop() {
    let obj = vtkStructuredCellArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStructuredCellArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// helper class to aid working with structured
///
/// extents.
///
///
/// vtkStructuredExtent is an helper class that helps in arithmetic with
/// structured extents. It defines a bunch of static methods (most of which are
/// inlined) to aid in dealing with extents.
#[allow(non_camel_case_types)]
pub struct vtkStructuredExtent(*mut core::ffi::c_void);
impl vtkStructuredExtent {
    /// Creates a new [vtkStructuredExtent] wrapped inside `vtkNew`
    #[doc(alias = "vtkStructuredExtent")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStructuredExtent_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStructuredExtent_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStructuredExtent_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStructuredExtent_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStructuredExtent {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStructuredExtent {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStructuredExtent_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStructuredExtent_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStructuredExtent_create_drop() {
    let obj = vtkStructuredExtent::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStructuredExtent(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// topologically regular array of data
///
///
/// vtkStructuredGrid is a data object that is a concrete implementation of
/// vtkDataSet. vtkStructuredGrid represents a geometric structure that is a
/// topologically regular array of points. The topology is that of a cube that
/// has been subdivided into a regular array of smaller cubes. Each point/cell
/// can be addressed with i-j-k indices. Examples include finite difference
/// grids.
///
/// The order and number of points must match that specified by the dimensions
/// of the grid. The point order increases in i fastest (from 0<=i<dims[0]),
/// then j (0<=j<dims[1]), then k (0<=k<dims[2]) where dims[] are the
/// dimensions of the grid in the i-j-k topological directions. The number of
/// points is dims[0]*dims[1]*dims[2]. The same is true for the cells of the
/// grid. The order and number of cells must match that specified by the
/// dimensions of the grid. The cell order increases in i fastest (from
/// 0<=i<(dims[0]-1)), then j (0<=j<(dims[1]-1)), then k (0<=k<(dims[2]-1))
/// The number of cells is (dims[0]-1)*(dims[1]-1)*(dims[2]-1).
///
/// vtkStructuredGrid has the ability to blank,
/// or "turn-off" points and cells in the dataset. This is done by setting
/// vtkDataSetAttributes::HIDDENPOINT or vtkDataSetAttributes::HIDDENCELL
/// in the ghost array for each point / cell that needs to be blanked.
#[allow(non_camel_case_types)]
pub struct vtkStructuredGrid(*mut core::ffi::c_void);
impl vtkStructuredGrid {
    /// Creates a new [vtkStructuredGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkStructuredGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStructuredGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStructuredGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStructuredGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStructuredGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStructuredGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStructuredGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStructuredGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStructuredGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStructuredGrid_create_drop() {
    let obj = vtkStructuredGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStructuredGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A subclass of ImageData.
///
///
/// StructuredPoints is a subclass of ImageData that requires the data extent
/// to exactly match the update extent. Normal image data allows that the
/// data extent may be larger than the update extent.
/// StructuredPoints also defines the origin differently that vtkImageData.
/// For structured points the origin is the location of first point.
/// Whereas images define the origin as the location of point 0, 0, 0.
/// Image Origin is stored in ivar, and structured points
/// have special methods for setting/getting the origin/extents.
#[allow(non_camel_case_types)]
pub struct vtkStructuredPoints(*mut core::ffi::c_void);
impl vtkStructuredPoints {
    /// Creates a new [vtkStructuredPoints] wrapped inside `vtkNew`
    #[doc(alias = "vtkStructuredPoints")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStructuredPoints_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStructuredPoints_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStructuredPoints_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStructuredPoints_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStructuredPoints {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStructuredPoints {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStructuredPoints_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStructuredPoints_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStructuredPoints_create_drop() {
    let obj = vtkStructuredPoints::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStructuredPoints(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain a list of structured points data objects
///
///
/// vtkStructuredPointsCollection is an object that creates and manipulates
/// ordered lists of structured points datasets. See also vtkCollection and
/// subclasses.
#[allow(non_camel_case_types)]
pub struct vtkStructuredPointsCollection(*mut core::ffi::c_void);
impl vtkStructuredPointsCollection {
    /// Creates a new [vtkStructuredPointsCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkStructuredPointsCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStructuredPointsCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStructuredPointsCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStructuredPointsCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStructuredPointsCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStructuredPointsCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStructuredPointsCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStructuredPointsCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStructuredPointsCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStructuredPointsCollection_create_drop() {
    let obj = vtkStructuredPointsCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStructuredPointsCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// implicit function for a Superquadric
///
///
/// vtkSuperquadric computes the implicit function and function gradient
/// for a superquadric. vtkSuperquadric is a concrete implementation of
/// vtkImplicitFunction.  The superquadric is centered at Center and axes
/// of rotation is along the y-axis. (Use the superclass'
/// vtkImplicitFunction transformation matrix if necessary to reposition.)
/// Roundness parameters (PhiRoundness and ThetaRoundness) control
/// the shape of the superquadric.  The Toroidal boolean controls whether
/// a toroidal superquadric is produced.  If so, the Thickness parameter
/// controls the thickness of the toroid:  0 is the thinnest allowable
/// toroid, and 1 has a minimum sized hole.  The Scale parameters allow
/// the superquadric to be scaled in x, y, and z (normal vectors are correctly
/// generated in any case).  The Size parameter controls size of the
/// superquadric.
///
/// This code is based on "Rigid physically based superquadrics", A. H. Barr,
/// in "Graphics Gems III", David Kirk, ed., Academic Press, 1992.
///
/// @warning
/// The Size and Thickness parameters control coefficients of superquadric
/// generation, and may do not exactly describe the size of the superquadric.
#[allow(non_camel_case_types)]
pub struct vtkSuperquadric(*mut core::ffi::c_void);
impl vtkSuperquadric {
    /// Creates a new [vtkSuperquadric] wrapped inside `vtkNew`
    #[doc(alias = "vtkSuperquadric")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSuperquadric_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSuperquadric_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSuperquadric_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSuperquadric_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSuperquadric {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSuperquadric {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSuperquadric_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSuperquadric_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSuperquadric_create_drop() {
    let obj = vtkSuperquadric::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSuperquadric(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A table, which contains similar-typed columns of data
///
///
///
/// vtkTable is a basic data structure for storing columns of data.
/// Internally, columns are stored in a vtkDataSetAttributes structure called
/// RowData. However, using the vtkTable API additionally ensures that every column
/// has the same number of entries, and provides row access (using vtkVariantArray)
/// and single entry access (using vtkVariant).
///
/// Inserting or removing rows via the class API preserves existing table data where possible.
///
/// The "RemoveRow*" and SetNumberOfRows() operations will not release memory. Call on SqueezeRows()
/// to achieve this after performing the operations.
///
/// The field data inherited from vtkDataObject may be used to store metadata
/// related to the table.
///
/// @warning
/// You should use the vtkTable API to change the table data. Performing
/// operations on the object returned by GetRowData() may
/// yield unexpected results. vtkTable does allow the user to set the field
/// data using SetRowData(); the number of rows in the table is determined
/// by the number of tuples in the first array (it is assumed that all arrays
/// are the same length).
///
/// @warning
/// Each column added with AddColumn <b>must</b> have its name set to a unique,
/// non-empty string in order for GetValue() to function properly.
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class API.
#[allow(non_camel_case_types)]
pub struct vtkTable(*mut core::ffi::c_void);
impl vtkTable {
    /// Creates a new [vtkTable] wrapped inside `vtkNew`
    #[doc(alias = "vtkTable")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTable_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTable_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTable_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTable_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTable {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTable {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTable_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTable_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTable_create_drop() {
    let obj = vtkTable::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTable(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a 3D cell that represents a tetrahedron
///
///
/// vtkTetra is a concrete implementation of vtkCell to represent a 3D
/// tetrahedron. vtkTetra uses the standard isoparametric shape functions
/// for a linear tetrahedron. The tetrahedron is defined by the four points
/// (0-3); where (0,1,2) is the base of the tetrahedron which, using the
/// right hand rule, forms a triangle whose normal points in the direction
/// of the fourth point.
///
/// @sa
/// vtkConvexPointSet vtkHexahedron vtkPyramid vtkVoxel vtkWedge
#[allow(non_camel_case_types)]
pub struct vtkTetra(*mut core::ffi::c_void);
impl vtkTetra {
    /// Creates a new [vtkTetra] wrapped inside `vtkNew`
    #[doc(alias = "vtkTetra")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTetra_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTetra_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTetra_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTetra_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTetra {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTetra {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTetra_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTetra_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTetra_create_drop() {
    let obj = vtkTetra::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTetra(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A rooted tree data structure.
///
///
///
/// vtkTree is a connected directed graph with no cycles. A tree is a type of
/// directed graph, so works with all graph algorithms.
///
/// vtkTree is a read-only data structure.
/// To construct a tree, create an instance of vtkMutableDirectedGraph.
/// Add vertices and edges with AddVertex() and AddEdge(). You may alternately
/// start by adding a single vertex as the root then call graph->AddChild(parent)
/// which adds a new vertex and connects the parent to the child.
/// The tree MUST have all edges in the proper direction, from parent to child.
/// After building the tree, call tree->CheckedShallowCopy(graph) to copy the
/// structure into a vtkTree. This method will return false if the graph is
/// an invalid tree.
///
/// vtkTree provides some convenience methods for obtaining the parent and
/// children of a vertex, for finding the root, and determining if a vertex
/// is a leaf (a vertex with no children).
///
/// @sa
/// vtkDirectedGraph vtkMutableDirectedGraph vtkGraph
#[allow(non_camel_case_types)]
pub struct vtkTree(*mut core::ffi::c_void);
impl vtkTree {
    /// Creates a new [vtkTree] wrapped inside `vtkNew`
    #[doc(alias = "vtkTree")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTree_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTree_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTree_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTree_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTree {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTree {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTree_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTree_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTree_create_drop() {
    let obj = vtkTree::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTree(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// breadth first search iterator through a vtkTree
///
///
///
/// vtkTreeBFSIterator performs a breadth first search traversal of a tree.
///
/// After setting up the iterator, the normal mode of operation is to
/// set up a <code>while(iter->HasNext())</code> loop, with the statement
/// <code>vtkIdType vertex = iter->Next()</code> inside the loop.
///
/// @par Thanks:
/// Thanks to David Doria for submitting this class.
#[allow(non_camel_case_types)]
pub struct vtkTreeBFSIterator(*mut core::ffi::c_void);
impl vtkTreeBFSIterator {
    /// Creates a new [vtkTreeBFSIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkTreeBFSIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTreeBFSIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTreeBFSIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTreeBFSIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTreeBFSIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTreeBFSIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTreeBFSIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTreeBFSIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTreeBFSIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTreeBFSIterator_create_drop() {
    let obj = vtkTreeBFSIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTreeBFSIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// depth first iterator through a vtkGraph
///
///
///
/// vtkTreeDFSIterator performs a depth first search traversal of a tree.
///
/// First, you must set the tree on which you are going to iterate, and then
/// optionally set the starting vertex and mode. The mode is either
/// DISCOVER (default), in which case vertices are visited as they are first
/// reached, or FINISH, in which case vertices are visited when they are
/// done, i.e. all adjacent vertices have been discovered already.
///
/// After setting up the iterator, the normal mode of operation is to
/// set up a <code>while(iter->HasNext())</code> loop, with the statement
/// <code>vtkIdType vertex = iter->Next()</code> inside the loop.
#[allow(non_camel_case_types)]
pub struct vtkTreeDFSIterator(*mut core::ffi::c_void);
impl vtkTreeDFSIterator {
    /// Creates a new [vtkTreeDFSIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkTreeDFSIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTreeDFSIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTreeDFSIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTreeDFSIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTreeDFSIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTreeDFSIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTreeDFSIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTreeDFSIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTreeDFSIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTreeDFSIterator_create_drop() {
    let obj = vtkTreeDFSIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTreeDFSIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 27-node isoparametric hexahedron
///
///
/// vtkTriQuadraticHexahedron is a concrete implementation of vtkNonLinearCell to
/// represent a three-dimensional, 27-node isoparametric triquadratic
/// hexahedron. The interpolation is the standard finite element, triquadratic
/// isoparametric shape function. The cell includes 8 edge nodes, 12 mid-edge nodes,
/// 6 mid-face nodes and one mid-volume node. The ordering of the 27 points defining the
/// cell is point ids (0-7,8-19, 20-25, 26)
/// where point ids 0-7 are the eight corner vertices of the cube; followed by
/// twelve midedge nodes (8-19); followed by 6 mid-face nodes (20-25) and the last node (26)
/// is the mid-volume node. Note that these midedge nodes correspond lie
/// on the edges defined by (0,1), (1,2), (2,3), (3,0), (4,5), (5,6), (6,7),
/// (7,4), (0,4), (1,5), (2,6), (3,7). The mid-surface nodes lies on the faces
/// defined by (first edge nodes id's, than mid-edge nodes id's):
/// (0,1,5,4;8,17,12,16), (1,2,6,5;9,18,13,17), (2,3,7,6,10,19,14,18),
/// (3,0,4,7;11,16,15,19), (0,1,2,3;8,9,10,11), (4,5,6,7;12,13,14,15).
/// The last point lies in the center of the cell (0,1,2,3,4,5,6,7).
///
/// \verbatim
///
/// top
/// 7--14--6
/// |      |
/// 15  25  13
/// |      |
/// 4--12--5
///
/// middle
/// 19--23--18
/// |      |
/// 20  26  21
/// |      |
/// 16--22--17
///
/// bottom
/// 3--10--2
/// |      |
/// 11  24  9
/// |      |
/// 0-- 8--1
///
/// \endverbatim
///
///
/// @sa
/// vtkQuadraticEdge vtkQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticQuad vtkQuadraticPyramid vtkQuadraticWedge
/// vtkBiQuadraticQuad
///
/// @par Thanks:
/// Thanks to Soeren Gebbert who developed this class and
/// integrated it into VTK 5.0.
#[allow(non_camel_case_types)]
pub struct vtkTriQuadraticHexahedron(*mut core::ffi::c_void);
impl vtkTriQuadraticHexahedron {
    /// Creates a new [vtkTriQuadraticHexahedron] wrapped inside `vtkNew`
    #[doc(alias = "vtkTriQuadraticHexahedron")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTriQuadraticHexahedron_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTriQuadraticHexahedron_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTriQuadraticHexahedron_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTriQuadraticHexahedron_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTriQuadraticHexahedron {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTriQuadraticHexahedron {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTriQuadraticHexahedron_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTriQuadraticHexahedron_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTriQuadraticHexahedron_create_drop() {
    let obj = vtkTriQuadraticHexahedron::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTriQuadraticHexahedron(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// cell represents a parabolic, 19-node isoparametric pyramid
///
///
/// vtkTriQuadraticPyramid is a concrete implementation of vtkNonLinearCell to
/// represent a second order three-dimensional isoparametric 19-node pyramid.
/// The interpolation is the standard finite element, tri-quadratic
/// isoparametric shape function. The cell includes 5 corner nodes, 8 mid-edge nodes,
/// 5 mid-face nodes, and 1 volumetric centroid node. The ordering of the nineteen points
/// defining the cell is point ids (0-4, 5-12, 13-17, 18), where point ids 0-4 are the five
/// corner vertices of the pyramid; followed by 8 mid-edge nodes (5-12); followed by
/// 5 mid-face nodes (13-17), and the last node (19) is the volumetric centroid node.
/// Note that these mid-edge nodes lie on the edges defined by (0, 1), (1, 2), (2, 3),
/// (3, 0), (0, 4), (1, 4), (2, 4), (3, 4), respectively. The mid-face nodes lie on the
/// faces defined by (first corner nodes id's, then mid-edge node id's):
/// quadrilateral face: (0, 3, 2, 1, 8, 7, 6, 5), triangle face 1: (0, 1, 4, 5, 10, 9),
/// triangle face 2: (1, 2, 4, 6, 11, 10), triangle face 3: (2, 3, 4, 7, 12, 11),
/// triangle face 5: (3, 0, 4, 8, 9, 12). The last point lies in the center of the cell
/// (0, 1, 2, 3, 4). The parametric location of vertex #4 is [0.5, 0.5, 1].
///
/// @note It should be noted that the parametric coordinates that describe this cell
/// are not distorted like in vtkPyramid and vtkQuadraticPyramid, which are a collapsed
/// hexahedron. They are the actual uniform isoparametric coordinates, which are described
/// in Browning's dissertation (see thanks section), but they are converted to [0, 1] space,
/// and the nodes are rotated so that node-0 has x = 0, y = 0, while maintaining the CCW order.
///
/// \verbatim
/// Description of 19-node pyramid from bottom to top (based on the z-axis).
///
/// base quadrilateral including mid-edge nodes and mid-face node:
/// 3-- 7--2
/// |      |
/// 8  13  6
/// |      |
/// 0-- 5--1
///
/// volumetric centroid node:
///
///
/// 18
///
///
/// mid-face nodes of triangular faces:
///
/// 16
/// /  \
/// 17    15
/// \  /
/// 14
///
/// mid-edge nodes of triangular faces:
///
/// 12--11
/// |  |
/// 9--10
///
/// top corner(apex):
///
///
/// 4
///
///
/// \endverbatim
///
/// @sa
/// vtkQuadraticEdge vtkBiQuadraticTriangle vtkQuadraticTetra
/// vtkQuadraticHexahedron vtkBiQuadraticQuad vtkQuadraticWedge
///
/// @par Thanks:
/// The shape functions and derivatives could be implemented thanks to
/// the doctoral dissertation: R.S. Browning. A Second-Order 19-Node Pyramid
/// Finite Element Suitable for Lumped Mass Explicit Dynamic methods in
/// Nonlinear Solid Mechanics, University of Alabama at Birmingham.
#[allow(non_camel_case_types)]
pub struct vtkTriQuadraticPyramid(*mut core::ffi::c_void);
impl vtkTriQuadraticPyramid {
    /// Creates a new [vtkTriQuadraticPyramid] wrapped inside `vtkNew`
    #[doc(alias = "vtkTriQuadraticPyramid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTriQuadraticPyramid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTriQuadraticPyramid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTriQuadraticPyramid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTriQuadraticPyramid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTriQuadraticPyramid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTriQuadraticPyramid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTriQuadraticPyramid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTriQuadraticPyramid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTriQuadraticPyramid_create_drop() {
    let obj = vtkTriQuadraticPyramid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTriQuadraticPyramid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents a triangle
///
///
/// vtkTriangle is a concrete implementation of vtkCell to represent a triangle
/// located in 3-space.
#[allow(non_camel_case_types)]
pub struct vtkTriangle(*mut core::ffi::c_void);
impl vtkTriangle {
    /// Creates a new [vtkTriangle] wrapped inside `vtkNew`
    #[doc(alias = "vtkTriangle")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTriangle_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTriangle_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTriangle_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTriangle_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTriangle {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTriangle {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTriangle_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTriangle_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTriangle_create_drop() {
    let obj = vtkTriangle::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTriangle(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents a triangle strip
///
///
/// vtkTriangleStrip is a concrete implementation of vtkCell to represent a 2D
/// triangle strip. A triangle strip is a compact representation of triangles
/// connected edge to edge in strip fashion. The connectivity of a triangle
/// strip is three points defining an initial triangle, then for each
/// additional triangle, a single point that, combined with the previous two
/// points, defines the next triangle.
#[allow(non_camel_case_types)]
pub struct vtkTriangleStrip(*mut core::ffi::c_void);
impl vtkTriangleStrip {
    /// Creates a new [vtkTriangleStrip] wrapped inside `vtkNew`
    #[doc(alias = "vtkTriangleStrip")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTriangleStrip_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTriangleStrip_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTriangleStrip_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTriangleStrip_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTriangleStrip {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTriangleStrip {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTriangleStrip_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTriangleStrip_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTriangleStrip_create_drop() {
    let obj = vtkTriangleStrip::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTriangleStrip(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// An undirected graph.
///
///
///
/// vtkUndirectedGraph is a collection of vertices along with a collection of
/// undirected edges (they connect two vertices in no particular order).
/// ShallowCopy(), DeepCopy(), CheckedShallowCopy(), CheckedDeepCopy()
/// accept instances of vtkUndirectedGraph and vtkMutableUndirectedGraph.
/// GetOutEdges(v, it) and GetInEdges(v, it) return the same list of edges,
/// which is the list of all edges which have a v as an endpoint.
/// GetInDegree(v), GetOutDegree(v) and GetDegree(v) all return the full
/// degree of vertex v.
///
/// vtkUndirectedGraph is read-only. To create an undirected graph,
/// use an instance of vtkMutableUndirectedGraph, then you may set the
/// structure to a vtkUndirectedGraph using ShallowCopy().
///
/// @sa
/// vtkGraph vtkMutableUndirectedGraph
#[allow(non_camel_case_types)]
pub struct vtkUndirectedGraph(*mut core::ffi::c_void);
impl vtkUndirectedGraph {
    /// Creates a new [vtkUndirectedGraph] wrapped inside `vtkNew`
    #[doc(alias = "vtkUndirectedGraph")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUndirectedGraph_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUndirectedGraph_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUndirectedGraph_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUndirectedGraph_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUndirectedGraph {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUndirectedGraph {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUndirectedGraph_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUndirectedGraph_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUndirectedGraph_create_drop() {
    let obj = vtkUndirectedGraph::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUndirectedGraph(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// image data with blanking
///
///
/// vtkUniformGrid is a subclass of vtkImageData. In addition to all
/// the image data functionality, it supports blanking.
#[allow(non_camel_case_types)]
pub struct vtkUniformGrid(*mut core::ffi::c_void);
impl vtkUniformGrid {
    /// Creates a new [vtkUniformGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkUniformGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUniformGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUniformGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUniformGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUniformGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformGrid_create_drop() {
    let obj = vtkUniformGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUniformGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a concrete implementation of vtkCompositeDataSet
///
///
/// vtkUniformGridAMR is an AMR (hierarchical) composite dataset that holds vtkUniformGrids.
///
/// @sa
/// vtkUniformGridAMRDataIterator
#[allow(non_camel_case_types)]
pub struct vtkUniformGridAMR(*mut core::ffi::c_void);
impl vtkUniformGridAMR {
    /// Creates a new [vtkUniformGridAMR] wrapped inside `vtkNew`
    #[doc(alias = "vtkUniformGridAMR")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformGridAMR_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUniformGridAMR_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUniformGridAMR_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUniformGridAMR_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUniformGridAMR {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformGridAMR {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformGridAMR_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformGridAMR_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformGridAMR_create_drop() {
    let obj = vtkUniformGridAMR::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUniformGridAMR(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// subclass of vtkCompositeDataIterator
///
/// with API to get current level and dataset index.
#[allow(non_camel_case_types)]
pub struct vtkUniformGridAMRDataIterator(*mut core::ffi::c_void);
impl vtkUniformGridAMRDataIterator {
    /// Creates a new [vtkUniformGridAMRDataIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkUniformGridAMRDataIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformGridAMRDataIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUniformGridAMRDataIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUniformGridAMRDataIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUniformGridAMRDataIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUniformGridAMRDataIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformGridAMRDataIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformGridAMRDataIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformGridAMRDataIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformGridAMRDataIterator_create_drop() {
    let obj = vtkUniformGridAMRDataIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUniformGridAMRDataIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A specifalized type of vtkHyperTreeGrid for the case
///
/// when root cells have uniform sizes in each direction
///
/// @sa
/// vtkHyperTree vtkHyperTreeGrid vtkRectilinearGrid
///
/// @par Thanks:
/// This class was written by Philippe Pebay, NexGen Analytics 2017
/// Modified to introduce Scales by Jacques-Bernard Lekien, CEA 2018.
/// This work was supported by Commissariat a l'Energie Atomique
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkUniformHyperTreeGrid(*mut core::ffi::c_void);
impl vtkUniformHyperTreeGrid {
    /// Creates a new [vtkUniformHyperTreeGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkUniformHyperTreeGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformHyperTreeGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUniformHyperTreeGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUniformHyperTreeGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUniformHyperTreeGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUniformHyperTreeGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformHyperTreeGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformHyperTreeGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformHyperTreeGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformHyperTreeGrid_create_drop() {
    let obj = vtkUniformHyperTreeGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUniformHyperTreeGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dataset represents arbitrary combinations of
///
/// all possible cell types
///
/// vtkUnstructuredGrid is a data object that is a concrete implementation of
/// vtkDataSet. vtkUnstructuredGrid represents any combinations of any cell
/// types. This includes 0D (e.g., points), 1D (e.g., lines, polylines), 2D
/// (e.g., triangles, polygons), and 3D (e.g., hexahedron, tetrahedron,
/// polyhedron, etc.). vtkUnstructuredGrid provides random access to cells, as
/// well as topological information (such as lists of cells using each point).
#[allow(non_camel_case_types)]
pub struct vtkUnstructuredGrid(*mut core::ffi::c_void);
impl vtkUnstructuredGrid {
    /// Creates a new [vtkUnstructuredGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnstructuredGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnstructuredGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnstructuredGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnstructuredGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnstructuredGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnstructuredGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnstructuredGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnstructuredGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnstructuredGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnstructuredGrid_create_drop() {
    let obj = vtkUnstructuredGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnstructuredGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Implementation of vtkCellIterator
///
/// specialized for vtkUnstructuredGrid.
#[allow(non_camel_case_types)]
pub struct vtkUnstructuredGridCellIterator(*mut core::ffi::c_void);
impl vtkUnstructuredGridCellIterator {
    /// Creates a new [vtkUnstructuredGridCellIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnstructuredGridCellIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnstructuredGridCellIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnstructuredGridCellIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnstructuredGridCellIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnstructuredGridCellIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnstructuredGridCellIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnstructuredGridCellIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnstructuredGridCellIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnstructuredGridCellIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnstructuredGridCellIterator_create_drop() {
    let obj = vtkUnstructuredGridCellIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnstructuredGridCellIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents a 3D point
///
///
/// vtkVertex is a concrete implementation of vtkCell to represent a 3D point.
#[allow(non_camel_case_types)]
pub struct vtkVertex(*mut core::ffi::c_void);
impl vtkVertex {
    /// Creates a new [vtkVertex] wrapped inside `vtkNew`
    #[doc(alias = "vtkVertex")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVertex_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkVertex_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkVertex_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkVertex_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkVertex {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVertex {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVertex_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVertex_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVertex_create_drop() {
    let obj = vtkVertex::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkVertex(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Iterates all vertices in a graph.
///
///
///
/// vtkVertexListIterator iterates through all vertices in a graph.
/// Create an instance of this and call graph->GetVertices(it) to initialize
/// this iterator. You may alternately call SetGraph() to initialize the
/// iterator.
///
/// @sa
/// vtkGraph
#[allow(non_camel_case_types)]
pub struct vtkVertexListIterator(*mut core::ffi::c_void);
impl vtkVertexListIterator {
    /// Creates a new [vtkVertexListIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkVertexListIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVertexListIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkVertexListIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkVertexListIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkVertexListIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkVertexListIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVertexListIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVertexListIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVertexListIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVertexListIterator_create_drop() {
    let obj = vtkVertexListIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkVertexListIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a cell that represents a 3D orthogonal parallelepiped
///
///
/// vtkVoxel is a concrete implementation of vtkCell to represent a 3D
/// orthogonal parallelepiped. Unlike vtkHexahedron, vtkVoxel has interior
/// angles of 90 degrees, and sides are parallel to coordinate axes. This
/// results in large increases in computational performance.
///
/// @sa
/// vtkConvexPointSet vtkHexahedron vtkPyramid vtkTetra vtkWedge
#[allow(non_camel_case_types)]
pub struct vtkVoxel(*mut core::ffi::c_void);
impl vtkVoxel {
    /// Creates a new [vtkVoxel] wrapped inside `vtkNew`
    #[doc(alias = "vtkVoxel")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVoxel_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkVoxel_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkVoxel_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkVoxel_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkVoxel {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVoxel {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVoxel_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVoxel_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVoxel_create_drop() {
    let obj = vtkVoxel::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkVoxel(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a 3D cell that represents a linear wedge
///
///
/// vtkWedge is a concrete implementation of vtkCell to represent a linear 3D
/// wedge. A wedge consists of two triangular and three quadrilateral faces
/// and is defined by the six points (0-5). vtkWedge uses the standard
/// isoparametric shape functions for a linear wedge. The wedge is defined
/// by the six points (0-5) where (0,1,2) is the base of the wedge which,
/// using the right hand rule, forms a triangle whose normal points outward
/// (away from the triangular face (3,4,5)).
///
/// @sa
/// vtkConvexPointSet vtkHexahedron vtkPyramid vtkTetra vtkVoxel
#[allow(non_camel_case_types)]
pub struct vtkWedge(*mut core::ffi::c_void);
impl vtkWedge {
    /// Creates a new [vtkWedge] wrapped inside `vtkNew`
    #[doc(alias = "vtkWedge")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkWedge_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkWedge_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkWedge_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkWedge_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkWedge {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkWedge {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkWedge_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkWedge_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkWedge_create_drop() {
    let obj = vtkWedge::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkWedge(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Represents an XML element and those nested inside.
///
///
/// vtkXMLDataElement is used by vtkXMLDataParser to represent an XML
/// element.  It provides methods to access the element's attributes
/// and nested elements in a convenient manner.  This allows easy
/// traversal of an input XML file by vtkXMLReader and its subclasses.
///
/// @sa
/// vtkXMLDataParser
#[allow(non_camel_case_types)]
pub struct vtkXMLDataElement(*mut core::ffi::c_void);
impl vtkXMLDataElement {
    /// Creates a new [vtkXMLDataElement] wrapped inside `vtkNew`
    #[doc(alias = "vtkXMLDataElement")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkXMLDataElement_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkXMLDataElement_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkXMLDataElement_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkXMLDataElement_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkXMLDataElement {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkXMLDataElement {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkXMLDataElement_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkXMLDataElement_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkXMLDataElement_create_drop() {
    let obj = vtkXMLDataElement::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkXMLDataElement(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
