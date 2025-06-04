#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_executive.h");

        pub(crate) type vtkExecutive;

        fn vtk_executive_delete(executive: Pin<&mut vtkExecutive>);
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkExecutive`](https://vtk.org/doc/nightly/html/classvtkExecutive.html)
#[allow(non_camel_case_types)]
pub trait vtkExecutive: private::Sealed {
    #[doc(hidden)]
    fn as_vtk_executive(&self) -> core::pin::Pin<&ffi::vtkExecutive>;
    #[doc(hidden)]
    fn as_vtk_executive_mut(&mut self) -> core::pin::Pin<&mut ffi::vtkExecutive>;
}

/* crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkExecutive.html",
    @name Executive, ffi::vtkExecutive,
    @delete ffi::vtk_executive_delete,
    @inherit vtkExecutive
);*/

/// [`vtkExecutive`](https://vtk.org/doc/nightly/html/classvtkExecutive.html)
pub struct Executive {
    ptr: core::pin::Pin<&'static mut ffi::vtkExecutive>,
}

impl core::ops::Drop for Executive {
    fn drop(&mut self) {
        ffi::vtk_executive_delete(self.ptr.as_mut());
    }
}

crate::inherit!(@notest Executive vtkObject ffi::vtkExecutive);
