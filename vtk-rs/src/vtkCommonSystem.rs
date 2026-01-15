/// Encapsulates a client socket.
///
#[allow(non_camel_case_types)]
pub struct vtkClientSocket(*mut core::ffi::c_void);
impl vtkClientSocket {
    /// Creates a new [vtkClientSocket] wrapped inside `vtkNew`
    #[doc(alias = "vtkClientSocket")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkClientSocket_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkClientSocket_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkClientSocket_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkClientSocket_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkClientSocket {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkClientSocket {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkClientSocket_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkClientSocket_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkClientSocket_create_drop() {
    let obj = vtkClientSocket::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkClientSocket(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// OS independent class for access and manipulation of system directories
///
///
/// vtkDirectory provides a portable way of finding the names of the files
/// in a system directory.  It also provides methods of manipulating directories.
///
/// @warning
/// vtkDirectory works with windows and unix only.
#[allow(non_camel_case_types)]
pub struct vtkDirectory(*mut core::ffi::c_void);
impl vtkDirectory {
    /// Creates a new [vtkDirectory] wrapped inside `vtkNew`
    #[doc(alias = "vtkDirectory")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDirectory_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkDirectory_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkDirectory_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkDirectory_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkDirectory {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDirectory {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDirectory_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDirectory_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDirectory_create_drop() {
    let obj = vtkDirectory::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkDirectory(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Launch a process on the current machine and get its output
///
///
/// Launch a process on the current machine and get its standard output and
/// standard error output.
#[allow(non_camel_case_types)]
pub struct vtkExecutableRunner(*mut core::ffi::c_void);
impl vtkExecutableRunner {
    /// Creates a new [vtkExecutableRunner] wrapped inside `vtkNew`
    #[doc(alias = "vtkExecutableRunner")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExecutableRunner_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkExecutableRunner_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkExecutableRunner_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkExecutableRunner_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkExecutableRunner {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExecutableRunner {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExecutableRunner_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExecutableRunner_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExecutableRunner_create_drop() {
    let obj = vtkExecutableRunner::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkExecutableRunner(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Encapsulate a socket that accepts connections.
///
#[allow(non_camel_case_types)]
pub struct vtkServerSocket(*mut core::ffi::c_void);
impl vtkServerSocket {
    /// Creates a new [vtkServerSocket] wrapped inside `vtkNew`
    #[doc(alias = "vtkServerSocket")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkServerSocket_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkServerSocket_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkServerSocket_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkServerSocket_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkServerSocket {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkServerSocket {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkServerSocket_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkServerSocket_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkServerSocket_create_drop() {
    let obj = vtkServerSocket::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkServerSocket(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// a collection for sockets.
///
///
/// Apart from being vtkCollection subclass for sockets, this class
/// provides means to wait for activity on all the sockets in the
/// collection simultaneously.
#[allow(non_camel_case_types)]
pub struct vtkSocketCollection(*mut core::ffi::c_void);
impl vtkSocketCollection {
    /// Creates a new [vtkSocketCollection] wrapped inside `vtkNew`
    #[doc(alias = "vtkSocketCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSocketCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkSocketCollection_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkSocketCollection_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkSocketCollection_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkSocketCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSocketCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSocketCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSocketCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSocketCollection_create_drop() {
    let obj = vtkSocketCollection::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkSocketCollection(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A class for performing inter-thread messaging
///
///
/// vtkThreadMessager is a class that provides support for messaging between
/// threads multithreaded using pthreads or Windows messaging.
#[allow(non_camel_case_types)]
pub struct vtkThreadMessager(*mut core::ffi::c_void);
impl vtkThreadMessager {
    /// Creates a new [vtkThreadMessager] wrapped inside `vtkNew`
    #[doc(alias = "vtkThreadMessager")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkThreadMessager_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkThreadMessager_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkThreadMessager_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkThreadMessager_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkThreadMessager {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkThreadMessager {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkThreadMessager_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkThreadMessager_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkThreadMessager_create_drop() {
    let obj = vtkThreadMessager::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkThreadMessager(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Timer support and logging
///
///
/// vtkTimerLog contains walltime and cputime measurements associated
/// with a given event.  These results can be later analyzed when
/// "dumping out" the table.
///
/// In addition, vtkTimerLog allows the user to simply get the current
/// time, and to start/stop a simple timer separate from the timing
/// table logging.
#[allow(non_camel_case_types)]
pub struct vtkTimerLog(*mut core::ffi::c_void);
impl vtkTimerLog {
    /// Creates a new [vtkTimerLog] wrapped inside `vtkNew`
    #[doc(alias = "vtkTimerLog")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTimerLog_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkTimerLog_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkTimerLog_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkTimerLog_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkTimerLog {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTimerLog {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTimerLog_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTimerLog_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTimerLog_create_drop() {
    let obj = vtkTimerLog::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkTimerLog(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
