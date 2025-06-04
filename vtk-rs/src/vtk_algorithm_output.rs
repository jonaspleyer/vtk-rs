#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_algorithm_output.h");

        pub type vtkAlgorithm;
        pub type vtkAlgorithmOutput;

        fn vtk_algorithm_output_new() -> *mut vtkAlgorithmOutput;
        fn vtk_algorithm_output_delete(algorithm_output: Pin<&mut vtkAlgorithmOutput>);

        fn vtk_algorithm_output_set_index(
            algorithm_output: Pin<&mut vtkAlgorithmOutput>,
            index: i64,
        );
        fn vtk_algorithm_output_get_index(algorithm_output: &vtkAlgorithmOutput) -> i64;
        fn vtk_algorithm_output_get_producer(
            algorithm_output: &vtkAlgorithmOutput,
        ) -> &vtkAlgorithm;
        fn vtk_algorithm_output_set_producer(
            algorithm_output: Pin<&mut vtkAlgorithmOutput>,
            producer: &vtkAlgorithm,
        );
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAlgorithmOutput.html",
    @name AlgorithmOutput, ffi::vtkAlgorithmOutput,
    @new ffi::vtk_algorithm_output_new,
    @delete ffi::vtk_algorithm_output_delete,
    @inherit vtkAlgorithmOutput
);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkAlgorithmOutput`](https://vtk.org/doc/nightly/html/classvtkAlgorithmOutput.html)
#[allow(non_camel_case_types)]
pub trait vtkAlgorithmOutput: private::Sealed {
    #[doc(hidden)]
    fn as_vtk_algorithm_output(&self) -> core::pin::Pin<&ffi::vtkAlgorithmOutput>;
    #[doc(hidden)]
    fn as_vtk_algorithm_output_mut(&mut self) -> core::pin::Pin<&mut ffi::vtkAlgorithmOutput>;

    #[doc(alias = "SetIndex")]
    fn set_index(&mut self, index: i64) {
        ffi::vtk_algorithm_output_set_index(self.as_vtk_algorithm_output_mut(), index);
    }

    #[doc(alias = "GetIndex")]
    fn get_index(&self) -> i64 {
        ffi::vtk_algorithm_output_get_index(&self.as_vtk_algorithm_output())
    }

    #[doc(alias = "GetProducer")]
    fn get_producer(&self) -> Option<&crate::Algorithm> {
        let sself = self.as_vtk_algorithm_output();
        let x = ffi::vtk_algorithm_output_get_producer(&sself);
        let y = x as *const _ as *const crate::vtk_algorithm::Algorithm;
        unsafe { y.as_ref() }
    }

    #[doc(alias = "SetProducer")]
    fn set_producer(&mut self, producer: &impl crate::vtkAlgorithm) {
        let producer = &producer.as_vtk_algorithm() as *const _ as *const ffi::vtkAlgorithm;
        let sself = self.as_vtk_algorithm_output_mut();
        ffi::vtk_algorithm_output_set_producer(sself, unsafe { &*producer });
    }
}
