#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_information.h");

        pub(crate) type vtkInformation;

        fn vtk_information_new() -> *mut vtkInformation;
        fn vtk_information_delete(information: Pin<&mut vtkInformation>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkInformation.html",
    @name Information, ffi::vtkInformation,
    @new ffi::vtk_information_new,
    @delete ffi::vtk_information_delete,
    @inherit vtkObject
);
