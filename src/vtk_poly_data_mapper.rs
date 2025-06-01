#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_poly_data_mapper.h");

        type vtkPolyDataMapper;

        fn poly_data_mapper_new() -> *mut vtkPolyDataMapper;
        fn poly_data_mapper_delete(pdm: Pin<&mut vtkPolyDataMapper>);
        // fn poly_data_mapper_set_input_connection(pdm: &vtkPolyDataMapper, port: usize);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkPolyDataMapper.html",
    @name PolyDataMapper, ffi::vtkPolyDataMapper,
    @new ffi::poly_data_mapper_new,
    // @clone ffi::poly_data_mapper_clone,
    @delete ffi::poly_data_mapper_delete
);

crate::inherit!(PolyDataMapper vtkPolyDataMapper ffi::vtkPolyDataMapper);

/* impl PolyData {
    pub fn set_input(&mut self, data: &ffi::vtkPolyData) {
        unsafe { ffi::poly_data_mapper_set_input(self.ptr, data) };
    }
}*/

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkPolyDataMapper`](https://vtk.org/doc/nightly/html/classvtkPolyDataMapper.html)
#[allow(non_camel_case_types)]
pub trait vtkPolyDataMapper: private::Sealed {}

#[test]
fn test_create_drop() {
    let pdm = PolyDataMapper::new();
    core::mem::drop(pdm);
}

#[test]
fn test_input() {
    let mut pdm = PolyDataMapper::new();
    // pdm.set_input(2);
}
