#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_abstract_mapper_3d.h");
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkAbstractMapper3D`](https://vtk.org/doc/nightly/html/classvtkAbstractMapper3D.html)
#[allow(non_camel_case_types)]
pub trait vtkAbstractMapper3D: private::Sealed {}
