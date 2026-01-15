/// a seqin an animation.
///
///
/// vtkAnimationCue and vtkAnimationScene provide the framework to support
/// animations in VTK. vtkAnimationCue represents an entity that changes/
/// animates with time, while vtkAnimationScene represents scene or setup
/// for the animation, which consists on individual cues or other scenes.
///
/// A cue has three states: UNINITIALIZED, ACTIVE and INACTIVE.
/// UNINITIALIZED represents an point in time before the start time of the cue.
/// The cue is in ACTIVE state at a point in time between start time and end time
/// for the cue. While, beyond the end time, it is in INACTIVE state.
/// When the cue enters the ACTIVE state, StartAnimationCueEvent is fired. This
/// event may be handled to initialize the entity to be animated.
/// When the cue leaves the ACTIVE state, EndAnimationCueEvent is fired, which
/// can be handled to cleanup after having run the animation.
/// For every request to render during the ACTIVE state, AnimationCueTickEvent is
/// fired, which must be handled to perform the actual animation.
/// @sa
/// vtkAnimationScene
#[allow(non_camel_case_types)]
pub struct vtkAnimationCue(*mut core::ffi::c_void);
impl vtkAnimationCue {
    /// Creates a new [vtkAnimationCue] wrapped inside `vtkNew`
    #[doc(alias = "vtkAnimationCue")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAnimationCue_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAnimationCue_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAnimationCue_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAnimationCue_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAnimationCue {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAnimationCue {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAnimationCue_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAnimationCue_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAnimationCue_create_drop() {
    let obj = vtkAnimationCue::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAnimationCue(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Writes an archive
///
///
/// vtkArchiver is a base class for constructing an archive. The default
/// implementation constructs a directory at the location of the ArchiveName
/// and populates it with files and directories as requested by Insert().
/// Classes that derive from vtkArchiver can customize the output using such
/// features as compression, in-memory serialization and third-party archival
/// tools.
#[allow(non_camel_case_types)]
pub struct vtkArchiver(*mut core::ffi::c_void);
impl vtkArchiver {
    /// Creates a new [vtkArchiver] wrapped inside `vtkNew`
    #[doc(alias = "vtkArchiver")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkArchiver_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkArchiver_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkArchiver_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkArchiver_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkArchiver {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkArchiver {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkArchiver_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkArchiver_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkArchiver_create_drop() {
    let obj = vtkArchiver::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkArchiver(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of bits
///
///
/// vtkBitArray is an array of bits (0/1 data value). The array is packed
/// so that each byte stores eight bits. vtkBitArray provides methods
/// for insertion and retrieval of bits, and will automatically resize
/// itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkBitArray(*mut core::ffi::c_void);
impl vtkBitArray {
    /// Creates a new [vtkBitArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkBitArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBitArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBitArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBitArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBitArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBitArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBitArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBitArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBitArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBitArray_create_drop() {
    let obj = vtkBitArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBitArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Iterator for vtkBitArray.
///
/// This iterator iterates over a vtkBitArray. It uses the double interface
/// to get/set bit values.
#[allow(non_camel_case_types)]
pub struct vtkBitArrayIterator(*mut core::ffi::c_void);
impl vtkBitArrayIterator {
    /// Creates a new [vtkBitArrayIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkBitArrayIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBitArrayIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBitArrayIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBitArrayIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBitArrayIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBitArrayIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBitArrayIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBitArrayIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBitArrayIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBitArrayIterator_create_drop() {
    let obj = vtkBitArrayIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBitArrayIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Gaussian sequence of pseudo random numbers implemented with the Box-Mueller transform
///
///
/// vtkGaussianRandomSequence is a sequence of pseudo random numbers
/// distributed according to the Gaussian/normal distribution (mean=0 and
/// standard deviation=1).
///
/// It based is calculation from a uniformly distributed pseudo random sequence.
/// The initial sequence is a vtkMinimalStandardRandomSequence.
#[allow(non_camel_case_types)]
pub struct vtkBoxMuellerRandomSequence(*mut core::ffi::c_void);
impl vtkBoxMuellerRandomSequence {
    /// Creates a new [vtkBoxMuellerRandomSequence] wrapped inside `vtkNew`
    #[doc(alias = "vtkBoxMuellerRandomSequence")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBoxMuellerRandomSequence_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBoxMuellerRandomSequence_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBoxMuellerRandomSequence_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBoxMuellerRandomSequence_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkBoxMuellerRandomSequence {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBoxMuellerRandomSequence {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBoxMuellerRandomSequence_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBoxMuellerRandomSequence_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBoxMuellerRandomSequence_create_drop() {
    let obj = vtkBoxMuellerRandomSequence::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBoxMuellerRandomSequence(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// perform machine dependent byte swapping
///
///
/// vtkByteSwap is used by other classes to perform machine dependent byte
/// swapping. Byte swapping is often used when reading or writing binary
/// files.
#[allow(non_camel_case_types)]
pub struct vtkByteSwap(*mut core::ffi::c_void);
impl vtkByteSwap {
    /// Creates a new [vtkByteSwap] wrapped inside `vtkNew`
    #[doc(alias = "vtkByteSwap")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkByteSwap_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkByteSwap_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkByteSwap_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkByteSwap_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkByteSwap {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkByteSwap {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkByteSwap_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkByteSwap_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkByteSwap_create_drop() {
    let obj = vtkByteSwap::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkByteSwap(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// supports function callbacks
///
///
/// Use vtkCallbackCommand for generic function callbacks. That is, this class
/// can be used when you wish to execute a function (of the signature
/// described below) using the Command/Observer design pattern in VTK.
/// The callback function should have the form
/// <pre>
/// void func(vtkObject*, unsigned long eid, void* clientdata, void *calldata)
/// </pre>
/// where the parameter vtkObject* is the object invoking the event; eid is
/// the event id (see vtkCommand.h); clientdata is special data that should
/// is associated with this instance of vtkCallbackCommand; and calldata is
/// data that the vtkObject::InvokeEvent() may send with the callback. For
/// example, the invocation of the ProgressEvent sends along the progress
/// value as calldata.
///
///
/// @sa
/// vtkCommand vtkOldStyleCallbackCommand
#[allow(non_camel_case_types)]
pub struct vtkCallbackCommand(*mut core::ffi::c_void);
impl vtkCallbackCommand {
    /// Creates a new [vtkCallbackCommand] wrapped inside `vtkNew`
    #[doc(alias = "vtkCallbackCommand")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCallbackCommand_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCallbackCommand_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCallbackCommand_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCallbackCommand_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCallbackCommand {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCallbackCommand {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCallbackCommand_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCallbackCommand_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCallbackCommand_create_drop() {
    let obj = vtkCallbackCommand::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCallbackCommand(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of char
///
///
/// vtkCharArray is an array of values of type char.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// @warning
/// This class should be avoided in favor of either
/// vtkSignedCharArray or vtkUnsignedCharArray. On some systems
/// the underlying data will be stored as unsigned chars and others
/// it will be stored as signed chars. Additionally, saving this
/// array out and then reading it back in it could be transformed to
/// a vtkSignedCharArray or vtkUnsignedCharArray and if that happens
/// the result of a vtkCharArray::SafeDownCast() of that pointer will be
/// a null pointer.
///
/// @sa
/// vtkSignedCharArray vtkUnsignedCharArray
#[allow(non_camel_case_types)]
pub struct vtkCharArray(*mut core::ffi::c_void);
impl vtkCharArray {
    /// Creates a new [vtkCharArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkCharArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCharArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCharArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCharArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCharArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCharArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCharArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCharArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCharArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCharArray_create_drop() {
    let obj = vtkCharArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCharArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// create and manipulate ordered lists of objects
///
///
/// vtkCollection is a general object for creating and manipulating lists
/// of objects. The lists are ordered and allow duplicate entries.
/// vtkCollection also serves as a base class for lists of specific types
/// of objects.
///
/// @sa
/// vtkActorCollection vtkAssemblyPaths vtkDataSetCollection
/// vtkImplicitFunctionCollection vtkLightCollection vtkPolyDataCollection
/// vtkRenderWindowCollection vtkRendererCollection
/// vtkStructuredPointsCollection vtkTransformCollection vtkVolumeCollection
#[allow(non_camel_case_types)]
pub struct vtkCollection(*mut core::ffi::c_void);
impl vtkCollection {
    /// Creates a new [vtkCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCollection_create_drop() {
    let obj = vtkCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// iterator through a vtkCollection.
///
///
/// vtkCollectionIterator provides an alternative way to traverse
/// through the objects in a vtkCollection.  Unlike the collection's
/// built in interface, this allows multiple iterators to
/// simultaneously traverse the collection.  If items are removed from
/// the collection, only the iterators currently pointing to those
/// items are invalidated.  Other iterators will still continue to
/// function normally.
#[allow(non_camel_case_types)]
pub struct vtkCollectionIterator(*mut core::ffi::c_void);
impl vtkCollectionIterator {
    /// Creates a new [vtkCollectionIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkCollectionIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCollectionIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCollectionIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCollectionIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCollectionIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCollectionIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCollectionIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCollectionIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCollectionIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCollectionIterator_create_drop() {
    let obj = vtkCollectionIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCollectionIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Critical section locking class
///
///
/// vtkCriticalSection allows the locking of variables which are accessed
/// through different threads.  This header file also defines
/// vtkSimpleCriticalSection which is not a subclass of vtkObject.
/// The API is identical to that of vtkMutexLock, and the behavior is
/// identical as well, except on Windows 9x/NT platforms. The only difference
/// on these platforms is that vtkMutexLock is more flexible, in that
/// it works across processes as well as across threads, but also costs
/// more, in that it evokes a 600-cycle x86 ring transition. The
/// vtkCriticalSection provides a higher-performance equivalent (on
/// Windows) but won't work across processes. Since it is unclear how,
/// in vtk, an object at the vtk level can be shared across processes
/// in the first place, one should use vtkCriticalSection unless one has
/// a very good reason to use vtkMutexLock. If higher-performance equivalents
/// for non-Windows platforms (Irix, SunOS, etc) are discovered, they
/// should replace the implementations in this class
#[allow(non_camel_case_types)]
pub struct vtkCriticalSection(*mut core::ffi::c_void);
impl vtkCriticalSection {
    /// Creates a new [vtkCriticalSection] wrapped inside `vtkNew`
    #[doc(alias = "vtkCriticalSection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCriticalSection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCriticalSection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCriticalSection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCriticalSection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCriticalSection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCriticalSection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCriticalSection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCriticalSection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCriticalSection_create_drop() {
    let obj = vtkCriticalSection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCriticalSection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain an ordered list of dataarray objects
///
///
/// vtkDataArrayCollection is an object that creates and manipulates lists of
/// datasets. See also vtkCollection and subclasses.
#[allow(non_camel_case_types)]
pub struct vtkDataArrayCollection(*mut core::ffi::c_void);
impl vtkDataArrayCollection {
    /// Creates a new [vtkDataArrayCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataArrayCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataArrayCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataArrayCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataArrayCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataArrayCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataArrayCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataArrayCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataArrayCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataArrayCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataArrayCollection_create_drop() {
    let obj = vtkDataArrayCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataArrayCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// iterator through a vtkDataArrayCollection.
///
///
/// vtkDataArrayCollectionIterator provides an implementation of
/// vtkCollectionIterator which allows the items to be retrieved with
/// the proper subclass pointer type for vtkDataArrayCollection.
#[allow(non_camel_case_types)]
pub struct vtkDataArrayCollectionIterator(*mut core::ffi::c_void);
impl vtkDataArrayCollectionIterator {
    /// Creates a new [vtkDataArrayCollectionIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataArrayCollectionIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataArrayCollectionIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataArrayCollectionIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataArrayCollectionIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataArrayCollectionIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataArrayCollectionIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataArrayCollectionIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataArrayCollectionIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataArrayCollectionIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataArrayCollectionIterator_create_drop() {
    let obj = vtkDataArrayCollectionIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataArrayCollectionIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Store on/off settings for data arrays, etc.
///
///
/// vtkDataArraySelection is intended to be used by algorithms that want to
/// expose a API that allow the user to enable/disable a collection of entities,
/// such as arrays. Readers, for example, can use vtkDataArraySelection to let
/// the user choose which array to read from the file.
///
/// Originally intended for selecting data arrays (hence the name), this class
/// can be used for letting users choose other items too, for example,
/// vtkIOSSReader uses vtkDataArraySelection to let users choose
/// which blocks to read.
///
/// Unlike most other vtkObject subclasses, vtkDataArraySelection has public API
/// that need not modify the MTime for the object. These M-Time non-modifying
/// methods are typically intended for use within the algorithm or reader to
/// populate the vtkDataArraySelection instance with available array names and
/// their default values.
#[allow(non_camel_case_types)]
pub struct vtkDataArraySelection(*mut core::ffi::c_void);
impl vtkDataArraySelection {
    /// Creates a new [vtkDataArraySelection] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataArraySelection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataArraySelection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataArraySelection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataArraySelection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataArraySelection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataArraySelection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataArraySelection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataArraySelection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataArraySelection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataArraySelection_create_drop() {
    let obj = vtkDataArraySelection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataArraySelection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// identify memory leaks at program termination
///
/// vtkDebugLeaks is used to report memory leaks at the exit of the program. It
/// uses vtkObjectBase::InitializeObjectBase() (called via vtkObjectFactory
/// macros) to intercept the construction of all VTK objects. It uses the
/// UnRegisterInternal method of vtkObjectBase to intercept the destruction of
/// all objects.
///
/// If not using the vtkObjectFactory macros to implement New(), be sure to call
/// vtkObjectBase::InitializeObjectBase() explicitly on the constructed
/// instance. The rule of thumb is that wherever "new [some vtkObjectBase
/// subclass]" is called, vtkObjectBase::InitializeObjectBase() must be called
/// as well.
///
/// There are exceptions to this:
///
/// - vtkCommand subclasses traditionally do not fully participate in
/// vtkDebugLeaks registration, likely because they typically do not use
/// vtkTypeMacro to configure GetClassName. InitializeObjectBase should not be
/// called on vtkCommand subclasses, and all such classes will be automatically
/// registered with vtkDebugLeaks as "vtkCommand or subclass".
///
/// - vtkInformationKey subclasses are not reference counted. They are allocated
/// statically and registered automatically with a singleton "manager" instance.
/// The manager ensures that all keys are cleaned up before exiting, and
/// registration/deregistration with vtkDebugLeaks is bypassed.
///
/// A table of object name to number of instances is kept. At the exit of the
/// program if there are still VTK objects around it will print them out. To
/// enable this class add the flag -DVTK_DEBUG_LEAKS to the compile line, and
/// rebuild vtkObject and vtkObjectFactory.
#[allow(non_camel_case_types)]
pub struct vtkDebugLeaks(*mut core::ffi::c_void);
impl vtkDebugLeaks {
    /// Creates a new [vtkDebugLeaks] wrapped inside `vtkNew`
    #[doc(alias = "vtkDebugLeaks")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDebugLeaks_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDebugLeaks_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDebugLeaks_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDebugLeaks_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDebugLeaks {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDebugLeaks {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDebugLeaks_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDebugLeaks_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDebugLeaks_create_drop() {
    let obj = vtkDebugLeaks::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDebugLeaks(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of double
///
///
/// vtkDoubleArray is an array of values of type double.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkDoubleArray(*mut core::ffi::c_void);
impl vtkDoubleArray {
    /// Creates a new [vtkDoubleArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkDoubleArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDoubleArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDoubleArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDoubleArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDoubleArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDoubleArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDoubleArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDoubleArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDoubleArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDoubleArray_create_drop() {
    let obj = vtkDoubleArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDoubleArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// class interface to system dynamic libraries
///
///
/// vtkDynamicLoader provides a portable interface to loading dynamic
/// libraries into a process.
/// @sa
/// A more portable and lightweight solution is kwsys::DynamicLoader
#[allow(non_camel_case_types)]
pub struct vtkDynamicLoader(*mut core::ffi::c_void);
impl vtkDynamicLoader {
    /// Creates a new [vtkDynamicLoader] wrapped inside `vtkNew`
    #[doc(alias = "vtkDynamicLoader")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDynamicLoader_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDynamicLoader_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDynamicLoader_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDynamicLoader_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDynamicLoader {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDynamicLoader {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDynamicLoader_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDynamicLoader_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDynamicLoader_create_drop() {
    let obj = vtkDynamicLoader::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDynamicLoader(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkEventDataDevice3D(*mut core::ffi::c_void);
impl vtkEventDataDevice3D {
    /// Creates a new [vtkEventDataDevice3D] wrapped inside `vtkNew`
    #[doc(alias = "vtkEventDataDevice3D")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEventDataDevice3D_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkEventDataDevice3D_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkEventDataDevice3D_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkEventDataDevice3D_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkEventDataDevice3D {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEventDataDevice3D {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEventDataDevice3D_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEventDataDevice3D_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEventDataDevice3D_create_drop() {
    let obj = vtkEventDataDevice3D::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkEventDataDevice3D(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkEventDataForDevice(*mut core::ffi::c_void);
impl vtkEventDataForDevice {
    /// Creates a new [vtkEventDataForDevice] wrapped inside `vtkNew`
    #[doc(alias = "vtkEventDataForDevice")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEventDataForDevice_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkEventDataForDevice_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkEventDataForDevice_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkEventDataForDevice_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkEventDataForDevice {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEventDataForDevice {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEventDataForDevice_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEventDataForDevice_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEventDataForDevice_create_drop() {
    let obj = vtkEventDataForDevice::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkEventDataForDevice(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a simple event forwarder command
///
///
/// Use vtkEventForwarderCommand to forward an event to a new object.
/// This command will intercept the event, and use InvokeEvent
/// on a 'target' as if that object was the one that invoked the event instead
/// of the object this command was attached to using AddObserver.
///
/// @sa
/// vtkCommand
#[allow(non_camel_case_types)]
pub struct vtkEventForwarderCommand(*mut core::ffi::c_void);
impl vtkEventForwarderCommand {
    /// Creates a new [vtkEventForwarderCommand] wrapped inside `vtkNew`
    #[doc(alias = "vtkEventForwarderCommand")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEventForwarderCommand_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkEventForwarderCommand_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkEventForwarderCommand_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkEventForwarderCommand_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkEventForwarderCommand {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEventForwarderCommand {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEventForwarderCommand_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEventForwarderCommand_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEventForwarderCommand_create_drop() {
    let obj = vtkEventForwarderCommand::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkEventForwarderCommand(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// File Specific output window class
///
///
/// Writes debug/warning/error output to a log file instead of the console.
/// To use this class, instantiate it and then call SetInstance(this).
#[allow(non_camel_case_types)]
pub struct vtkFileOutputWindow(*mut core::ffi::c_void);
impl vtkFileOutputWindow {
    /// Creates a new [vtkFileOutputWindow] wrapped inside `vtkNew`
    #[doc(alias = "vtkFileOutputWindow")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFileOutputWindow_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkFileOutputWindow_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkFileOutputWindow_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkFileOutputWindow_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkFileOutputWindow {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFileOutputWindow {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFileOutputWindow_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFileOutputWindow_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFileOutputWindow_create_drop() {
    let obj = vtkFileOutputWindow::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkFileOutputWindow(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of float
///
///
/// vtkFloatArray is an array of values of type float.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkFloatArray(*mut core::ffi::c_void);
impl vtkFloatArray {
    /// Creates a new [vtkFloatArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkFloatArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFloatArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkFloatArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkFloatArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkFloatArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkFloatArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFloatArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFloatArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFloatArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFloatArray_create_drop() {
    let obj = vtkFloatArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkFloatArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Detect and break reference loops
///
///
/// vtkGarbageCollector is used by VTK classes that may be involved in
/// reference counting loops (such as Algorithm <-> Executive).  It
/// detects strongly connected components of the reference graph that
/// have been leaked deletes them.  The garbage collector uses the
/// ReportReferences method to search the reference graph and construct
/// a net reference count for each connected component.  If the net
/// reference count is zero the entire set of objects is deleted.
/// Deleting each component may leak other components, which are then
/// collected recursively.
///
/// To enable garbage collection for a class, add these members:
///
/// \code
///
/// public:
/// bool UsesGarbageCollector() const override { return true; }
///
/// protected:
///
/// void ReportReferences(vtkGarbageCollector* collector) override
/// {
/// // Report references held by this object that may be in a loop.
/// this->Superclass::ReportReferences(collector);
/// vtkGarbageCollectorReport(collector, this->OtherObject, "Other Object");
/// }
/// \endcode
///
/// The implementations should be in the .cxx file in practice.
/// It is important that the reference be reported using the real
/// pointer or smart pointer instance that holds the reference.  When
/// collecting the garbage collector will actually set this pointer to
/// nullptr.  The destructor of the class should be written to deal with
/// this.  It is also expected that an invariant is maintained for any
/// reference that is reported.  The variable holding the reference
/// must always either be nullptr or refer to a fully constructed valid
/// object.  Therefore code like "this->Object->UnRegister(this)" must
/// be avoided if "this->Object" is a reported reference because it
/// is possible that the object is deleted before UnRegister returns
/// but then "this->Object" will be left as a dangling pointer.  Instead
/// use code like
///
/// \code
/// vtkObjectBase* obj = this->Object;
/// this->Object = 0;
/// obj->UnRegister(this);
/// \endcode
///
/// so that the reported reference maintains the invariant.
///
/// If subclassing from a class that already supports garbage
/// collection, one need only provide the ReportReferences method.
#[allow(non_camel_case_types)]
pub struct vtkGarbageCollector(*mut core::ffi::c_void);
impl vtkGarbageCollector {
    /// Creates a new [vtkGarbageCollector] wrapped inside `vtkNew`
    #[doc(alias = "vtkGarbageCollector")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGarbageCollector_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGarbageCollector_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGarbageCollector_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGarbageCollector_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGarbageCollector {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGarbageCollector_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGarbageCollector_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGarbageCollector_create_drop() {
    let obj = vtkGarbageCollector::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGarbageCollector(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// list of point or cell ids
///
///
/// vtkIdList is used to represent and pass data id's between
/// objects. vtkIdList may represent any type of integer id, but
/// usually represents point and cell ids.
#[allow(non_camel_case_types)]
pub struct vtkIdList(*mut core::ffi::c_void);
impl vtkIdList {
    /// Creates a new [vtkIdList] wrapped inside `vtkNew`
    #[doc(alias = "vtkIdList")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIdList_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkIdList_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkIdList_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkIdList_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkIdList {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIdList {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIdList_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIdList_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIdList_create_drop() {
    let obj = vtkIdList::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkIdList(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain an ordered list of IdList objects
///
///
/// vtkIdListCollection is an object that creates and manipulates lists of
/// IdLists. See also vtkCollection and subclasses.
#[allow(non_camel_case_types)]
pub struct vtkIdListCollection(*mut core::ffi::c_void);
impl vtkIdListCollection {
    /// Creates a new [vtkIdListCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkIdListCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIdListCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkIdListCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkIdListCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkIdListCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkIdListCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIdListCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIdListCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIdListCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIdListCollection_create_drop() {
    let obj = vtkIdListCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkIdListCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of vtkIdType
///
///
/// vtkIdTypeArray is an array of values of type vtkIdType.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkIdTypeArray(*mut core::ffi::c_void);
impl vtkIdTypeArray {
    /// Creates a new [vtkIdTypeArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkIdTypeArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIdTypeArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkIdTypeArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkIdTypeArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkIdTypeArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkIdTypeArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIdTypeArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIdTypeArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIdTypeArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIdTypeArray_create_drop() {
    let obj = vtkIdTypeArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkIdTypeArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Store vtkAlgorithm input/output information.
///
///
/// vtkInformation represents information and/or data for one input or
/// one output of a vtkAlgorithm.  It maps from keys to values of
/// several data types.  Instances of this class are collected in
/// vtkInformationVector instances and passed to
/// vtkAlgorithm::ProcessRequest calls.  The information and
/// data referenced by the instance on a particular input or output
/// define the request made to the vtkAlgorithm instance.
#[allow(non_camel_case_types)]
pub struct vtkInformation(*mut core::ffi::c_void);
impl vtkInformation {
    /// Creates a new [vtkInformation] wrapped inside `vtkNew`
    #[doc(alias = "vtkInformation")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInformation_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkInformation_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkInformation_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkInformation_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkInformation {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInformation {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInformation_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInformation_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInformation_create_drop() {
    let obj = vtkInformation::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkInformation(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Iterates over keys of an information object
///
///
/// vtkInformationIterator can be used to iterate over the keys of an
/// information object. The corresponding values can then be directly
/// obtained from the information object using the keys.
///
/// @sa
/// vtkInformation vtkInformationKey
#[allow(non_camel_case_types)]
pub struct vtkInformationIterator(*mut core::ffi::c_void);
impl vtkInformationIterator {
    /// Creates a new [vtkInformationIterator] wrapped inside `vtkNew`
    #[doc(alias = "vtkInformationIterator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInformationIterator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkInformationIterator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkInformationIterator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkInformationIterator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkInformationIterator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInformationIterator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInformationIterator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInformationIterator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInformationIterator_create_drop() {
    let obj = vtkInformationIterator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkInformationIterator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Find vtkInformationKeys from name and
///
/// location strings.
#[allow(non_camel_case_types)]
pub struct vtkInformationKeyLookup(*mut core::ffi::c_void);
impl vtkInformationKeyLookup {
    /// Creates a new [vtkInformationKeyLookup] wrapped inside `vtkNew`
    #[doc(alias = "vtkInformationKeyLookup")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInformationKeyLookup_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkInformationKeyLookup_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkInformationKeyLookup_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkInformationKeyLookup_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkInformationKeyLookup {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInformationKeyLookup {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInformationKeyLookup_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInformationKeyLookup_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInformationKeyLookup_create_drop() {
    let obj = vtkInformationKeyLookup::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkInformationKeyLookup(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Store zero or more vtkInformation instances.
///
///
///
/// vtkInformationVector stores a vector of zero or more vtkInformation
/// objects corresponding to the input or output information for a
/// vtkAlgorithm.  An instance of this class is passed to
/// vtkAlgorithm::ProcessRequest calls.
#[allow(non_camel_case_types)]
pub struct vtkInformationVector(*mut core::ffi::c_void);
impl vtkInformationVector {
    /// Creates a new [vtkInformationVector] wrapped inside `vtkNew`
    #[doc(alias = "vtkInformationVector")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkInformationVector_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkInformationVector_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkInformationVector_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkInformationVector_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkInformationVector {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkInformationVector {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkInformationVector_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkInformationVector_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkInformationVector_create_drop() {
    let obj = vtkInformationVector::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkInformationVector(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of int
///
///
/// vtkIntArray is an array of values of type int.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the int type, so use
/// of this type directly is discouraged.  If an array of 32 bit integers is
/// needed, prefer vtkTypeInt32Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkIntArray(*mut core::ffi::c_void);
impl vtkIntArray {
    /// Creates a new [vtkIntArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkIntArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIntArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkIntArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkIntArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkIntArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkIntArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIntArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIntArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIntArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIntArray_create_drop() {
    let obj = vtkIntArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkIntArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of long
///
///
/// vtkLongArray is an array of values of type long.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the long type, so use
/// of this type directly is discouraged.  If an array of 32 bit integers is
/// needed, prefer vtkTypeInt32Array to this class.  If an array of 64 bit
/// integers is needed, prefer vtkTypeInt64Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkLongArray(*mut core::ffi::c_void);
impl vtkLongArray {
    /// Creates a new [vtkLongArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkLongArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLongArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLongArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLongArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLongArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLongArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLongArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLongArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLongArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLongArray_create_drop() {
    let obj = vtkLongArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLongArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of long long
///
///
/// vtkLongLongArray is an array of values of type long long.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// This class should not be used directly, as it only exists on systems
/// where the long long type is defined.  If you need a 64 bit integer
/// data array, use vtkTypeInt64Array instead.
#[allow(non_camel_case_types)]
pub struct vtkLongLongArray(*mut core::ffi::c_void);
impl vtkLongLongArray {
    /// Creates a new [vtkLongLongArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkLongLongArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLongLongArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLongLongArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLongLongArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLongLongArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLongLongArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLongLongArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLongLongArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLongLongArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLongLongArray_create_drop() {
    let obj = vtkLongLongArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLongLongArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// map scalar values into colors via a lookup table
///
///
/// vtkLookupTable is an object that is used by mapper objects to map scalar
/// values into RGBA (red-green-blue-alpha) color specification,
/// or RGBA into scalar values. The color table can be created by direct
/// insertion of color values, or by specifying a hue, saturation, value, and
/// alpha range and generating a table.
///
/// A special color for NaN values in the data can be specified via
/// SetNanColor(). In addition, a color for data values below the
/// lookup table range minimum can be specified with
/// SetBelowRangeColor(), and that color will be used for values below
/// the range minimum when UseBelowRangeColor is on.  Likewise, a color
/// for data values above the lookup table range maximum can be
/// specified with SetAboveRangeColor(), and it is used when
/// UseAboveRangeColor is on.
///
/// This class behaves differently depending on how \a IndexedLookup is set.
/// When true, vtkLookupTable enters a mode for representing categorical color maps.
/// By setting \a IndexedLookup to true, you indicate that the annotated
/// values are the only valid values for which entries in the color table
/// should be returned. The colors in the lookup \a Table are assigned
/// to annotated values by taking the modulus of their index in the list
/// of annotations. \a IndexedLookup changes the behavior of \a GetIndex,
/// which in turn changes the way \a MapScalarsThroughTable2 behaves;
/// when \a IndexedLookup is true, \a MapScalarsThroughTable2 will search for
/// scalar values in \a AnnotatedValues and use the resulting index to
/// determine the color. If a scalar value is not present in \a AnnotatedValues,
/// then \a NanColor will be used.
///
/// @warning
/// You need to explicitly call Build() when constructing the LUT by hand.
///
/// @sa
/// vtkLogLookupTable vtkWindowLevelLookupTable
#[allow(non_camel_case_types)]
pub struct vtkLookupTable(*mut core::ffi::c_void);
impl vtkLookupTable {
    /// Creates a new [vtkLookupTable] wrapped inside `vtkNew`
    #[doc(alias = "vtkLookupTable")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLookupTable_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkLookupTable_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkLookupTable_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkLookupTable_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkLookupTable {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLookupTable {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLookupTable_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLookupTable_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLookupTable_create_drop() {
    let obj = vtkLookupTable::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkLookupTable(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// performs common math operations
///
///
/// vtkMath provides methods to perform common math operations. These
/// include providing constants such as Pi; conversion from degrees to
/// radians; vector operations such as dot and cross products and vector
/// norm; matrix determinant for 2x2 and 3x3 matrices; univariate polynomial
/// solvers; and for random number generation (for backward compatibility only).
/// @sa
/// vtkMinimalStandardRandomSequence, vtkBoxMuellerRandomSequence,
/// vtkQuaternion
#[allow(non_camel_case_types)]
pub struct vtkMath(*mut core::ffi::c_void);
impl vtkMath {
    /// Creates a new [vtkMath] wrapped inside `vtkNew`
    #[doc(alias = "vtkMath")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMath_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMath_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMath_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMath_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMath {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMath {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMath_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMath_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMath_create_drop() {
    let obj = vtkMath::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMath(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generator for Mersenne Twister pseudorandom numbers
///
///
/// vtkMersenneTwister is an implementation of the Mersenne Twister pseudorandom
/// number generator. The VTK class is simply a wrapper around an implementation
/// written by M. Matsumoto, T. Nishimura and M. Saito, whose source code can be
/// found at http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/DC/dc.html.
///
/// This implementation of the Mersenne Twister facilitates the generation and
/// query from multiple independent pseudorandom sequences. Independent sequences
/// are identified by a unique vtkMersenneTwister::SequenceId, which is either
/// generated upon request or passed into the initialization method. This id is
/// factored into the initialization of the Mersenne Twister's initial state, so
/// two sequences with the same seed and different sequence ids will produce
/// different results. Once a sequence is initialized with an associated sequence
/// id, this id is used to obtain values from the sequence.
///
/// This class, besides generating random sequences in sequential order, can
/// also populate a double array of specified size with a random sequence. It
/// will do so using one or more threads depending on the number of values
/// requested to generate.
#[allow(non_camel_case_types)]
pub struct vtkMersenneTwister(*mut core::ffi::c_void);
impl vtkMersenneTwister {
    /// Creates a new [vtkMersenneTwister] wrapped inside `vtkNew`
    #[doc(alias = "vtkMersenneTwister")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMersenneTwister_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMersenneTwister_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMersenneTwister_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMersenneTwister_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMersenneTwister {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMersenneTwister {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMersenneTwister_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMersenneTwister_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMersenneTwister_create_drop() {
    let obj = vtkMersenneTwister::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMersenneTwister(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Park and Miller Sequence of pseudo random numbers
///
///
/// vtkMinimalStandardRandomSequence is a sequence of statistically independent
/// pseudo random numbers uniformly distributed between 0.0 and 1.0.
///
/// The sequence is generated by a prime modulus multiplicative linear
/// congruential generator (PMMLCG) or "Lehmer generator" with multiplier 16807
/// and prime modulus 2^(31)-1. The authors calls it
/// "minimal standard random number generator"
///
/// ref: "Random Number Generators: Good Ones are Hard to Find,"
/// by Stephen K. Park and Keith W. Miller in Communications of the ACM,
/// 31, 10 (Oct. 1988) pp. 1192-1201.
/// Code is at page 1195, "Integer version 2"
///
/// Correctness test is described in first column, page 1195:
/// A seed of 1 at step 1 should give a seed of 1043618065 at step 10001.
#[allow(non_camel_case_types)]
pub struct vtkMinimalStandardRandomSequence(*mut core::ffi::c_void);
impl vtkMinimalStandardRandomSequence {
    /// Creates a new [vtkMinimalStandardRandomSequence] wrapped inside `vtkNew`
    #[doc(alias = "vtkMinimalStandardRandomSequence")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMinimalStandardRandomSequence_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMinimalStandardRandomSequence_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMinimalStandardRandomSequence_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMinimalStandardRandomSequence_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMinimalStandardRandomSequence {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMinimalStandardRandomSequence {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMinimalStandardRandomSequence_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkMinimalStandardRandomSequence_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMinimalStandardRandomSequence_create_drop() {
    let obj = vtkMinimalStandardRandomSequence::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMinimalStandardRandomSequence(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A class for performing multithreaded execution
///
///
/// vtkMultithreader is a class that provides support for multithreaded
/// execution using pthreads on POSIX systems, or Win32 threads on
/// Windows.  This class can be used to execute a single
/// method on multiple threads, or to specify a method per thread.
#[allow(non_camel_case_types)]
pub struct vtkMultiThreader(*mut core::ffi::c_void);
impl vtkMultiThreader {
    /// Creates a new [vtkMultiThreader] wrapped inside `vtkNew`
    #[doc(alias = "vtkMultiThreader")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMultiThreader_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMultiThreader_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMultiThreader_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMultiThreader_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMultiThreader {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMultiThreader {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMultiThreader_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMultiThreader_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMultiThreader_create_drop() {
    let obj = vtkMultiThreader::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMultiThreader(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// abstract base class for most VTK objects
///
///
/// vtkObject is the base class for most objects in the visualization
/// toolkit. vtkObject provides methods for tracking modification time,
/// debugging, printing, and event callbacks. Most objects created
/// within the VTK framework should be a subclass of vtkObject or one
/// of its children.  The few exceptions tend to be very small helper
/// classes that usually never get instantiated or situations where
/// multiple inheritance gets in the way.  vtkObject also performs
/// reference counting: objects that are reference counted exist as
/// long as another object uses them. Once the last reference to a
/// reference counted object is removed, the object will spontaneously
/// destruct.
///
/// @warning
/// Note: in VTK objects should always be created with the New() method
/// and deleted with the Delete() method. VTK objects cannot be
/// allocated off the stack (i.e., automatic objects) because the
/// constructor is a protected method.
///
/// @sa
/// vtkCommand vtkTimeStamp
#[allow(non_camel_case_types)]
pub struct vtkObject(*mut core::ffi::c_void);
impl vtkObject {
    /// Creates a new [vtkObject] wrapped inside `vtkNew`
    #[doc(alias = "vtkObject")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkObject_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkObject_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkObject_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkObject_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkObject {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkObject {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkObject_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkObject_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkObject_create_drop() {
    let obj = vtkObject::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkObject(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain a list of object factories
///
///
/// vtkObjectFactoryCollection is an object that creates and manipulates
/// ordered lists of objects of type vtkObjectFactory.
///
/// @sa
/// vtkCollection vtkObjectFactory
#[allow(non_camel_case_types)]
pub struct vtkObjectFactoryCollection(*mut core::ffi::c_void);
impl vtkObjectFactoryCollection {
    /// Creates a new [vtkObjectFactoryCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkObjectFactoryCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkObjectFactoryCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkObjectFactoryCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkObjectFactoryCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkObjectFactoryCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkObjectFactoryCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkObjectFactoryCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkObjectFactoryCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkObjectFactoryCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkObjectFactoryCollection_create_drop() {
    let obj = vtkObjectFactoryCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkObjectFactoryCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// supports legacy function callbacks for VTK
///
///
/// vtkOldStyleCallbackCommand is a callback that supports the legacy callback
/// methods found in VTK. For example, the legacy method
/// vtkProcessObject::SetStartMethod() is actually invoked using the
/// command/observer design pattern of VTK, and the vtkOldStyleCallbackCommand
/// is used to provide the legacy functionality. The callback function should
/// have the form void func(void *clientdata), where clientdata is special data
/// that should is associated with this instance of vtkCallbackCommand.
///
/// @warning
/// This is legacy glue. Please do not use; it will be eventually eliminated.
///
/// @sa
/// vtkCommand vtkCallbackCommand
#[allow(non_camel_case_types)]
pub struct vtkOldStyleCallbackCommand(*mut core::ffi::c_void);
impl vtkOldStyleCallbackCommand {
    /// Creates a new [vtkOldStyleCallbackCommand] wrapped inside `vtkNew`
    #[doc(alias = "vtkOldStyleCallbackCommand")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOldStyleCallbackCommand_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOldStyleCallbackCommand_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOldStyleCallbackCommand_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOldStyleCallbackCommand_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOldStyleCallbackCommand {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOldStyleCallbackCommand {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOldStyleCallbackCommand_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOldStyleCallbackCommand_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOldStyleCallbackCommand_create_drop() {
    let obj = vtkOldStyleCallbackCommand::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOldStyleCallbackCommand(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// base class for writing debug output to a console
///
///
/// This class is used to encapsulate all text output, so that it will work
/// with operating systems that have a stdout and stderr, and ones that
/// do not.  (i.e windows does not).  Sub-classes can be provided which can
/// redirect the output to a window.
#[allow(non_camel_case_types)]
pub struct vtkOutputWindow(*mut core::ffi::c_void);
impl vtkOutputWindow {
    /// Creates a new [vtkOutputWindow] wrapped inside `vtkNew`
    #[doc(alias = "vtkOutputWindow")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOutputWindow_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOutputWindow_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOutputWindow_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOutputWindow_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOutputWindow {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOutputWindow {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOutputWindow_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOutputWindow_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOutputWindow_create_drop() {
    let obj = vtkOutputWindow::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOutputWindow(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// maintain a list of override information objects
///
///
/// vtkOverrideInformationCollection is an object that creates and manipulates
/// lists of objects of type vtkOverrideInformation.
/// @sa
/// vtkCollection
#[allow(non_camel_case_types)]
pub struct vtkOverrideInformationCollection(*mut core::ffi::c_void);
impl vtkOverrideInformationCollection {
    /// Creates a new [vtkOverrideInformationCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkOverrideInformationCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOverrideInformationCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOverrideInformationCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOverrideInformationCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOverrideInformationCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOverrideInformationCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOverrideInformationCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOverrideInformationCollection_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkOverrideInformationCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOverrideInformationCollection_create_drop() {
    let obj = vtkOverrideInformationCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOverrideInformationCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// represent and manipulate 3D points
///
///
/// vtkPoints represents 3D points. The data model for vtkPoints is an
/// array of vx-vy-vz triplets accessible by (point or cell) id.
#[allow(non_camel_case_types)]
pub struct vtkPoints(*mut core::ffi::c_void);
impl vtkPoints {
    /// Creates a new [vtkPoints] wrapped inside `vtkNew`
    #[doc(alias = "vtkPoints")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPoints_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPoints_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPoints_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPoints_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPoints {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPoints {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPoints_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPoints_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPoints_create_drop() {
    let obj = vtkPoints::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPoints(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// represent and manipulate 2D points
///
///
/// vtkPoints2D represents 2D points. The data model for vtkPoints2D is an
/// array of vx-vy doublets accessible by (point or cell) id.
#[allow(non_camel_case_types)]
pub struct vtkPoints2D(*mut core::ffi::c_void);
impl vtkPoints2D {
    /// Creates a new [vtkPoints2D] wrapped inside `vtkNew`
    #[doc(alias = "vtkPoints2D")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPoints2D_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPoints2D_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPoints2D_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPoints2D_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPoints2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPoints2D {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPoints2D_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPoints2D_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPoints2D_create_drop() {
    let obj = vtkPoints2D::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPoints2D(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a list of ids arranged in priority order
///
///
/// vtkPriorityQueue is a general object for creating and manipulating lists
/// of object ids (e.g., point or cell ids). Object ids are sorted according
/// to a user-specified priority, where entries at the top of the queue have
/// the smallest values.
///
/// This implementation provides a feature beyond the usual ability to insert
/// and retrieve (or pop) values from the queue. It is also possible to
/// pop any item in the queue given its id number. This allows you to delete
/// entries in the queue which can useful for reinserting an item into the
/// queue.
///
/// @warning
/// This implementation is a variation of the priority queue described in
/// "Data Structures & Algorithms" by Aho, Hopcroft, Ullman. It creates
/// a balanced, partially ordered binary tree implemented as an ordered
/// array. This avoids the overhead associated with parent/child pointers,
/// and frequent memory allocation and deallocation.
#[allow(non_camel_case_types)]
pub struct vtkPriorityQueue(*mut core::ffi::c_void);
impl vtkPriorityQueue {
    /// Creates a new [vtkPriorityQueue] wrapped inside `vtkNew`
    #[doc(alias = "vtkPriorityQueue")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPriorityQueue_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPriorityQueue_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPriorityQueue_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPriorityQueue_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPriorityQueue {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPriorityQueue {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPriorityQueue_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPriorityQueue_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPriorityQueue_create_drop() {
    let obj = vtkPriorityQueue::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPriorityQueue(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// convenience class to quickly generate a pool of random numbers
///
///
/// vtkRandomPool generates random numbers, and can do so using
/// multithreading.  It supports parallel applications where generating random
/// numbers on the fly is difficult (i.e., non-deterministic). Also, it can be
/// used to populate vtkDataArrays in an efficient manner. By default it uses
/// an instance of vtkMersenneTwister to generate random sequences, but any
/// subclass of vtkRandomSequence may be used. It also supports simple methods
/// to generate, access, and pass random memory pools between objects.
///
/// In threaded applications, these class may be conveniently used to
/// pre-generate a sequence of random numbers, followed by the use of
/// deterministic accessor methods to produce random sequences without
/// problems etc. due to unpredictable work load and order of thread
/// execution.
///
/// @warning
/// The class uses vtkMultiThreader if the size of the pool is larger than
/// the specified chunk size. Also, vtkSMPTools may be used to scale the
/// components in the method PopulateDataArray().
#[allow(non_camel_case_types)]
pub struct vtkRandomPool(*mut core::ffi::c_void);
impl vtkRandomPool {
    /// Creates a new [vtkRandomPool] wrapped inside `vtkNew`
    #[doc(alias = "vtkRandomPool")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRandomPool_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkRandomPool_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkRandomPool_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkRandomPool_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkRandomPool {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRandomPool {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRandomPool_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRandomPool_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRandomPool_create_drop() {
    let obj = vtkRandomPool::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkRandomPool(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Obsolete / empty subclass of object.
///
///
/// vtkReferenceCount functionality has now been moved into vtkObject
/// @sa
/// vtkObject
#[allow(non_camel_case_types)]
pub struct vtkReferenceCount(*mut core::ffi::c_void);
impl vtkReferenceCount {
    /// Creates a new [vtkReferenceCount] wrapped inside `vtkNew`
    #[doc(alias = "vtkReferenceCount")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkReferenceCount_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkReferenceCount_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkReferenceCount_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkReferenceCount_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkReferenceCount {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkReferenceCount {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkReferenceCount_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkReferenceCount_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkReferenceCount_create_drop() {
    let obj = vtkReferenceCount::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkReferenceCount(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for mapping scalar values to colors
///
///
/// vtkScalarsToColors is a general-purpose base class for objects that
/// convert scalars to colors. This include vtkLookupTable classes and
/// color transfer functions.  By itself, this class will simply rescale
/// the scalars.
///
/// The scalar-to-color mapping can be augmented with an additional
/// uniform alpha blend. This is used, for example, to blend a vtkActor's
/// opacity with the lookup table values.
///
/// Specific scalar values may be annotated with text strings that will
/// be included in color legends using \a SetAnnotations, \a SetAnnotation,
/// \a GetNumberOfAnnotatedValues, \a GetAnnotatedValue, \a GetAnnotation,
/// \a RemoveAnnotation, and \a ResetAnnotations.
///
/// This class also has a method for indicating that the set of
/// annotated values form a categorical color map; by setting \a
/// IndexedLookup to true, you indicate that the annotated values are
/// the only valid values for which entries in the color table should
/// be returned. In this mode, subclasses should then assign colors to
/// annotated values by taking the modulus of an annotated value's
/// index in the list of annotations with the number of colors in the
/// table.
///
/// @sa
/// vtkLookupTable vtkColorTransferFunction
#[allow(non_camel_case_types)]
pub struct vtkScalarsToColors(*mut core::ffi::c_void);
impl vtkScalarsToColors {
    /// Creates a new [vtkScalarsToColors] wrapped inside `vtkNew`
    #[doc(alias = "vtkScalarsToColors")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkScalarsToColors_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkScalarsToColors_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkScalarsToColors_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkScalarsToColors_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkScalarsToColors {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkScalarsToColors {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkScalarsToColors_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkScalarsToColors_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkScalarsToColors_create_drop() {
    let obj = vtkScalarsToColors::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkScalarsToColors(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of short
///
///
/// vtkShortArray is an array of values of type short.  It provides
/// methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the short type,
/// so use of this type directly is discouraged.  If an array of 16 bit
/// integers is needed, prefer vtkTypeInt16Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkShortArray(*mut core::ffi::c_void);
impl vtkShortArray {
    /// Creates a new [vtkShortArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkShortArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkShortArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkShortArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkShortArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkShortArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkShortArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkShortArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkShortArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkShortArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkShortArray_create_drop() {
    let obj = vtkShortArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkShortArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of signed char
///
///
/// vtkSignedCharArray is an array of values of type signed char.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkSignedCharArray(*mut core::ffi::c_void);
impl vtkSignedCharArray {
    /// Creates a new [vtkSignedCharArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkSignedCharArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSignedCharArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSignedCharArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSignedCharArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSignedCharArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSignedCharArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSignedCharArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSignedCharArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSignedCharArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSignedCharArray_create_drop() {
    let obj = vtkSignedCharArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSignedCharArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// provides several methods for sorting VTK arrays.
///
///
///
/// vtkSortDataArray is used to sort data, based on its value, or with an
/// associated key, into either ascending or descending order. This is useful
/// for operations like selection, or analysis, when evaluating and processing
/// data. A variety of sorting functions are provided, treating both arrays
/// (i.e., vtkAbstractArray) and id lists (vtkIdList). Note that complex arrays
/// like variants and string arrays are also handled.
///
/// Additional functionality is provided to generate data ordering, without
/// necessarily shuffling the data into a final, sorted position. Hence, the
/// sorting process is organized into three steps because of the complexity of
/// dealing with multiple types and multiple component data arrays. The first
/// step involves creating and initializing a sorted index array, and then
/// (second step) sorting this array to produce a map indicating the sorting
/// order.  In other words, the sorting index array is a permutation which can
/// be applied to other, associated data to shuffle it (third step) into an
/// order consistent with the sorting operation. Note that the generation of
/// the sorted index array is useful unto itself (even without the final
/// shuffling of data) because it generates an ordered list (from the data
/// values of any component in any array). So for example, it is possible to
/// find the top N cells with the largest scalar value simply by generating
/// the sorting index array from the call scalar values.
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
/// vtkSortFieldData
#[allow(non_camel_case_types)]
pub struct vtkSortDataArray(*mut core::ffi::c_void);
impl vtkSortDataArray {
    /// Creates a new [vtkSortDataArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkSortDataArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSortDataArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSortDataArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSortDataArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSortDataArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSortDataArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSortDataArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSortDataArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSortDataArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSortDataArray_create_drop() {
    let obj = vtkSortDataArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSortDataArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a vtkAbstractArray subclass for strings
///
///
/// Points and cells may sometimes have associated data that are stored
/// as strings, e.g. labels for information visualization projects.
/// This class provides a clean way to store and access those strings.
/// @par Thanks:
/// Andy Wilson (atwilso@sandia.gov) wrote this class.
#[allow(non_camel_case_types)]
pub struct vtkStringArray(*mut core::ffi::c_void);
impl vtkStringArray {
    /// Creates a new [vtkStringArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkStringArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStringArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStringArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStringArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStringArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStringArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStringArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStringArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStringArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStringArray_create_drop() {
    let obj = vtkStringArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStringArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// File Specific output window class
///
///
/// Writes debug/warning/error output to a log file instead of the console.
/// To use this class, instantiate it and then call SetInstance(this).
#[allow(non_camel_case_types)]
pub struct vtkStringOutputWindow(*mut core::ffi::c_void);
impl vtkStringOutputWindow {
    /// Creates a new [vtkStringOutputWindow] wrapped inside `vtkNew`
    #[doc(alias = "vtkStringOutputWindow")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStringOutputWindow_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStringOutputWindow_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStringOutputWindow_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStringOutputWindow_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStringOutputWindow {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStringOutputWindow {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStringOutputWindow_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStringOutputWindow_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStringOutputWindow_create_drop() {
    let obj = vtkStringOutputWindow::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStringOutputWindow(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// performs common time operations
///
///
///
/// vtkTimePointUtility is provides methods to perform common time operations.
#[allow(non_camel_case_types)]
pub struct vtkTimePointUtility(*mut core::ffi::c_void);
impl vtkTimePointUtility {
    /// Creates a new [vtkTimePointUtility] wrapped inside `vtkNew`
    #[doc(alias = "vtkTimePointUtility")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTimePointUtility_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTimePointUtility_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTimePointUtility_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTimePointUtility_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTimePointUtility {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTimePointUtility {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTimePointUtility_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTimePointUtility_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTimePointUtility_create_drop() {
    let obj = vtkTimePointUtility::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTimePointUtility(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeFloat32Array(*mut core::ffi::c_void);
impl vtkTypeFloat32Array {
    /// Creates a new [vtkTypeFloat32Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeFloat32Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeFloat32Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeFloat32Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeFloat32Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeFloat32Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeFloat32Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeFloat32Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeFloat32Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeFloat32Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeFloat32Array_create_drop() {
    let obj = vtkTypeFloat32Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeFloat32Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeFloat64Array(*mut core::ffi::c_void);
impl vtkTypeFloat64Array {
    /// Creates a new [vtkTypeFloat64Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeFloat64Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeFloat64Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeFloat64Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeFloat64Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeFloat64Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeFloat64Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeFloat64Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeFloat64Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeFloat64Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeFloat64Array_create_drop() {
    let obj = vtkTypeFloat64Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeFloat64Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeInt16Array(*mut core::ffi::c_void);
impl vtkTypeInt16Array {
    /// Creates a new [vtkTypeInt16Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeInt16Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeInt16Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeInt16Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeInt16Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeInt16Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeInt16Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeInt16Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeInt16Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeInt16Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeInt16Array_create_drop() {
    let obj = vtkTypeInt16Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeInt16Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeInt32Array(*mut core::ffi::c_void);
impl vtkTypeInt32Array {
    /// Creates a new [vtkTypeInt32Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeInt32Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeInt32Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeInt32Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeInt32Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeInt32Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeInt32Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeInt32Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeInt32Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeInt32Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeInt32Array_create_drop() {
    let obj = vtkTypeInt32Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeInt32Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeInt64Array(*mut core::ffi::c_void);
impl vtkTypeInt64Array {
    /// Creates a new [vtkTypeInt64Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeInt64Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeInt64Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeInt64Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeInt64Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeInt64Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeInt64Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeInt64Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeInt64Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeInt64Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeInt64Array_create_drop() {
    let obj = vtkTypeInt64Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeInt64Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeInt8Array(*mut core::ffi::c_void);
impl vtkTypeInt8Array {
    /// Creates a new [vtkTypeInt8Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeInt8Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeInt8Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeInt8Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeInt8Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeInt8Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeInt8Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeInt8Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeInt8Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeInt8Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeInt8Array_create_drop() {
    let obj = vtkTypeInt8Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeInt8Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeUInt16Array(*mut core::ffi::c_void);
impl vtkTypeUInt16Array {
    /// Creates a new [vtkTypeUInt16Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeUInt16Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeUInt16Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeUInt16Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeUInt16Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeUInt16Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeUInt16Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeUInt16Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeUInt16Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeUInt16Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeUInt16Array_create_drop() {
    let obj = vtkTypeUInt16Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeUInt16Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeUInt32Array(*mut core::ffi::c_void);
impl vtkTypeUInt32Array {
    /// Creates a new [vtkTypeUInt32Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeUInt32Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeUInt32Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeUInt32Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeUInt32Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeUInt32Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeUInt32Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeUInt32Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeUInt32Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeUInt32Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeUInt32Array_create_drop() {
    let obj = vtkTypeUInt32Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeUInt32Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeUInt64Array(*mut core::ffi::c_void);
impl vtkTypeUInt64Array {
    /// Creates a new [vtkTypeUInt64Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeUInt64Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeUInt64Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeUInt64Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeUInt64Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeUInt64Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeUInt64Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeUInt64Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeUInt64Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeUInt64Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeUInt64Array_create_drop() {
    let obj = vtkTypeUInt64Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeUInt64Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkTypeUInt8Array(*mut core::ffi::c_void);
impl vtkTypeUInt8Array {
    /// Creates a new [vtkTypeUInt8Array] wrapped inside `vtkNew`
    #[doc(alias = "vtkTypeUInt8Array")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTypeUInt8Array_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTypeUInt8Array_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTypeUInt8Array_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTypeUInt8Array_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTypeUInt8Array {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTypeUInt8Array {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTypeUInt8Array_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTypeUInt8Array_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTypeUInt8Array_create_drop() {
    let obj = vtkTypeUInt8Array::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTypeUInt8Array(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of unsigned char
///
///
/// vtkUnsignedCharArray is an array of values of type unsigned char.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedCharArray(*mut core::ffi::c_void);
impl vtkUnsignedCharArray {
    /// Creates a new [vtkUnsignedCharArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnsignedCharArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedCharArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnsignedCharArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnsignedCharArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnsignedCharArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnsignedCharArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedCharArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedCharArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedCharArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedCharArray_create_drop() {
    let obj = vtkUnsignedCharArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnsignedCharArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of unsigned int
///
///
/// vtkUnsignedIntArray is an array of values of type unsigned int.  It
/// provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the unsigned int type,
/// so use of this type directly is discouraged.  If an array of 32 bit unsigned
/// integers is needed, prefer vtkTypeUInt32Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedIntArray(*mut core::ffi::c_void);
impl vtkUnsignedIntArray {
    /// Creates a new [vtkUnsignedIntArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnsignedIntArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedIntArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnsignedIntArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnsignedIntArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnsignedIntArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnsignedIntArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedIntArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedIntArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedIntArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedIntArray_create_drop() {
    let obj = vtkUnsignedIntArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnsignedIntArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of unsigned long
///
///
/// vtkUnsignedLongArray is an array of values of type unsigned long.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the unsigned long type,
/// so use of this type directly is discouraged.  If an array of 32 bit
/// unsigned integers is needed, prefer vtkTypeUInt32Array to this class.
/// If an array of 64 bit unsigned integers is needed, prefer
/// vtkUTypeInt64Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedLongArray(*mut core::ffi::c_void);
impl vtkUnsignedLongArray {
    /// Creates a new [vtkUnsignedLongArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnsignedLongArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedLongArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnsignedLongArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnsignedLongArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnsignedLongArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnsignedLongArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedLongArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedLongArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedLongArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedLongArray_create_drop() {
    let obj = vtkUnsignedLongArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnsignedLongArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of unsigned long long
///
///
/// vtkUnsignedLongLongArray is an array of values of type unsigned long long.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// This class should not be used directly, as it only exists on systems
/// where the unsigned long long type is defined.  If you need an unsigned
/// 64 bit integer data array, use vtkTypeUInt64Array instead.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedLongLongArray(*mut core::ffi::c_void);
impl vtkUnsignedLongLongArray {
    /// Creates a new [vtkUnsignedLongLongArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnsignedLongLongArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedLongLongArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnsignedLongLongArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnsignedLongLongArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnsignedLongLongArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnsignedLongLongArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedLongLongArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedLongLongArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedLongLongArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedLongLongArray_create_drop() {
    let obj = vtkUnsignedLongLongArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnsignedLongLongArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of unsigned short
///
///
/// vtkUnsignedShortArray is an array of values of type unsigned short.
/// It provides methods for insertion and retrieval of values and will
/// automatically resize itself to hold new data.
///
/// The C++ standard does not define the exact size of the unsigned short type,
/// so use of this type directly is discouraged.  If an array of 16 bit
/// unsigned integers is needed, prefer vtkTypeUInt16Array to this class.
#[allow(non_camel_case_types)]
pub struct vtkUnsignedShortArray(*mut core::ffi::c_void);
impl vtkUnsignedShortArray {
    /// Creates a new [vtkUnsignedShortArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnsignedShortArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnsignedShortArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnsignedShortArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnsignedShortArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnsignedShortArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnsignedShortArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnsignedShortArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnsignedShortArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnsignedShortArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnsignedShortArray_create_drop() {
    let obj = vtkUnsignedShortArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnsignedShortArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// An array holding vtkVariants.
///
///
///
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkVariantArray(*mut core::ffi::c_void);
impl vtkVariantArray {
    /// Creates a new [vtkVariantArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkVariantArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVariantArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkVariantArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkVariantArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkVariantArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkVariantArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVariantArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVariantArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVariantArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVariantArray_create_drop() {
    let obj = vtkVariantArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkVariantArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Versioning class for vtk
///
///
/// Holds methods for defining/determining the current vtk version
/// (major, minor, build).
///
/// @warning
/// This file will change frequently to update the VTKSourceVersion which
/// timestamps a particular source release.
#[allow(non_camel_case_types)]
pub struct vtkVersion(*mut core::ffi::c_void);
impl vtkVersion {
    /// Creates a new [vtkVersion] wrapped inside `vtkNew`
    #[doc(alias = "vtkVersion")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVersion_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkVersion_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkVersion_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkVersion_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkVersion {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVersion {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVersion_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVersion_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVersion_create_drop() {
    let obj = vtkVersion::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkVersion(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// dynamic, self-adjusting array of void* pointers
///
///
/// vtkVoidArray is an array of pointers to void. It provides methods
/// for insertion and retrieval of these pointers values, and will
/// automatically resize itself to hold new data.
#[allow(non_camel_case_types)]
pub struct vtkVoidArray(*mut core::ffi::c_void);
impl vtkVoidArray {
    /// Creates a new [vtkVoidArray] wrapped inside `vtkNew`
    #[doc(alias = "vtkVoidArray")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkVoidArray_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkVoidArray_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkVoidArray_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkVoidArray_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkVoidArray {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkVoidArray {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkVoidArray_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkVoidArray_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkVoidArray_create_drop() {
    let obj = vtkVoidArray::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkVoidArray(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Utility class to hold a weak reference to a vtkObject.
///
///
/// Simple Set(...)/Get(...) interface. Used in numpy support to provide a
/// reference to a vtkObject without preventing it from being collected.
#[allow(non_camel_case_types)]
pub struct vtkWeakReference(*mut core::ffi::c_void);
impl vtkWeakReference {
    /// Creates a new [vtkWeakReference] wrapped inside `vtkNew`
    #[doc(alias = "vtkWeakReference")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkWeakReference_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkWeakReference_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkWeakReference_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkWeakReference_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkWeakReference {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkWeakReference {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkWeakReference_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkWeakReference_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkWeakReference_create_drop() {
    let obj = vtkWeakReference::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkWeakReference(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// XML File Specific output window class
///
///
/// Writes debug/warning/error output to an XML file. Uses prefined XML
/// tags for each text display method. The text is processed to replace
/// XML markup characters.
///
/// DisplayText - \<Text\>
///
/// DisplayErrorText - \<Error\>
///
/// DisplayWarningText - \<Warning\>
///
/// DisplayGenericWarningText - \<GenericWarning\>
///
/// DisplayDebugText - \<Debug\>
///
/// The method DisplayTag outputs the text unprocessed. To use this
/// class, instantiate it and then call SetInstance(this).
#[allow(non_camel_case_types)]
pub struct vtkXMLFileOutputWindow(*mut core::ffi::c_void);
impl vtkXMLFileOutputWindow {
    /// Creates a new [vtkXMLFileOutputWindow] wrapped inside `vtkNew`
    #[doc(alias = "vtkXMLFileOutputWindow")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkXMLFileOutputWindow_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkXMLFileOutputWindow_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkXMLFileOutputWindow_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkXMLFileOutputWindow_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkXMLFileOutputWindow {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkXMLFileOutputWindow {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkXMLFileOutputWindow_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkXMLFileOutputWindow_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkXMLFileOutputWindow_create_drop() {
    let obj = vtkXMLFileOutputWindow::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkXMLFileOutputWindow(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
