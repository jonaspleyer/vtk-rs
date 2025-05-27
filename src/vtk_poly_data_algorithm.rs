pub(crate) mod ffi {
    unsafe extern "C" {}
}

#[cxx::bridge]
pub(crate) mod cxx_ffi {
    unsafe extern "C++" {
        include!("vtk_poly_data_algorithm.h");

        pub(crate) type vtkPolyDataAlgorithm;
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkPolyDataAlgorithm`](https://vtk.org/doc/nightly/html/classvtkPolyDataAlgorithm.html)
#[allow(non_camel_case_types)]
pub trait vtkPolyDataAlgorithm: private::Sealed {
    fn is_a(&self, ttype: &str) -> i32;
    fn print_self(&self, indent: usize) -> String;
    // fn process_request(&self, information: &Information, i1: &InformationVector, i2: &InformationVector) -> bool;
    fn get_input(&self, port: Option<i32>) -> crate::vtk_data_object::DataObject;
    fn get_poly_data_input(&self, port: i32) -> crate::vtk_poly_data::PolyData;
    fn get_output(&self, port: Option<i32>) -> crate::vtk_poly_data::PolyData;
    fn set_output(&mut self, object: crate::vtk_data_object::DataObject);
    fn set_input_data(&mut self, id: Option<i32>, object: crate::vtk_data_object::DataObject);
    fn add_input_data(&mut self, id: Option<i32>, object: crate::vtk_data_object::DataObject);
}
