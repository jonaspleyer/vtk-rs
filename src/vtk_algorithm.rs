#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_algorithm.h");

        pub type vtkAlgorithm;
        pub type vtkAlgorithmOutput;
        pub type vtkInformation;
        pub type vtkInformationVector;
        pub type vtkExecutive;

        fn vtk_algorithm_new() -> *mut vtkAlgorithm;
        fn vtk_algorithm_delete(algorithm: Pin<&mut vtkAlgorithm>);

        fn vtk_algorithm_set_input_connection(
            algorithm: Pin<&mut vtkAlgorithm>,
            port: i64,
            input: &vtkAlgorithmOutput,
        );
        fn vtk_algorithm_has_executive(algorithm: &vtkAlgorithm) -> bool;
        fn vtk_algorithm_get_executive(algorithm: &vtkAlgorithm) -> &vtkExecutive;
        fn vtk_algorithm_set_executive(
            algorithm: Pin<&mut vtkAlgorithm>,
            executive: Pin<&mut vtkExecutive>,
        );
        unsafe fn vtk_algorithm_process_request(
            algorithm: &vtkAlgorithm,
            request: &vtkInformation,
            in_info: &mut [*mut vtkInformationVector],
            out_info: Pin<&mut vtkInformationVector>,
        ) -> bool;
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkAlgorithm.html",
    @name Algorithm, ffi::vtkAlgorithm,
    @new ffi::vtk_algorithm_new,
    @delete ffi::vtk_algorithm_delete,
    @inherit vtkAlgorithm
);

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

    fn has_executive(&self) -> bool {
        ffi::vtk_algorithm_has_executive(&self.as_vtk_algorithm())
    }

    fn get_executive(&self) -> Option<&crate::vtk_executive::Executive> {
        // let sself = self.as_vtk_algorithm();
        let x = ffi::vtk_algorithm_get_executive(&self.as_vtk_algorithm()) as *const _
            as *const crate::vtk_executive::Executive;
        unsafe { x.as_ref() }
    }

    fn set_executive(&mut self, executive: &mut impl crate::vtk_executive::vtkExecutive) {
        let executive = unsafe {
            executive
                .as_vtk_executive_mut()
                .map_unchecked_mut(|x| &mut *(x as *mut _ as *mut ffi::vtkExecutive))
        };
        let sself = self.as_vtk_algorithm_mut();
        ffi::vtk_algorithm_set_executive(sself, executive);
    }

    fn set_input_connection(&mut self, port: i64, input: &impl crate::vtkAlgorithmOutput) {
        let sself = self.as_vtk_algorithm_mut();
        let input = &input.as_vtk_algorithm_output() as *const _ as *const ffi::vtkAlgorithmOutput;
        ffi::vtk_algorithm_set_input_connection(sself, port, unsafe { &*input });
    }
}
