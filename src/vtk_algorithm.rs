#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_algorithm.h");

        pub type vtkAlgorithm;

        fn algorithm_new() -> *mut vtkAlgorithm;
        fn algorithm_delete(algorithm: Pin<&mut vtkAlgorithm>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAlgorithm.html",
    @name Algorithm, ffi::vtkAlgorithm,
    @new ffi::algorithm_new,
    @delete ffi::algorithm_delete
);

crate::inherit!(Algorithm vtkAlgorithm ffi::vtkAlgorithm);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkAlgorithm`](https://vtk.org/doc/nightly/html/classvtkAlgorithm.html)
#[allow(non_camel_case_types)]
pub trait vtkAlgorithm: private::Sealed {}
