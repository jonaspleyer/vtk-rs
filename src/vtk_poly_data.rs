#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_poly_data.h");

        type vtkPolyData;

        pub(crate) fn poly_data_new() -> *mut vtkPolyData;
        pub(crate) fn poly_data_delete(ptr: Pin<&mut vtkPolyData>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkNamedColors.html",
    @name PolyData, ffi::vtkPolyData,
    @new ffi::poly_data_new,
    // @clone ffi::poly_data_clone,
    @delete ffi::poly_data_delete,
    @inherit vtkPolyData
);

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkPolyData`](https://vtk.org/doc/nightly/html/classvtkPolyData.html)
#[allow(non_camel_case_types)]
pub trait vtkPolyData: private::Sealed {}
