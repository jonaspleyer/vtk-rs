#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_algorithm.h");

        pub type vtkAlgorithm;

        fn algorithm_new() -> *mut vtkAlgorithm;
        unsafe fn algorithm_delete(algorithm: *mut vtkAlgorithm);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAlgorithm.html",
    @name Algorithm, *mut ffi::vtkAlgorithm,
    @new ffi::algorithm_new,
    @delete ffi::algorithm_delete
);

crate::inherit!(Algorithm vtkAlgorithm);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkAlgorithm`](https://vtk.org/doc/nightly/html/classvtkAlgorithm.html)
#[allow(non_camel_case_types)]
pub trait vtkAlgorithm: private::Sealed {}
