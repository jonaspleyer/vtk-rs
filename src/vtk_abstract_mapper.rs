#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_abstract_mapper.h");
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkAbstractMapper`](https://vtk.org/doc/nightly/html/classvtkAbstractMapper.html)
#[allow(non_camel_case_types)]
pub trait vtkAbstractMapper: private::Sealed {}
