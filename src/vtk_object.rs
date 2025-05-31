#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_object.h");

        pub(crate) type vtkObject;

        fn vtk_object_debug_on(vtk_object: Pin<&mut vtkObject>);
        fn vtk_object_debug_off(vtk_object: Pin<&mut vtkObject>);
        fn vtk_object_set_debug(object: Pin<&mut vtkObject>, status: bool);
        fn vtk_object_get_debug(object: &vtkObject) -> bool;
        fn vtk_object_modified(object: &vtkObject);
        fn vtk_object_remove_observer(object: Pin<&mut vtkObject>, tag: u64);
        fn vtk_object_remove_observers(object: Pin<&mut vtkObject>, event: u64);
        fn vtk_object_remove_all_observers(object: Pin<&mut vtkObject>);
        fn vtk_object_has_observer(object: &vtkObject, event: u64) -> u64;
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
    #[doc(hidden)]
    fn as_vtk_object(&self) -> core::pin::Pin<&ffi::vtkObject>;
    #[doc(hidden)]
    fn as_vtk_object_mut(&mut self) -> core::pin::Pin<&mut ffi::vtkObject>;

    #[doc(alias = "SetDebug")]
    fn set_debug(&mut self, status: bool) {
        ffi::vtk_object_set_debug(self.as_vtk_object_mut(), status);
    }

    #[doc(alias = "GetDebug")]
    fn get_debug(&self) -> bool {
        ffi::vtk_object_get_debug(&self.as_vtk_object())
    }

    #[doc(alias = "DebugOn")]
    fn debug_on(&mut self) {
        ffi::vtk_object_debug_on(self.as_vtk_object_mut())
    }

    #[doc(alias = "DebugOff")]
    fn debug_off(&mut self) {
        ffi::vtk_object_debug_off(self.as_vtk_object_mut())
    }

    #[doc(alias = "Modified")]
    fn modified(&mut self) {
        ffi::vtk_object_modified(&self.as_vtk_object())
    }

    #[doc(alias = "RemoveObserver")]
    fn remove_observer(&mut self, tag: u64) {
        ffi::vtk_object_remove_observer(self.as_vtk_object_mut(), tag)
    }

    #[doc(alias = "RemoveObservers")]
    fn remove_observers(&mut self, event: u64) {
        ffi::vtk_object_remove_observers(self.as_vtk_object_mut(), event)
    }

    #[doc(alias = "RemoveAllObservers")]
    fn remove_all_observers(&mut self) {
        ffi::vtk_object_remove_all_observers(self.as_vtk_object_mut())
    }

    #[doc(alias = "HasObserver")]
    fn has_observer(&self, event: u64) -> u64 {
        ffi::vtk_object_has_observer(&self.as_vtk_object(), event)
    }

    // TODO
    // [`AddObserver`](https://vtk.org/doc/nightly/html/classvtkObject.html)
    // fn add_observer(&self, vtk_command: impl ..., priority: f64);
    // fn invoke_event(&self, event: c_ulong);
    // fn print_self(&self, indent: usize) -> String;
}
