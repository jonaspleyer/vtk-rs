#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_object.h");

        pub(crate) type vtkObject;

        pub(crate) fn debug_on(vtk_object: Pin<&mut vtkObject>);
        pub(crate) fn debug_off(vtk_object: Pin<&mut vtkObject>);
        pub(crate) fn set_debug(object: Pin<&mut vtkObject>, status: bool);
        pub(crate) fn get_debug(object: &vtkObject) -> bool;
        pub(crate) fn modified(object: &vtkObject);
        pub(crate) fn remove_observer(object: Pin<&mut vtkObject>, tag: u64);
        pub(crate) fn remove_observers(object: Pin<&mut vtkObject>, event: u64);
        pub(crate) fn remove_all_observers(object: Pin<&mut vtkObject>);
        pub(crate) fn has_observer(object: &vtkObject, event: u64) -> i64;
        // pub(crate) fn add_observer(object: Pin<&mut vtkObject>, event: u64, command: &vtkCommand, priority: f32);

        // pub(crate) unsafe fn print_self(ptr: &vtkObject, indent: usize) -> String;
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkObject`](https://vtk.org/doc/nightly/html/classvtkObject.html)
#[allow(non_camel_case_types)]
pub trait vtkObject: private::Sealed {
    fn as_vtk_object(&self) -> core::pin::Pin<&crate::vtk_object::ffi::vtkObject>;
    fn as_vtk_object_mut(&mut self) -> core::pin::Pin<&mut crate::vtk_object::ffi::vtkObject>;

    /// [`SetDebug`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    // fn set_debug(&mut self, status: bool);
    #[doc(alias = "SetDebug")]
    fn set_debug(&mut self, status: bool) {
        crate::vtk_object::ffi::set_debug(self.as_vtk_object_mut(), status);
    }

    /// [`GetDebug`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    // fn get_debug(&self) -> bool;
    #[doc(alias = "GetDebug")]
    fn get_debug(&self) -> bool {
        crate::vtk_object::ffi::get_debug(&self.as_vtk_object())
    }

    /// [`DebugOff`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    #[doc(alias = "DebugOn")]
    fn debug_on(&mut self) {
        crate::vtk_object::ffi::debug_on(self.as_vtk_object_mut())
    }

    /// [`DebugOn`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    #[doc(alias = "DebugOff")]
    fn debug_off(&mut self) {
        crate::vtk_object::ffi::debug_off(self.as_vtk_object_mut())
    }

    /// [`Modified`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    #[doc(alias = "Modified")]
    fn modified(&mut self) {
        crate::vtk_object::ffi::modified(&self.as_vtk_object())
    }

    /// [`RemoveObserver`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    #[doc(alias = "RemoveObserver")]
    fn remove_observer(&mut self, tag: u64) {
        crate::vtk_object::ffi::remove_observer(self.as_vtk_object_mut(), tag)
    }

    /// [`RemoveObservers`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    #[doc(alias = "RemoveObservers")]
    fn remove_observers(&mut self, event: u64) {
        crate::vtk_object::ffi::remove_observers(self.as_vtk_object_mut(), event)
    }

    /// [`RemoveAllObservers`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    #[doc(alias = "RemoveAllObservers")]
    fn remove_all_observers(&mut self) {
        crate::vtk_object::ffi::remove_all_observers(self.as_vtk_object_mut())
    }

    /// [`HasObserver`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    #[doc(alias = "HasObserver")]
    fn has_observer(&self, event: u64) -> i64 {
        crate::vtk_object::ffi::has_observer(&self.as_vtk_object(), event)
    }

    // TODO
    // [`AddObserver`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    // fn add_observer(&self, vtk_command: impl ..., priority: f64);
    // fn invoke_event(&self, event: c_ulong);
    // fn print_self(&self, indent: usize) -> String;
}
