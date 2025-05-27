#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_data_object.h");

        pub type vtkDataObject;

        pub(crate) fn data_object_new() -> *mut vtkDataObject;
        pub(crate) fn data_object_delete(data_object: Pin<&mut vtkDataObject>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkNamedColors.html",
    @name DataObject, ffi::vtkDataObject,
    @new ffi::data_object_new,
    // @clone ffi::data_object_clone,
    @delete ffi::data_object_delete
);

crate::inherit!(DataObject vtkDataObject ffi::vtkDataObject);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkDataObject`](https://vtk.org/doc/nightly/html/classvtkDataObject.html)
#[allow(non_camel_case_types)]
pub trait vtkDataObject: private::Sealed {}
