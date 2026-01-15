/// Aggregate results in the vtkEndFor
///
///
/// vtkAggregateToPartitionedDataSetCollection is an execution aggregator for the
/// `vtkEndFor` filter that insert each iteration result in a partition of a
/// vtkPartitionnedDataSetCollection.
///
/// @sa vtkEndFor, vtkForEach, vtkExecutionAggregator
#[allow(non_camel_case_types)]
pub struct vtkAggregateToPartitionedDataSetCollection(*mut core::ffi::c_void);
impl vtkAggregateToPartitionedDataSetCollection {
    /// Creates a new [vtkAggregateToPartitionedDataSetCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkAggregateToPartitionedDataSetCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAggregateToPartitionedDataSetCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAggregateToPartitionedDataSetCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAggregateToPartitionedDataSetCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAggregateToPartitionedDataSetCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAggregateToPartitionedDataSetCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAggregateToPartitionedDataSetCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAggregateToPartitionedDataSetCollection_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkAggregateToPartitionedDataSetCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAggregateToPartitionedDataSetCollection_create_drop() {
    let obj = vtkAggregateToPartitionedDataSetCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAggregateToPartitionedDataSetCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for all sources, filters, and sinks in VTK.
///
///
/// vtkAlgorithm is the superclass for all sources, filters, and sinks
/// in VTK.  It defines a generalized interface for executing data
/// processing algorithms.  Pipeline connections are associated with
/// input and output ports that are independent of the type of data
/// passing through the connections.
///
/// Instances may be used independently or within pipelines with a
/// variety of architectures and update mechanisms.  Pipelines are
/// controlled by instances of vtkExecutive.  Every vtkAlgorithm
/// instance has an associated vtkExecutive when it is used in a
/// pipeline.  The executive is responsible for data flow.
#[allow(non_camel_case_types)]
pub struct vtkAlgorithm(*mut core::ffi::c_void);
impl vtkAlgorithm {
    /// Creates a new [vtkAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAlgorithm_create_drop() {
    let obj = vtkAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Proxy object to connect input/output ports.
///
///
/// vtkAlgorithmOutput is a proxy object returned by the GetOutputPort
/// method of vtkAlgorithm.  It may be passed to the
/// SetInputConnection, AddInputConnection, or RemoveInputConnection
/// methods of another vtkAlgorithm to establish a connection between
/// an output and input port.  The connection is not stored in the
/// proxy object: it is simply a convenience for creating or removing
/// connections.
#[allow(non_camel_case_types)]
pub struct vtkAlgorithmOutput(*mut core::ffi::c_void);
impl vtkAlgorithmOutput {
    /// Creates a new [vtkAlgorithmOutput] wrapped inside `vtkNew`
    #[doc(alias = "vtkAlgorithmOutput")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAlgorithmOutput_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAlgorithmOutput_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAlgorithmOutput_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAlgorithmOutput_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAlgorithmOutput {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAlgorithmOutput {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAlgorithmOutput_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAlgorithmOutput_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAlgorithmOutput_create_drop() {
    let obj = vtkAlgorithmOutput::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAlgorithmOutput(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only vtkAnnotationLayers as output
///
///
///
/// vtkAnnotationLayersAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be vtkAnnotationLayers. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkAnnotationLayersAlgorithm(*mut core::ffi::c_void);
impl vtkAnnotationLayersAlgorithm {
    /// Creates a new [vtkAnnotationLayersAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkAnnotationLayersAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAnnotationLayersAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkAnnotationLayersAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkAnnotationLayersAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkAnnotationLayersAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkAnnotationLayersAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAnnotationLayersAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAnnotationLayersAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAnnotationLayersAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAnnotationLayersAlgorithm_create_drop() {
    let obj = vtkAnnotationLayersAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkAnnotationLayersAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce
///
/// vtkArrayDatas as output
///
///
/// vtkArrayDataAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be vtkArrayData. If that
/// isn't the case then please override this method in your subclass.
///
/// @par Thanks:
/// Developed by Timothy M. Shead (tshead@sandia.gov) at Sandia National Laboratories.
#[allow(non_camel_case_types)]
pub struct vtkArrayDataAlgorithm(*mut core::ffi::c_void);
impl vtkArrayDataAlgorithm {
    /// Creates a new [vtkArrayDataAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkArrayDataAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkArrayDataAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkArrayDataAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkArrayDataAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkArrayDataAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkArrayDataAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkArrayDataAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkArrayDataAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkArrayDataAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkArrayDataAlgorithm_create_drop() {
    let obj = vtkArrayDataAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkArrayDataAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
/// vtkCachedStreamingDemandDrivenPipeline
#[allow(non_camel_case_types)]
pub struct vtkCachedStreamingDemandDrivenPipeline(*mut core::ffi::c_void);
impl vtkCachedStreamingDemandDrivenPipeline {
    /// Creates a new [vtkCachedStreamingDemandDrivenPipeline] wrapped inside `vtkNew`
    #[doc(alias = "vtkCachedStreamingDemandDrivenPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCachedStreamingDemandDrivenPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCachedStreamingDemandDrivenPipeline_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCachedStreamingDemandDrivenPipeline_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCachedStreamingDemandDrivenPipeline_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCachedStreamingDemandDrivenPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCachedStreamingDemandDrivenPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCachedStreamingDemandDrivenPipeline_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkCachedStreamingDemandDrivenPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCachedStreamingDemandDrivenPipeline_create_drop() {
    let obj = vtkCachedStreamingDemandDrivenPipeline::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCachedStreamingDemandDrivenPipeline(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// works around type-checking limitations
///
///
/// vtkCastToConcrete is a filter that works around type-checking limitations
/// in the filter classes. Some filters generate abstract types on output,
/// and cannot be connected to the input of filters requiring a concrete
/// input type. For example, vtkElevationFilter generates vtkDataSet for output,
/// and cannot be connected to vtkDecimate, because vtkDecimate requires
/// vtkPolyData as input. This is true even though (in this example) the input
/// to vtkElevationFilter is of type vtkPolyData, and you know the output of
/// vtkElevationFilter is the same type as its input.
///
/// vtkCastToConcrete performs run-time checking to ensure that output type
/// is of the right type. An error message will result if you try to cast
/// an input type improperly. Otherwise, the filter performs the appropriate
/// cast and returns the data.
///
/// @warning
/// You must specify the input before you can get the output. Otherwise an
/// error results.
///
/// @sa
/// vtkDataSetAlgorithm vtkPointSetToPointSetFilter
#[allow(non_camel_case_types)]
pub struct vtkCastToConcrete(*mut core::ffi::c_void);
impl vtkCastToConcrete {
    /// Creates a new [vtkCastToConcrete] wrapped inside `vtkNew`
    #[doc(alias = "vtkCastToConcrete")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCastToConcrete_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCastToConcrete_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCastToConcrete_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCastToConcrete_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCastToConcrete {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCastToConcrete {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCastToConcrete_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCastToConcrete_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCastToConcrete_create_drop() {
    let obj = vtkCastToConcrete::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCastToConcrete(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only polydata as output
///
///
///
/// vtkCellGridAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be PolyData. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkCellGridAlgorithm(*mut core::ffi::c_void);
impl vtkCellGridAlgorithm {
    /// Creates a new [vtkCellGridAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkCellGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCellGridAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCellGridAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCellGridAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCellGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellGridAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellGridAlgorithm_create_drop() {
    let obj = vtkCellGridAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCellGridAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Executive supporting composite datasets.
///
///
/// vtkCompositeDataPipeline is an executive that supports the processing of
/// composite dataset. It supports algorithms that are aware of composite
/// dataset as well as those that are not. Type checking is performed at run
/// time. Algorithms that are not composite dataset-aware have to support
/// all dataset types contained in the composite dataset. The pipeline
/// execution can be summarized as follows:
///
/// * REQUEST_INFORMATION: The producers have to provide information about
/// the contents of the composite dataset in this pass.
/// Sources that can produce more than one piece (note that a piece is
/// different than a block; each piece consists of 0 or more blocks) should
/// set CAN_HANDLE_PIECE_REQUEST.
///
/// * REQUEST_UPDATE_EXTENT: This pass is identical to the one implemented
/// in vtkStreamingDemandDrivenPipeline
///
/// * REQUEST_DATA: This is where the algorithms execute. If the
/// vtkCompositeDataPipeline is assigned to a simple filter,
/// it will invoke the  vtkStreamingDemandDrivenPipeline passes in a loop,
/// passing a different block each time and will collect the results in a
/// composite dataset.
/// @sa
/// vtkCompositeDataSet
#[allow(non_camel_case_types)]
pub struct vtkCompositeDataPipeline(*mut core::ffi::c_void);
impl vtkCompositeDataPipeline {
    /// Creates a new [vtkCompositeDataPipeline] wrapped inside `vtkNew`
    #[doc(alias = "vtkCompositeDataPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCompositeDataPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCompositeDataPipeline_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCompositeDataPipeline_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCompositeDataPipeline_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCompositeDataPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCompositeDataPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCompositeDataPipeline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCompositeDataPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCompositeDataPipeline_create_drop() {
    let obj = vtkCompositeDataPipeline::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCompositeDataPipeline(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only vtkCompositeDataSet as output
///
///
/// Algorithms that take any type of data object (including composite dataset)
/// and produce a vtkCompositeDataSet in the output can subclass from this
/// class.
#[allow(non_camel_case_types)]
pub struct vtkCompositeDataSetAlgorithm(*mut core::ffi::c_void);
impl vtkCompositeDataSetAlgorithm {
    /// Creates a new [vtkCompositeDataSetAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkCompositeDataSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCompositeDataSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkCompositeDataSetAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkCompositeDataSetAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkCompositeDataSetAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkCompositeDataSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCompositeDataSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCompositeDataSetAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCompositeDataSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCompositeDataSetAlgorithm_create_drop() {
    let obj = vtkCompositeDataSetAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkCompositeDataSetAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only data object as output
///
///
///
/// vtkDataObjectAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be DataObject. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkDataObjectAlgorithm(*mut core::ffi::c_void);
impl vtkDataObjectAlgorithm {
    /// Creates a new [vtkDataObjectAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataObjectAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataObjectAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataObjectAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataObjectAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataObjectAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataObjectAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataObjectAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataObjectAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataObjectAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataObjectAlgorithm_create_drop() {
    let obj = vtkDataObjectAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataObjectAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce output of the same type as input
///
///
/// vtkDataSetAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be DataSet. If that isn't
/// the case then please override this method in your subclass. This class
/// breaks out the downstream requests into separate functions such as
/// RequestDataObject RequestData and RequestInformation. The default
/// implementation of RequestDataObject will create an output data of the
/// same type as the input.
#[allow(non_camel_case_types)]
pub struct vtkDataSetAlgorithm(*mut core::ffi::c_void);
impl vtkDataSetAlgorithm {
    /// Creates a new [vtkDataSetAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkDataSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDataSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDataSetAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDataSetAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDataSetAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDataSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDataSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDataSetAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDataSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDataSetAlgorithm_create_drop() {
    let obj = vtkDataSetAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDataSetAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Executive supporting on-demand execution.
///
///
/// vtkDemandDrivenPipeline is an executive that will execute an
/// algorithm only when its outputs are out-of-date with respect to its
/// inputs.
#[allow(non_camel_case_types)]
pub struct vtkDemandDrivenPipeline(*mut core::ffi::c_void);
impl vtkDemandDrivenPipeline {
    /// Creates a new [vtkDemandDrivenPipeline] wrapped inside `vtkNew`
    #[doc(alias = "vtkDemandDrivenPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDemandDrivenPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDemandDrivenPipeline_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDemandDrivenPipeline_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDemandDrivenPipeline_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDemandDrivenPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDemandDrivenPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDemandDrivenPipeline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDemandDrivenPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDemandDrivenPipeline_create_drop() {
    let obj = vtkDemandDrivenPipeline::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDemandDrivenPipeline(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only directed graph as output
///
///
///
/// vtkDirectedGraphAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline edgehitecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Graph. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
///
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkDirectedGraphAlgorithm(*mut core::ffi::c_void);
impl vtkDirectedGraphAlgorithm {
    /// Creates a new [vtkDirectedGraphAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkDirectedGraphAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDirectedGraphAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDirectedGraphAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDirectedGraphAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDirectedGraphAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDirectedGraphAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDirectedGraphAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDirectedGraphAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDirectedGraphAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDirectedGraphAlgorithm_create_drop() {
    let obj = vtkDirectedGraphAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDirectedGraphAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// vtkEndFor define the end of the sub-pipeline to loop
///
///
/// vtkEndFor works together with vtkForEach. It marks the end of the loop.
/// Its goals is to use the given `vtkExecutionAggregator` to process the result
/// of each iteration and provide an output dataset.
///
/// The default aggregator is vtkAggregateToPartitionedDataSetCollection, which
/// build a vtkPartitionedDataSetCollection with each result in a separate partition.
///
/// > Largely inspired by the ttkForEach/ttkEndFor in the TTK project
/// > (https://github.com/topology-tool-kit/ttk/tree/dev)
///
/// @sa vtkForEach, vtkExecutionAggregator
#[allow(non_camel_case_types)]
pub struct vtkEndFor(*mut core::ffi::c_void);
impl vtkEndFor {
    /// Creates a new [vtkEndFor] wrapped inside `vtkNew`
    #[doc(alias = "vtkEndFor")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEndFor_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkEndFor_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkEndFor_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkEndFor_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkEndFor {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEndFor {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEndFor_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEndFor_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEndFor_create_drop() {
    let obj = vtkEndFor::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkEndFor(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// source that manages dataset ensembles
///
///
/// vtkEnsembleSource manages a collection of data sources in order to
/// represent a dataset ensemble. It has the ability to provide meta-data
/// about the ensemble in the form of a table, using the META_DATA key
/// as well as accept a pipeline request using the UPDATE_MEMBER key.
/// Note that it is expected that all ensemble members produce data of the
/// same type.
#[allow(non_camel_case_types)]
pub struct vtkEnsembleSource(*mut core::ffi::c_void);
impl vtkEnsembleSource {
    /// Creates a new [vtkEnsembleSource] wrapped inside `vtkNew`
    #[doc(alias = "vtkEnsembleSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEnsembleSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkEnsembleSource_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkEnsembleSource_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkEnsembleSource_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkEnsembleSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEnsembleSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEnsembleSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEnsembleSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEnsembleSource_create_drop() {
    let obj = vtkEnsembleSource::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkEnsembleSource(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only
///
/// explicit structured grid as output.
#[allow(non_camel_case_types)]
pub struct vtkExplicitStructuredGridAlgorithm(*mut core::ffi::c_void);
impl vtkExplicitStructuredGridAlgorithm {
    /// Creates a new [vtkExplicitStructuredGridAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkExplicitStructuredGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExplicitStructuredGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkExplicitStructuredGridAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkExplicitStructuredGridAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkExplicitStructuredGridAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkExplicitStructuredGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExplicitStructuredGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExplicitStructuredGridAlgorithm_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkExplicitStructuredGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExplicitStructuredGridAlgorithm_create_drop() {
    let obj = vtkExplicitStructuredGridAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkExplicitStructuredGridAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// This method partitions a global extent to N partitions where N is a user
///
/// supplied parameter.
#[allow(non_camel_case_types)]
pub struct vtkExtentRCBPartitioner(*mut core::ffi::c_void);
impl vtkExtentRCBPartitioner {
    /// Creates a new [vtkExtentRCBPartitioner] wrapped inside `vtkNew`
    #[doc(alias = "vtkExtentRCBPartitioner")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExtentRCBPartitioner_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkExtentRCBPartitioner_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkExtentRCBPartitioner_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkExtentRCBPartitioner_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkExtentRCBPartitioner {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExtentRCBPartitioner {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExtentRCBPartitioner_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExtentRCBPartitioner_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExtentRCBPartitioner_create_drop() {
    let obj = vtkExtentRCBPartitioner::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkExtentRCBPartitioner(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Split an extent across other extents.
///
///
/// vtkExtentSplitter splits each input extent into non-overlapping
/// sub-extents that are completely contained within other "source
/// extents".  A source extent corresponds to some resource providing
/// an extent.  Each source extent has an integer identifier, integer
/// priority, and an extent.  The input extents are split into
/// sub-extents according to priority, availability, and amount of
/// overlap of the source extents.  This can be used by parallel data
/// readers to read as few piece files as possible.
#[allow(non_camel_case_types)]
pub struct vtkExtentSplitter(*mut core::ffi::c_void);
impl vtkExtentSplitter {
    /// Creates a new [vtkExtentSplitter] wrapped inside `vtkNew`
    #[doc(alias = "vtkExtentSplitter")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExtentSplitter_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkExtentSplitter_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkExtentSplitter_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkExtentSplitter_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkExtentSplitter {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExtentSplitter {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExtentSplitter_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExtentSplitter_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExtentSplitter_create_drop() {
    let obj = vtkExtentSplitter::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkExtentSplitter(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Generates a structured extent from unstructured.
///
///
///
/// vtkExtentTranslator generates a structured extent from an unstructured
/// extent.  It uses a recursive scheme that splits the largest axis.  A hard
/// coded extent can be used for a starting point.
#[allow(non_camel_case_types)]
pub struct vtkExtentTranslator(*mut core::ffi::c_void);
impl vtkExtentTranslator {
    /// Creates a new [vtkExtentTranslator] wrapped inside `vtkNew`
    #[doc(alias = "vtkExtentTranslator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExtentTranslator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkExtentTranslator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkExtentTranslator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkExtentTranslator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkExtentTranslator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExtentTranslator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExtentTranslator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExtentTranslator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExtentTranslator_create_drop() {
    let obj = vtkExtentTranslator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkExtentTranslator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Algorithm allowing to implement a for loop using the VTK pipeline and a sister filter
///
/// vtkEndFor
///
/// This filter begins a for loop that can execute a portion of a pipeline (sub-pipeline) a certain
/// number of times. To be used in conjunction with the `vtkEndFor` filter that should end the loop.
///
/// > Largely inspired by the ttkForEach/ttkEndFor in the TTK project
/// > (https://github.com/topology-tool-kit/ttk/tree/dev)
///
/// @sa vtkEndFor, vtkExecutionRange
#[allow(non_camel_case_types)]
pub struct vtkForEach(*mut core::ffi::c_void);
impl vtkForEach {
    /// Creates a new [vtkForEach] wrapped inside `vtkNew`
    #[doc(alias = "vtkForEach")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkForEach_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkForEach_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkForEach_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkForEach_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkForEach {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkForEach {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkForEach_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkForEach_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkForEach_create_drop() {
    let obj = vtkForEach::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkForEach(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only graph as output
///
///
///
/// vtkGraphAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Graph. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkGraphAlgorithm(*mut core::ffi::c_void);
impl vtkGraphAlgorithm {
    /// Creates a new [vtkGraphAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkGraphAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGraphAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkGraphAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkGraphAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkGraphAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkGraphAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGraphAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGraphAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGraphAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGraphAlgorithm_create_drop() {
    let obj = vtkGraphAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkGraphAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// superclass for algorithms that
///
/// produce vtkHierarchicalBoxDataSet as output.
///
/// Algorithms that take any type of data object (including composite dataset)
/// and produce a vtkHierarchicalBoxDataSet in the output can subclass from this
/// class.
#[allow(non_camel_case_types)]
pub struct vtkHierarchicalBoxDataSetAlgorithm(*mut core::ffi::c_void);
impl vtkHierarchicalBoxDataSetAlgorithm {
    /// Creates a new [vtkHierarchicalBoxDataSetAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkHierarchicalBoxDataSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHierarchicalBoxDataSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHierarchicalBoxDataSetAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHierarchicalBoxDataSetAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHierarchicalBoxDataSetAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHierarchicalBoxDataSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHierarchicalBoxDataSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHierarchicalBoxDataSetAlgorithm_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkHierarchicalBoxDataSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHierarchicalBoxDataSetAlgorithm_create_drop() {
    let obj = vtkHierarchicalBoxDataSetAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHierarchicalBoxDataSetAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
/// a structured grid instance.
///
///
/// A concrete instance of vtkStructuredGridAlgorithm which provides
/// functionality for converting instances of vtkImageData to vtkStructuredGrid.
#[allow(non_camel_case_types)]
pub struct vtkImageToStructuredGrid(*mut core::ffi::c_void);
impl vtkImageToStructuredGrid {
    /// Creates a new [vtkImageToStructuredGrid] wrapped inside `vtkNew`
    #[doc(alias = "vtkImageToStructuredGrid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImageToStructuredGrid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImageToStructuredGrid_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImageToStructuredGrid_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImageToStructuredGrid_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImageToStructuredGrid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImageToStructuredGrid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImageToStructuredGrid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImageToStructuredGrid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImageToStructuredGrid_create_drop() {
    let obj = vtkImageToStructuredGrid::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImageToStructuredGrid(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Attaches image pipeline to VTK.
///
///
/// vtkImageToStructuredPoints changes an image cache format to
/// a structured points dataset.  It takes an Input plus an optional
/// VectorInput. The VectorInput converts the RGB scalar components
/// of the VectorInput to vector pointdata attributes. This filter
/// will try to reference count the data but in some cases it must
/// make a copy.
#[allow(non_camel_case_types)]
pub struct vtkImageToStructuredPoints(*mut core::ffi::c_void);
impl vtkImageToStructuredPoints {
    /// Creates a new [vtkImageToStructuredPoints] wrapped inside `vtkNew`
    #[doc(alias = "vtkImageToStructuredPoints")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkImageToStructuredPoints_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkImageToStructuredPoints_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkImageToStructuredPoints_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkImageToStructuredPoints_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkImageToStructuredPoints {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkImageToStructuredPoints {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkImageToStructuredPoints_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkImageToStructuredPoints_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkImageToStructuredPoints_create_drop() {
    let obj = vtkImageToStructuredPoints::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkImageToStructuredPoints(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that operate on
///
/// vtkMolecules
///
///
///
/// vtkMoleculeAlgorithm is a convenience class to make writing algorithms
/// easier. There are some assumptions and defaults made by this class you
/// should be aware of. This class defaults such that your filter will have
/// one input port and one output port. If that is not the case simply change
/// it with SetNumberOfInputPorts etc. See this class constructor for the
/// default. This class also provides a FillInputPortInfo method that by
/// default says that all inputs will be vtkMolecules. If that isn't the case
/// then please override this method in your subclass. You should implement
/// the subclass's algorithm into RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkMoleculeAlgorithm(*mut core::ffi::c_void);
impl vtkMoleculeAlgorithm {
    /// Creates a new [vtkMoleculeAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkMoleculeAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMoleculeAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMoleculeAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMoleculeAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMoleculeAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMoleculeAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMoleculeAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMoleculeAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMoleculeAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMoleculeAlgorithm_create_drop() {
    let obj = vtkMoleculeAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMoleculeAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only vtkMultiBlockDataSet as output
///
///
/// Algorithms that take any type of data object (including composite dataset)
/// and produce a vtkMultiBlockDataSet in the output can subclass from this
/// class.
#[allow(non_camel_case_types)]
pub struct vtkMultiBlockDataSetAlgorithm(*mut core::ffi::c_void);
impl vtkMultiBlockDataSetAlgorithm {
    /// Creates a new [vtkMultiBlockDataSetAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkMultiBlockDataSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMultiBlockDataSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMultiBlockDataSetAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMultiBlockDataSetAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMultiBlockDataSetAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMultiBlockDataSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMultiBlockDataSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMultiBlockDataSetAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMultiBlockDataSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMultiBlockDataSetAlgorithm_create_drop() {
    let obj = vtkMultiBlockDataSetAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMultiBlockDataSetAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that would like to make multiple time requests
///
///
/// This class can be inherited by any algorithm that wishes to make multiple
/// time requests upstream.
///
/// A subclass should override `RequestUpdateExtent` and use
/// `vtkMultiTimeStepAlgorithm::UPDATE_TIME_STEPS` key to indicate which
/// timesteps are to be requested. This class will then take care of executing
/// the upstream pipeline to obtain the requested timesteps.
///
/// Subclasses can then override `Execute` which is provided a vector of input
/// data objects corresponding to the requested timesteps.
///
/// In VTK 9.1 and earlier, subclasses overrode `RequestData` instead of
/// `Execute`. RequestData was passed a `vtkMultiBlockDataSet` with blocks corresponding
/// to the input timesteps. However, with addition of vtkPartitionedDataSet and
/// vtkPartitionedDataSetCollection in VTK 9.2, it is not possible to package all
/// input data types into a multiblock dataset. Hence, the method is deprecated
/// and only used when `Execute` is not overridden.
#[allow(non_camel_case_types)]
pub struct vtkMultiTimeStepAlgorithm(*mut core::ffi::c_void);
impl vtkMultiTimeStepAlgorithm {
    /// Creates a new [vtkMultiTimeStepAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkMultiTimeStepAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMultiTimeStepAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkMultiTimeStepAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkMultiTimeStepAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkMultiTimeStepAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkMultiTimeStepAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMultiTimeStepAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMultiTimeStepAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMultiTimeStepAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMultiTimeStepAlgorithm_create_drop() {
    let obj = vtkMultiTimeStepAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkMultiTimeStepAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
/// produce vtkNonOverlappingAMR as output.
#[allow(non_camel_case_types)]
pub struct vtkNonOverlappingAMRAlgorithm(*mut core::ffi::c_void);
impl vtkNonOverlappingAMRAlgorithm {
    /// Creates a new [vtkNonOverlappingAMRAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkNonOverlappingAMRAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkNonOverlappingAMRAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkNonOverlappingAMRAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkNonOverlappingAMRAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkNonOverlappingAMRAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkNonOverlappingAMRAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkNonOverlappingAMRAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkNonOverlappingAMRAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkNonOverlappingAMRAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkNonOverlappingAMRAlgorithm_create_drop() {
    let obj = vtkNonOverlappingAMRAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkNonOverlappingAMRAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A base class for all algorithms that take as input vtkOverlappingAMR and
///
/// produce vtkOverlappingAMR.
#[allow(non_camel_case_types)]
pub struct vtkOverlappingAMRAlgorithm(*mut core::ffi::c_void);
impl vtkOverlappingAMRAlgorithm {
    /// Creates a new [vtkOverlappingAMRAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkOverlappingAMRAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOverlappingAMRAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkOverlappingAMRAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkOverlappingAMRAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkOverlappingAMRAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkOverlappingAMRAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOverlappingAMRAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOverlappingAMRAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOverlappingAMRAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOverlappingAMRAlgorithm_create_drop() {
    let obj = vtkOverlappingAMRAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkOverlappingAMRAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce output of the same type as input
///
///
/// vtkPassInputTypeAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be DataObject. If that isn't
/// the case then please override this method in your subclass. This class
/// breaks out the downstream requests into separate functions such as
/// RequestDataObject RequestData and RequestInformation. The default
/// implementation of RequestDataObject will create an output data of the
/// same type as the input.
#[allow(non_camel_case_types)]
pub struct vtkPassInputTypeAlgorithm(*mut core::ffi::c_void);
impl vtkPassInputTypeAlgorithm {
    /// Creates a new [vtkPassInputTypeAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkPassInputTypeAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPassInputTypeAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPassInputTypeAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPassInputTypeAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPassInputTypeAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPassInputTypeAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPassInputTypeAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPassInputTypeAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPassInputTypeAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPassInputTypeAlgorithm_create_drop() {
    let obj = vtkPassInputTypeAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPassInputTypeAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only piecewise function as output
///
///
///
/// vtkPiecewiseFunctionAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be PiecewiseFunction. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkPiecewiseFunctionAlgorithm(*mut core::ffi::c_void);
impl vtkPiecewiseFunctionAlgorithm {
    /// Creates a new [vtkPiecewiseFunctionAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkPiecewiseFunctionAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPiecewiseFunctionAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPiecewiseFunctionAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPiecewiseFunctionAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPiecewiseFunctionAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPiecewiseFunctionAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPiecewiseFunctionAlgorithm_create_drop() {
    let obj = vtkPiecewiseFunctionAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPiecewiseFunctionAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
#[allow(non_camel_case_types)]
pub struct vtkPiecewiseFunctionShiftScale(*mut core::ffi::c_void);
impl vtkPiecewiseFunctionShiftScale {
    /// Creates a new [vtkPiecewiseFunctionShiftScale] wrapped inside `vtkNew`
    #[doc(alias = "vtkPiecewiseFunctionShiftScale")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionShiftScale_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPiecewiseFunctionShiftScale_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionShiftScale_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPiecewiseFunctionShiftScale_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPiecewiseFunctionShiftScale {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPiecewiseFunctionShiftScale {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPiecewiseFunctionShiftScale_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPiecewiseFunctionShiftScale_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPiecewiseFunctionShiftScale_create_drop() {
    let obj = vtkPiecewiseFunctionShiftScale::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPiecewiseFunctionShiftScale(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce output of the same type as input
///
///
/// vtkPointSetAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be PointSet. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkPointSetAlgorithm(*mut core::ffi::c_void);
impl vtkPointSetAlgorithm {
    /// Creates a new [vtkPointSetAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkPointSetAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointSetAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPointSetAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPointSetAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPointSetAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPointSetAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointSetAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointSetAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointSetAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointSetAlgorithm_create_drop() {
    let obj = vtkPointSetAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPointSetAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only polydata as output
///
///
///
/// vtkPolyDataAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be PolyData. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkPolyDataAlgorithm(*mut core::ffi::c_void);
impl vtkPolyDataAlgorithm {
    /// Creates a new [vtkPolyDataAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkPolyDataAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyDataAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPolyDataAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPolyDataAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPolyDataAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkPolyDataAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyDataAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyDataAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyDataAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyDataAlgorithm_create_drop() {
    let obj = vtkPolyDataAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPolyDataAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Basic class to optionally replace vtkAlgorithm progress functionality.
///
///
/// When the basic functionality in vtkAlgorithm that reports progress is
/// not enough, a subclass of vtkProgressObserver can be used to provide
/// custom functionality.
/// The main use case for this is when an algorithm's RequestData() is
/// called from multiple threads in parallel - the basic functionality in
/// vtkAlgorithm is not thread safe. vtkSMPProgressObserver can
/// handle this situation by routing progress from each thread to a
/// thread local vtkProgressObserver, which will invoke events separately
/// for each thread.
#[allow(non_camel_case_types)]
pub struct vtkProgressObserver(*mut core::ffi::c_void);
impl vtkProgressObserver {
    /// Creates a new [vtkProgressObserver] wrapped inside `vtkNew`
    #[doc(alias = "vtkProgressObserver")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkProgressObserver_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkProgressObserver_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkProgressObserver_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkProgressObserver_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkProgressObserver {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkProgressObserver {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkProgressObserver_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkProgressObserver_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkProgressObserver_create_drop() {
    let obj = vtkProgressObserver::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkProgressObserver(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only rectilinear grid as output
///
///
///
/// vtkRectilinearGridAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be RectilinearGrid. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
#[allow(non_camel_case_types)]
pub struct vtkRectilinearGridAlgorithm(*mut core::ffi::c_void);
impl vtkRectilinearGridAlgorithm {
    /// Creates a new [vtkRectilinearGridAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkRectilinearGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRectilinearGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkRectilinearGridAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkRectilinearGridAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkRectilinearGridAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkRectilinearGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRectilinearGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRectilinearGridAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRectilinearGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRectilinearGridAlgorithm_create_drop() {
    let obj = vtkRectilinearGridAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkRectilinearGridAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Progress observer that is thread safe
///
///
/// vtkSMPProgressObserver is designed to handle progress events coming
/// from an algorithm in a thread safe way. It does this by using
/// thread local objects that it updates. To receive the progress
/// information, one has to listen to the local observer in the same
/// thread. Since the execution will be somewhat load balanced,
/// it may be enough to do this only on the main thread.
#[allow(non_camel_case_types)]
pub struct vtkSMPProgressObserver(*mut core::ffi::c_void);
impl vtkSMPProgressObserver {
    /// Creates a new [vtkSMPProgressObserver] wrapped inside `vtkNew`
    #[doc(alias = "vtkSMPProgressObserver")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSMPProgressObserver_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSMPProgressObserver_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSMPProgressObserver_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSMPProgressObserver_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSMPProgressObserver {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSMPProgressObserver {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSMPProgressObserver_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSMPProgressObserver_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSMPProgressObserver_create_drop() {
    let obj = vtkSMPProgressObserver::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSMPProgressObserver(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only Selection as output
///
///
///
/// vtkSelectionAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline edgehitecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Selection. If that
/// isn't the case then please override this method in your subclass.
/// You should implement the subclass's algorithm into
/// RequestData( request, inputVec, outputVec).
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkSelectionAlgorithm(*mut core::ffi::c_void);
impl vtkSelectionAlgorithm {
    /// Creates a new [vtkSelectionAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkSelectionAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSelectionAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSelectionAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSelectionAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSelectionAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSelectionAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSelectionAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSelectionAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSelectionAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSelectionAlgorithm_create_drop() {
    let obj = vtkSelectionAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSelectionAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// organize data according to scalar values (used to accelerate contouring operations)
///
///
/// vtkSimpleScalarTree creates a pointerless binary tree that helps search
/// for cells that lie within a particular scalar range. This object is used
/// to accelerate some contouring (and other scalar-based techniques).
///
/// The tree consists of an array of (min,max) scalar range pairs per
/// node in the tree. The (min,max) range is determined from looking at
/// the range of the children of the tree node. If the node is a leaf,
/// then the range is determined by scanning the range of scalar data
/// in n cells in the dataset. The n cells are determined by arbitrary
/// selecting cell ids from id(i) to id(i+n), and where n is specified
/// using the BranchingFactor ivar. Note that leaf node i=0 contains
/// the scalar range computed from cell ids (0,n-1); leaf node i=1
/// contains the range from cell ids (n,2n-1); and so on. The
/// implication is that there are no direct lists of cell ids per leaf
/// node, instead the cell ids are implicitly known. Despite the
/// arbitrary grouping of cells, in practice this scalar tree actually
/// performs quite well due to spatial/data coherence.
///
/// This class has an API that supports both serial and parallel
/// operation.  The parallel API enables the using class to grab arrays
/// (or batches) of cells that potentially intersect the
/// isocontour. These batches can then be processed in separate
/// threads.
///
/// @sa
/// vtkScalarTree vtkSpanSpace
#[allow(non_camel_case_types)]
pub struct vtkSimpleScalarTree(*mut core::ffi::c_void);
impl vtkSimpleScalarTree {
    /// Creates a new [vtkSimpleScalarTree] wrapped inside `vtkNew`
    #[doc(alias = "vtkSimpleScalarTree")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSimpleScalarTree_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSimpleScalarTree_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSimpleScalarTree_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSimpleScalarTree_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSimpleScalarTree {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSimpleScalarTree {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSimpleScalarTree_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSimpleScalarTree_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSimpleScalarTree_create_drop() {
    let obj = vtkSimpleScalarTree::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSimpleScalarTree(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// organize data according to scalar span space
///
///
/// This is a helper class used to accelerate contouring operations. Given an
/// dataset, it organizes the dataset cells into a 2D binned space, with
/// coordinate axes (scalar_min,scalar_max). This so-called span space can
/// then be traversed quickly to find the cells that intersect a specified
/// contour value.
///
/// This class has an API that supports both serial and parallel
/// operation.  The parallel API enables the using class to grab arrays
/// (or batches) of cells that lie along a particular row in the span
/// space. These arrays can then be processed separately or in parallel.
///
/// Learn more about span space in these two publications: 1) "A Near
/// Optimal Isosorface Extraction Algorithm Using the Span Space."
/// Yarden Livnat et al. and 2) Isosurfacing in Span Space with Utmost
/// Efficiency." Han-Wei Shen et al.
///
/// @sa
/// vtkScalarTree vtkSimpleScalarTree
#[allow(non_camel_case_types)]
pub struct vtkSpanSpace(*mut core::ffi::c_void);
impl vtkSpanSpace {
    /// Creates a new [vtkSpanSpace] wrapped inside `vtkNew`
    #[doc(alias = "vtkSpanSpace")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSpanSpace_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSpanSpace_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSpanSpace_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSpanSpace_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSpanSpace {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSpanSpace {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSpanSpace_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSpanSpace_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSpanSpace_create_drop() {
    let obj = vtkSpanSpace::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSpanSpace(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// class to build and traverse sphere trees
///
///
/// vtkSphereTree is a helper class used to build and traverse sphere
/// trees. Various types of trees can be constructed for different VTK
/// dataset types, as well well as different approaches to organize
/// the tree into hierarchies.
///
/// Typically building a complete sphere tree consists of two parts: 1)
/// creating spheres for each cell in the dataset, then 2) creating an
/// organizing hierarchy. The structure of the hierarchy varies depending on
/// the topological characteristics of the dataset.
///
/// Once the tree is constructed, various geometric operations are available
/// for quickly selecting cells based on sphere tree operations; for example,
/// process all cells intersecting a plane (i.e., use the sphere tree to identify
/// candidate cells for plane intersection).
///
/// This class does not necessarily create optimal sphere trees because
/// some of its requirements (fast build time, provide simple reference
/// code, a single bounding sphere per cell, etc.) precludes optimal
/// performance. It is also oriented to computing on cells versus the
/// classic problem of collision detection for polygonal models. For
/// more information you want to read Gareth Bradshaw's PhD thesis
/// "Bounding Volume Hierarchies for Level-of-Detail Collision
/// Handling" which does a nice job of laying out the challenges and
/// important algorithms relative to sphere trees and BVH (bounding
/// volume hierarchies).
///
/// @sa
/// vtkSphereTreeFilter vtkPlaneCutter
#[allow(non_camel_case_types)]
pub struct vtkSphereTree(*mut core::ffi::c_void);
impl vtkSphereTree {
    /// Creates a new [vtkSphereTree] wrapped inside `vtkNew`
    #[doc(alias = "vtkSphereTree")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSphereTree_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSphereTree_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSphereTree_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSphereTree_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSphereTree {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSphereTree {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSphereTree_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSphereTree_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSphereTree_create_drop() {
    let obj = vtkSphereTree::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSphereTree(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Executive supporting partial updates.
///
///
/// vtkStreamingDemandDrivenPipeline is an executive that supports
/// updating only a portion of the data set in the pipeline.  This is
/// the style of pipeline update that is provided by the old-style VTK
/// 4.x pipeline.  Instead of always updating an entire data set, this
/// executive supports asking for pieces or sub-extents.
#[allow(non_camel_case_types)]
pub struct vtkStreamingDemandDrivenPipeline(*mut core::ffi::c_void);
impl vtkStreamingDemandDrivenPipeline {
    /// Creates a new [vtkStreamingDemandDrivenPipeline] wrapped inside `vtkNew`
    #[doc(alias = "vtkStreamingDemandDrivenPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStreamingDemandDrivenPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStreamingDemandDrivenPipeline_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStreamingDemandDrivenPipeline_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStreamingDemandDrivenPipeline_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStreamingDemandDrivenPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStreamingDemandDrivenPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStreamingDemandDrivenPipeline_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkStreamingDemandDrivenPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStreamingDemandDrivenPipeline_create_drop() {
    let obj = vtkStreamingDemandDrivenPipeline::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStreamingDemandDrivenPipeline(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only structured grid as output
///
///
///
/// vtkStructuredGridAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be StructuredGrid. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkStructuredGridAlgorithm(*mut core::ffi::c_void);
impl vtkStructuredGridAlgorithm {
    /// Creates a new [vtkStructuredGridAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkStructuredGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkStructuredGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkStructuredGridAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkStructuredGridAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkStructuredGridAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkStructuredGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkStructuredGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkStructuredGridAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkStructuredGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkStructuredGridAlgorithm_create_drop() {
    let obj = vtkStructuredGridAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkStructuredGridAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only vtkTables as output
///
///
///
/// vtkTableAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Tree. If that
/// isn't the case then please override this method in your subclass.
///
/// @par Thanks:
/// Thanks to Brian Wylie for creating this class.
#[allow(non_camel_case_types)]
pub struct vtkTableAlgorithm(*mut core::ffi::c_void);
impl vtkTableAlgorithm {
    /// Creates a new [vtkTableAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkTableAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTableAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTableAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTableAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTableAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTableAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTableAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTableAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTableAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTableAlgorithm_create_drop() {
    let obj = vtkTableAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTableAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Executive that works in parallel
///
///
/// vtkThreadedCompositeDataPipeline processes a composite data object in
/// parallel using the SMP framework. It does this by creating a vector of
/// data objects (the pieces of the composite data) and processing them
/// using vtkSMPTools::For. Note that this requires that the
/// algorithm implement all pipeline passes in a re-entrant way. It should
/// store/retrieve all state changes using input and output information
/// objects, which are unique to each thread.
#[allow(non_camel_case_types)]
pub struct vtkThreadedCompositeDataPipeline(*mut core::ffi::c_void);
impl vtkThreadedCompositeDataPipeline {
    /// Creates a new [vtkThreadedCompositeDataPipeline] wrapped inside `vtkNew`
    #[doc(alias = "vtkThreadedCompositeDataPipeline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkThreadedCompositeDataPipeline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkThreadedCompositeDataPipeline_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkThreadedCompositeDataPipeline_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkThreadedCompositeDataPipeline_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkThreadedCompositeDataPipeline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkThreadedCompositeDataPipeline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkThreadedCompositeDataPipeline_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkThreadedCompositeDataPipeline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkThreadedCompositeDataPipeline_create_drop() {
    let obj = vtkThreadedCompositeDataPipeline::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkThreadedCompositeDataPipeline(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// vtkExecutionRange using time to dispatch in a vtkForEach sub-pipeline
///
///
/// vtkTimeRange is an execution range for the vtkForEach, that split execution by time steps.
/// The resulting sub-pipeline will be executed once for each time step of the input dataset.
///
/// @sa vtkForEach, vtkExecutionRange, vtkTimeRange
#[allow(non_camel_case_types)]
pub struct vtkTimeRange(*mut core::ffi::c_void);
impl vtkTimeRange {
    /// Creates a new [vtkTimeRange] wrapped inside `vtkNew`
    #[doc(alias = "vtkTimeRange")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTimeRange_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTimeRange_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTimeRange_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTimeRange_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTimeRange {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTimeRange {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTimeRange_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTimeRange_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTimeRange_create_drop() {
    let obj = vtkTimeRange::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTimeRange(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only Tree as output
///
///
///
/// vtkTreeAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline edgehitecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Tree. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkTreeAlgorithm(*mut core::ffi::c_void);
impl vtkTreeAlgorithm {
    /// Creates a new [vtkTreeAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkTreeAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTreeAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTreeAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTreeAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTreeAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTreeAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTreeAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTreeAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTreeAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTreeAlgorithm_create_drop() {
    let obj = vtkTreeAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTreeAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Consumer to consume data off of a pipeline.
///
///
/// vtkTrivialConsumer caps off a pipeline so that no output data is left
/// hanging around when a pipeline executes when data is set to be released (see
/// vtkDataObject::SetGlobalReleaseDataFlag). This is intended to be used for
/// tools such as Catalyst and not end users.
#[allow(non_camel_case_types)]
pub struct vtkTrivialConsumer(*mut core::ffi::c_void);
impl vtkTrivialConsumer {
    /// Creates a new [vtkTrivialConsumer] wrapped inside `vtkNew`
    #[doc(alias = "vtkTrivialConsumer")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTrivialConsumer_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTrivialConsumer_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTrivialConsumer_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTrivialConsumer_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTrivialConsumer {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTrivialConsumer {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTrivialConsumer_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTrivialConsumer_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTrivialConsumer_create_drop() {
    let obj = vtkTrivialConsumer::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTrivialConsumer(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Producer for stand-alone data objects.
///
///
/// vtkTrivialProducer allows stand-alone data objects to be connected
/// as inputs in a pipeline.  All data objects that are connected to a
/// pipeline involving vtkAlgorithm must have a producer.  This trivial
/// producer allows data objects that are hand-constructed in a program
/// without another vtk producer to be connected.
#[allow(non_camel_case_types)]
pub struct vtkTrivialProducer(*mut core::ffi::c_void);
impl vtkTrivialProducer {
    /// Creates a new [vtkTrivialProducer] wrapped inside `vtkNew`
    #[doc(alias = "vtkTrivialProducer")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTrivialProducer_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTrivialProducer_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTrivialProducer_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTrivialProducer_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTrivialProducer {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTrivialProducer {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTrivialProducer_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTrivialProducer_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTrivialProducer_create_drop() {
    let obj = vtkTrivialProducer::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTrivialProducer(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce undirected graph as output
///
///
///
/// vtkUndirectedGraphAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline edgehitecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this class
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be Graph. If that
/// isn't the case then please override this method in your subclass.
///
/// @par Thanks:
/// Thanks to Patricia Crossno, Ken Moreland, Andrew Wilson and Brian Wylie from
/// Sandia National Laboratories for their help in developing this class.
#[allow(non_camel_case_types)]
pub struct vtkUndirectedGraphAlgorithm(*mut core::ffi::c_void);
impl vtkUndirectedGraphAlgorithm {
    /// Creates a new [vtkUndirectedGraphAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkUndirectedGraphAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUndirectedGraphAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUndirectedGraphAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUndirectedGraphAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUndirectedGraphAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUndirectedGraphAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUndirectedGraphAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUndirectedGraphAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUndirectedGraphAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUndirectedGraphAlgorithm_create_drop() {
    let obj = vtkUndirectedGraphAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUndirectedGraphAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
/// vtkUniformGridAMR as output.
///
///
/// A base class for all algorithms that take as input any type of data object
/// including composite datasets and produce vtkUniformGridAMR in the output.
#[allow(non_camel_case_types)]
pub struct vtkUniformGridAMRAlgorithm(*mut core::ffi::c_void);
impl vtkUniformGridAMRAlgorithm {
    /// Creates a new [vtkUniformGridAMRAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkUniformGridAMRAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformGridAMRAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUniformGridAMRAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUniformGridAMRAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUniformGridAMRAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUniformGridAMRAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformGridAMRAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformGridAMRAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformGridAMRAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformGridAMRAlgorithm_create_drop() {
    let obj = vtkUniformGridAMRAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUniformGridAMRAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
///
/// A concrete implementation of vtkMultiBlockDataSetAlgorithm that provides
/// functionality for partitioning a uniform grid. The partitioning method
/// that is used is Recursive Coordinate Bisection (RCB) where each time
/// the longest dimension is split.
///
/// @sa
/// vtkStructuredGridPartitioner vtkRectilinearGridPartitioner
#[allow(non_camel_case_types)]
pub struct vtkUniformGridPartitioner(*mut core::ffi::c_void);
impl vtkUniformGridPartitioner {
    /// Creates a new [vtkUniformGridPartitioner] wrapped inside `vtkNew`
    #[doc(alias = "vtkUniformGridPartitioner")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformGridPartitioner_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUniformGridPartitioner_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUniformGridPartitioner_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUniformGridPartitioner_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUniformGridPartitioner {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformGridPartitioner {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformGridPartitioner_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformGridPartitioner_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformGridPartitioner_create_drop() {
    let obj = vtkUniformGridPartitioner::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUniformGridPartitioner(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that produce only unstructured grid as output
///
///
///
/// vtkUnstructuredGridAlgorithm is a convenience class to make writing algorithms
/// easier. It is also designed to help transition old algorithms to the new
/// pipeline architecture. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be UnstructuredGrid. If that
/// isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkUnstructuredGridAlgorithm(*mut core::ffi::c_void);
impl vtkUnstructuredGridAlgorithm {
    /// Creates a new [vtkUnstructuredGridAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnstructuredGridAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnstructuredGridAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnstructuredGridAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnstructuredGridAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnstructuredGridAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnstructuredGridAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnstructuredGridAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnstructuredGridAlgorithm_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUnstructuredGridAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnstructuredGridAlgorithm_create_drop() {
    let obj = vtkUnstructuredGridAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnstructuredGridAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Superclass for algorithms that
///
/// produce only vtkUnstructureGridBase subclasses as output
///
/// vtkUnstructuredGridBaseAlgorithm is a convenience class to make writing
/// algorithms easier. There are some assumptions and defaults made by this
/// class you should be aware of. This class defaults such that your filter
/// will have one input port and one output port. If that is not the case
/// simply change it with SetNumberOfInputPorts etc. See this classes
/// constructor for the default. This class also provides a FillInputPortInfo
/// method that by default says that all inputs will be UnstructuredGridBase. If
/// that isn't the case then please override this method in your subclass.
#[allow(non_camel_case_types)]
pub struct vtkUnstructuredGridBaseAlgorithm(*mut core::ffi::c_void);
impl vtkUnstructuredGridBaseAlgorithm {
    /// Creates a new [vtkUnstructuredGridBaseAlgorithm] wrapped inside `vtkNew`
    #[doc(alias = "vtkUnstructuredGridBaseAlgorithm")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUnstructuredGridBaseAlgorithm_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkUnstructuredGridBaseAlgorithm_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkUnstructuredGridBaseAlgorithm_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkUnstructuredGridBaseAlgorithm_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkUnstructuredGridBaseAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUnstructuredGridBaseAlgorithm {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUnstructuredGridBaseAlgorithm_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkUnstructuredGridBaseAlgorithm_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUnstructuredGridBaseAlgorithm_create_drop() {
    let obj = vtkUnstructuredGridBaseAlgorithm::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkUnstructuredGridBaseAlgorithm(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
