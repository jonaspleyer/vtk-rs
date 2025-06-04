#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_information.h");

        pub(crate) type vtkInformation;

        fn vtk_information_new() -> *mut vtkInformation;
        fn vtk_information_delete(information: Pin<&mut vtkInformation>);
    }
}

crate::define_object!(
    "https://vtk.org/doc/nightly/html/classvtkInformation.html",
    @name Information, ffi::vtkInformation,
    @new ffi::vtk_information_new,
    @delete ffi::vtk_information_delete,
    @inherit vtkObject
);

crate::impl_as_ref_mut!(Information, ffi::vtkInformation);

impl Information {
    pub(crate) fn as_vtk_information(&self) -> core::pin::Pin<&Self> {
        unsafe { self.ptr.as_ref().map_unchecked(|x| x.as_ref()) }
    }

    pub(crate) fn as_vtk_information_mut(&mut self) -> core::pin::Pin<&mut Self> {
        unsafe { self.ptr.as_mut().map_unchecked_mut(|x| x.as_mut()) }
    }
}
