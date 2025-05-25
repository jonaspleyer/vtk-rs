#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_poly_data_mapper.h");

        type vtkPolyDataMapper;

        fn poly_data_mapper_new() -> *mut vtkPolyDataMapper;
        unsafe fn poly_data_mapper_delete(pdm: *mut vtkPolyDataMapper);
        // fn poly_data_mapper_set_input_connection(pdm: &vtkPolyDataMapper, port: usize);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkPolyDataMapper.html",
    @name PolyDataMapper, *mut ffi::vtkPolyDataMapper,
    @new ffi::poly_data_mapper_new,
    @delete ffi::poly_data_mapper_delete
);

#[test]
fn test_create_drop() {
    let pdm = PolyDataMapper::new();
    core::mem::drop(pdm);
}
