#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("poly_data_mapper.h");

        type vtkPolyDataMapper;

        fn poly_data_mapper_new() -> *mut vtkPolyDataMapper;
        unsafe fn poly_data_mapper_delete(pdm: *mut vtkPolyDataMapper);
        // fn poly_data_mapper_set_input_connection(pdm: &vtkPolyDataMapper, port: usize);
    }
}

/// Wraps the [`vtkPolyDataMapper`](https://vtk.org/doc/nightly/html/classvtkPolyDataMapper.html)
/// class.
pub struct PolyDataMapper {
    ptr: *mut ffi::vtkPolyDataMapper,
}

impl PolyDataMapper {
    pub fn new() -> Self {
        Self {
            ptr: ffi::poly_data_mapper_new(),
        }
    }
}

impl Default for PolyDataMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PolyDataMapper {
    fn drop(&mut self) {
        unsafe { ffi::poly_data_mapper_delete(self.ptr) };
    }
}

#[test]
fn test_create_drop() {
    let pdm = PolyDataMapper::new();
    core::mem::drop(pdm);
}
