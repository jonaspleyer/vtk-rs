#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_named_colors.h");

        type vtkNamedColors;

        fn named_colors_new() -> *mut vtkNamedColors;
        unsafe fn named_colors_delete(named_colors: *mut vtkNamedColors);
        // unsafe fn named_colors_set_input(pdm: *mut vtkNamedColors, data: vtkPolyData);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkNamedColors.html",
    @name NamedColors, *mut ffi::vtkNamedColors,
    @new ffi::named_colors_new,
    @delete ffi::named_colors_delete
);

crate::inherit!(NamedColors vtkAlgorithm);
