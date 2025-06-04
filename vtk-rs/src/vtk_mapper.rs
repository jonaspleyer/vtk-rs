#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {}
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkMapper`](https://vtk.org/doc/nightly/html/classvtkMapper.html)
#[allow(non_camel_case_types)]
pub trait vtkMapper: private::Sealed {}
