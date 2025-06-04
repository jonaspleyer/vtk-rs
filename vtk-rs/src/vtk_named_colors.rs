#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("vtk_named_colors.h");

        type vtkNamedColors;

        fn named_colors_new() -> *mut vtkNamedColors;
        fn named_colors_delete(named_colors: Pin<&mut vtkNamedColors>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkNamedColors.html",
    @name NamedColors, ffi::vtkNamedColors,
    @new ffi::named_colors_new,
    @delete ffi::named_colors_delete,
    @inherit vtkObject
);
