/// Writes an archive to a buffer for vtk-js datasets
///
///
/// vtkvtkJSBufferedArchiver is a specialized archiver for writing datasets into
/// a memory buffer with zip compression.
///
/// @sa
/// vtkArchiver
#[allow(non_camel_case_types)]
pub struct vtkBufferedArchiver(*mut core::ffi::c_void);
impl vtkBufferedArchiver {
    /// Creates a new [vtkBufferedArchiver] wrapped inside `vtkNew`
    #[doc(alias = "vtkBufferedArchiver")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkBufferedArchiver_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkBufferedArchiver_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkBufferedArchiver_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkBufferedArchiver_get_ptr(self.0) }
    }
}
impl crate::vtkCommonCore::VtkArchiver for vtkBufferedArchiver {
    fn set_archive_name(&self, name: &str) {
        unsafe extern "C" {
            fn vtkBufferedArchiver_set_archive_name(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            );
        }
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe { vtkBufferedArchiver_set_archive_name(self.0, c_name.as_ptr()) }
    }
    fn get_archive_name(&self) -> &str {
        unsafe extern "C" {
            fn vtkBufferedArchiver_get_archive_name(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtkBufferedArchiver_get_archive_name(self.0) };
        if ptr.is_null() { return ""; }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}
impl std::default::Default for vtkBufferedArchiver {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkBufferedArchiver {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkBufferedArchiver_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkBufferedArchiver_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkBufferedArchiver_create_drop() {
    let obj = vtkBufferedArchiver::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkBufferedArchiver(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
#[test]
fn test_vtkBufferedArchiver_set_archive_name() {
    use crate::vtkCommonCore::VtkArchiver;
    let obj = vtkBufferedArchiver::new();
    obj.set_archive_name("test_archive");
    assert_eq!(obj.get_archive_name(), "test_archive");
}
/// Writes an archive to several buffers
///
///
/// vtkPartitionedArchiver is a specialized archiver for writing datasets into
/// several memory buffers with zip compression. Each insertion into the archiver
/// is assigned to its own buffer.
///
/// @sa
/// vtkArchiver
#[allow(non_camel_case_types)]
pub struct vtkPartitionedArchiver(*mut core::ffi::c_void);
impl vtkPartitionedArchiver {
    /// Creates a new [vtkPartitionedArchiver] wrapped inside `vtkNew`
    #[doc(alias = "vtkPartitionedArchiver")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPartitionedArchiver_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkPartitionedArchiver_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkPartitionedArchiver_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkPartitionedArchiver_get_ptr(self.0) }
    }
}
impl crate::vtkCommonCore::VtkArchiver for vtkPartitionedArchiver {
    fn set_archive_name(&self, name: &str) {
        unsafe extern "C" {
            fn vtkPartitionedArchiver_set_archive_name(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            );
        }
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe { vtkPartitionedArchiver_set_archive_name(self.0, c_name.as_ptr()) }
    }
    fn get_archive_name(&self) -> &str {
        unsafe extern "C" {
            fn vtkPartitionedArchiver_get_archive_name(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtkPartitionedArchiver_get_archive_name(self.0) };
        if ptr.is_null() { return ""; }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
}
impl std::default::Default for vtkPartitionedArchiver {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPartitionedArchiver {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPartitionedArchiver_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPartitionedArchiver_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPartitionedArchiver_create_drop() {
    let obj = vtkPartitionedArchiver::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkPartitionedArchiver(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
#[test]
fn test_vtkPartitionedArchiver_set_archive_name() {
    use crate::vtkCommonCore::VtkArchiver;
    let obj = vtkPartitionedArchiver::new();
    obj.set_archive_name("test_archive");
    assert_eq!(obj.get_archive_name(), "test_archive");
}
