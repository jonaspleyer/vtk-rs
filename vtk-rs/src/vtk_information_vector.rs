#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_information_vector.h");

        pub(crate) type vtkInformationVector;

        fn vtk_information_vector_new() -> *mut vtkInformationVector;
        fn vtk_information_vector_delete(information_vector: Pin<&mut vtkInformationVector>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkInformation.html",
    @name InformationVector, ffi::vtkInformationVector,
    @new ffi::vtk_information_vector_new,
    @delete ffi::vtk_information_vector_delete,
    @inherit vtkObject
);
