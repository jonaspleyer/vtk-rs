#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_data_object.h");

        pub type vtkDataObject;

        fn vtk_data_object_new() -> *mut vtkDataObject;
        fn vtk_data_object_delete(data_object: Pin<&mut vtkDataObject>);
        fn vtk_data_object_initialize(data_object: Pin<&mut vtkDataObject>);
        fn vtk_data_object_release_data(data_object: Pin<&mut vtkDataObject>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkNamedColors.html",
    @name DataObject, ffi::vtkDataObject,
    @new ffi::vtk_data_object_new,
    // @clone ffi::data_object_clone,
    @delete ffi::vtk_data_object_delete
);

crate::inherit!(DataObject vtkDataObject ffi::vtkDataObject);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkDataObject`](https://vtk.org/doc/nightly/html/classvtkDataObject.html)
#[allow(non_camel_case_types)]
pub trait vtkDataObject: private::Sealed {
    #[doc(hidden)]
    fn as_vtk_data_object(&self) -> core::pin::Pin<&ffi::vtkDataObject>;
    #[doc(hidden)]
    fn as_vtk_data_object_mut(&mut self) -> core::pin::Pin<&mut ffi::vtkDataObject>;

    fn initialize(&mut self) {
        ffi::vtk_data_object_initialize(self.as_vtk_data_object_mut())
    }

    fn release_data(&mut self) {
        ffi::vtk_data_object_release_data(self.as_vtk_data_object_mut())
    }
}
