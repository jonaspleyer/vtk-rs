#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("vtk_data_object.h");

        pub type vtkDataObject;

        pub(crate) fn data_object_new() -> *mut vtkDataObject;
        pub(crate) unsafe fn data_object_delete(data_object: *mut vtkDataObject);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkNamedColors.html",
    @name DataObject, *mut ffi::vtkDataObject,
    @new ffi::data_object_new,
    // @clone ffi::data_object_clone,
    @delete ffi::data_object_delete
);

crate::inherit!(DataObject vtkDataObject);
