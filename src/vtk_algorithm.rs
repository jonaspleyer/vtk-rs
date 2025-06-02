#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_algorithm.h");

        pub type vtkAlgorithm;

        fn vtk_algorithm_new() -> *mut vtkAlgorithm;
        fn vtk_algorithm_delete(algorithm: Pin<&mut vtkAlgorithm>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAlgorithm.html",
    @name Algorithm, ffi::vtkAlgorithm,
    @new ffi::vtk_algorithm_new,
    @delete ffi::vtk_algorithm_delete
);

crate::inherit!(Algorithm vtkAlgorithm ffi::vtkAlgorithm);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkAlgorithm`](https://vtk.org/doc/nightly/html/classvtkAlgorithm.html)
#[allow(non_camel_case_types)]
pub trait vtkAlgorithm: private::Sealed {
    #[doc(hidden)]
    fn as_vtk_algorithm(&self) -> core::pin::Pin<&ffi::vtkAlgorithm>;
    #[doc(hidden)]
    fn as_vtk_algorithm_mut(&mut self) -> core::pin::Pin<&mut ffi::vtkAlgorithm>;
}
