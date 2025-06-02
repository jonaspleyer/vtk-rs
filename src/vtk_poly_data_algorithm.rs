#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_poly_data_algorithm.h");

        pub(crate) type vtkPolyDataAlgorithm;
        pub(crate) type vtkDataObject;
        pub(crate) type vtkPolyData;

        fn vtk_poly_data_algorithm_new() -> *mut vtkPolyDataAlgorithm;
        fn vtk_poly_data_algorithm_delete(poly_data_algorithm: Pin<&mut vtkPolyDataAlgorithm>);
        fn vtk_poly_data_algorithm_get_input(
            vtk_poly_data_algorithm: &vtkPolyDataAlgorithm,
            port: i64,
        ) -> &vtkDataObject;
        fn vtk_poly_data_algorithm_get_poly_data_input(
            vtk_poly_data_algorithm: &vtkPolyDataAlgorithm,
            port: i64,
        ) -> &vtkPolyData;
        fn vtk_poly_data_algorithm_get_output(
            vtk_poly_data_algorithm: &vtkPolyDataAlgorithm,
            port: i64,
        ) -> &vtkPolyData;
        fn vtk_poly_data_algorithm_set_output(
            vtk_poly_data_algorithm: Pin<&mut vtkPolyDataAlgorithm>,
            data_object: &vtkDataObject,
        );
        fn vtk_poly_data_algorithm_set_input_data(
            poly_data_algorithm: Pin<&mut vtkPolyDataAlgorithm>,
            port: i64,
            data_object: &vtkDataObject,
        );
        fn vtk_poly_data_algorithm_add_input_data(
            poly_data_algorithm: Pin<&mut vtkPolyDataAlgorithm>,
            port: i64,
            data_object: &vtkDataObject,
        );
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkPolyDataAlgorithm.html",
    @name PolyDataAlgorithm, ffi::vtkPolyDataAlgorithm,
    @new ffi::vtk_poly_data_algorithm_new,
    // @clone ffi::data_object_clone,
    @delete ffi::vtk_poly_data_algorithm_delete
);

crate::inherit!(PolyDataAlgorithm vtkPolyDataAlgorithm ffi::vtkPolyDataAlgorithm);

/// [`vtkPolyDataAlgorithm`](https://vtk.org/doc/nightly/html/classvtkPolyDataAlgorithm.html)
#[allow(non_camel_case_types)]
pub trait vtkPolyDataAlgorithm: private::Sealed {
    #[doc(hidden)]
    fn as_vtk_poly_data_algorithm(&self) -> core::pin::Pin<&ffi::vtkPolyDataAlgorithm>;
    #[doc(hidden)]
    fn as_vtk_poly_data_algorithm_mut(&mut self) -> core::pin::Pin<&mut ffi::vtkPolyDataAlgorithm>;

    // fn process_request(
    //      &self,
    //      information: &crate::Information,
    //      i1: &crate::InformationVector,
    //      i2: &crate::InformationVector
    // ) -> bool;

    fn get_input(&self, port: i64) -> Option<&crate::DataObject> {
        let sself = self.as_vtk_poly_data_algorithm();
        let raw_ref = ffi::vtk_poly_data_algorithm_get_input(&sself, port);
        let raw_ptr = raw_ref as *const _ as *const crate::vtk_data_object::DataObject;
        unsafe { raw_ptr.as_ref() }
    }

    fn vtk_poly_data_algorithm_get_poly_data_input(&self, port: i64) -> Option<&crate::PolyData> {
        let sself = self.as_vtk_poly_data_algorithm();
        let raw_ref = ffi::vtk_poly_data_algorithm_get_poly_data_input(&sself, port);
        let raw_ptr = raw_ref as *const _ as *const crate::vtk_poly_data::PolyData;
        unsafe { raw_ptr.as_ref() }
    }

    fn get_poly_data_input(&self, port: i64) -> Option<&crate::vtk_poly_data::PolyData> {
        let sself = self.as_vtk_poly_data_algorithm();
        let raw_ref = ffi::vtk_poly_data_algorithm_get_poly_data_input(&sself, port);
        let raw_ptr = raw_ref as *const _ as *const crate::PolyData;
        unsafe { raw_ptr.as_ref() }
    }

    fn set_output(&mut self, data_object: &impl crate::vtkDataObject) {
        let dobject = &data_object.as_vtk_data_object() as *const _ as *const ffi::vtkDataObject;
        ffi::vtk_poly_data_algorithm_set_output(self.as_vtk_poly_data_algorithm_mut(), unsafe {
            &*dobject
        });
    }

    fn set_input_data(&mut self, port: i64, data_object: &impl crate::vtkDataObject) {
        let dobject = &data_object.as_vtk_data_object() as *const _ as *const ffi::vtkDataObject;
        ffi::vtk_poly_data_algorithm_set_input_data(
            self.as_vtk_poly_data_algorithm_mut(),
            port,
            unsafe { &*dobject },
        );
    }

    fn add_input_data(&mut self, port: i64, data_object: &impl crate::vtkDataObject) {
        let dobject = &data_object.as_vtk_data_object() as *const _ as *const ffi::vtkDataObject;
        ffi::vtk_poly_data_algorithm_add_input_data(
            self.as_vtk_poly_data_algorithm_mut(),
            port,
            unsafe { &*dobject },
        );
    }

    fn get_output(&self, port: i64) -> Option<&crate::PolyData> {
        let sself = self.as_vtk_poly_data_algorithm();
        let raw_ref = ffi::vtk_poly_data_algorithm_get_output(&sself, port);
        let raw_ptr = raw_ref as *const _ as *const crate::PolyData;
        unsafe { raw_ptr.as_ref() }
    }
}
