use core::ffi::*;

pub(crate) mod ffi {
    use core::ffi::*;

    unsafe extern "C" {
        pub(crate) unsafe fn debug_on(ptr: *mut c_void);
        pub(crate) unsafe fn debug_off(ptr: *mut c_void);
        pub(crate) unsafe fn set_debug(ptr: *mut c_void, status: bool);
        pub(crate) unsafe fn get_debug(ptr: *const c_void) -> bool;
        pub(crate) unsafe fn modified(ptr: *mut c_void);
        pub(crate) unsafe fn remove_observer(ptr: *mut c_void, tag: u64);
        pub(crate) unsafe fn remove_observers(ptr: *mut c_void, event: u64);
        pub(crate) unsafe fn remove_all_observers(ptr: *mut c_void);
        pub(crate) unsafe fn has_observer(ptr: *const c_void, event: c_ulong) -> c_int;
    }
}

#[cxx::bridge]
pub(crate) mod cxx_ffi {
    unsafe extern "C++" {
        include!("vtk_object.h");

        pub(crate) type vtkObject;

        pub(crate) unsafe fn get_object_description(ptr: *mut vtkObject) -> String;
        pub(crate) unsafe fn print_self(ptr: *mut vtkObject, indent: usize) -> String;
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkObject`](https://vtk.org/doc/nightly/html/classvtkObject.html)
pub trait Object: private::Sealed {
    /// [`SetDebug`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn set_debug(&mut self, status: bool);
    /// [`GetDebug`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn get_debug(&self) -> bool;
    /// [`DebugOff`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn debug_on(&mut self);
    /// [`DebugOn`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn debug_off(&mut self);
    /// [`Modified`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn modified(&mut self);
    /// [`RemoveObserver`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn remove_observer(&mut self, tag: c_ulong);
    /// [`RemoveObservers`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn remove_observers(&mut self, event: c_ulong);
    /// [`RemoveAllObservers`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn remove_all_observers(&mut self);
    /// [`HasObserver`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    fn has_observer(&self, event: c_ulong) -> c_int;
    // TODO
    // [`AddObserver`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    // fn add_observer(&self, vtk_command: impl ..., priority: f64);
    fn get_object_description(&self) -> String;
    // fn invoke_event(&self, event: c_ulong);
    // fn print_self(&self) -> String;
}
