#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("vtk_object_base.h");

        type vtkObjectBase;

        fn vtk_object_base_get_class_name(obj: &vtkObjectBase) -> String;
        fn vtk_object_base_get_object_description(obj: &vtkObjectBase) -> String;
        fn vtk_object_base_is_a(obj: &vtkObjectBase, class_name: &str) -> bool;
        fn vtk_object_base_get_number_of_generations_from_base(
            obj: &vtkObjectBase,
            base_class: &str,
        ) -> i64;
        fn vtk_object_base_fast_delete(obj: Pin<&mut vtkObjectBase>);
        fn vtk_object_base_get_reference_count(obj: &vtkObjectBase) -> i64;
        fn vtk_object_base_set_reference_count(obj: Pin<&mut vtkObjectBase>, count: i64);
        fn vtk_object_base_get_is_in_memkind(obj: &vtkObjectBase) -> bool;
        fn vtk_object_base_print_self(obj: &vtkObjectBase, indent: u64) -> String;
        fn vtk_object_base_print_header(obj: &vtkObjectBase, indent: u64) -> String;
        fn vtk_object_base_print_trailer(obj: &vtkObjectBase, indent: u64) -> String;
        fn vtk_object_base_uses_garbage_collector(obj: &vtkObjectBase) -> bool;
    }
}

pub(crate) mod private {
    pub trait Sealed {}
}

/// [`vtkObjectBase`](https://vtk.org/doc/nightly/html/classvtkObjectBase.html)
#[allow(non_camel_case_types)]
pub trait vtkObjectBase: private::Sealed {
    fn as_vtk_object_base(&self) -> core::pin::Pin<&ffi::vtkObjectBase>;
    fn as_vtk_object_base_mut(&mut self) -> core::pin::Pin<&mut ffi::vtkObjectBase>;

    fn get_class_name(&self) -> String {
        ffi::vtk_object_base_get_class_name(&self.as_vtk_object_base())
    }

    #[cfg(feature = "v9.4")]
    fn get_object_description(&self) -> String {
        ffi::vtk_object_base_get_object_description(&self.as_vtk_object_base())
    }

    fn is_a(&self, class_name: &str) -> bool {
        ffi::vtk_object_base_is_a(&self.as_vtk_object_base(), class_name)
    }

    fn get_number_of_generations_from_base(&self, base_class: &str) -> i64 {
        ffi::vtk_object_base_get_number_of_generations_from_base(
            &self.as_vtk_object_base(),
            base_class,
        )
    }

    /* TODO this is currently not working yet
    * fn fast_delete(self)
    where
        Self: Sized + Drop,
    {
        let mut sself = self;
        let pin = sself.as_vtk_object_base_mut();
        ffi::vtk_object_base_fast_delete(pin);
        core::mem::drop(sself);
    }*/

    fn get_reference_count(&self) -> i64 {
        ffi::vtk_object_base_get_reference_count(&self.as_vtk_object_base())
    }

    fn set_reference_count(&mut self, count: i64) {
        ffi::vtk_object_base_set_reference_count(self.as_vtk_object_base_mut(), count)
    }

    fn get_is_in_memkind(&self) -> bool {
        ffi::vtk_object_base_get_is_in_memkind(&self.as_vtk_object_base())
    }

    fn print_self(&self, indent: u64) -> String {
        ffi::vtk_object_base_print_self(&self.as_vtk_object_base(), indent)
    }

    fn print_header(&self, indent: u64) -> String {
        ffi::vtk_object_base_print_header(&self.as_vtk_object_base(), indent)
    }

    fn print_trailer(&self, indent: u64) -> String {
        ffi::vtk_object_base_print_trailer(&self.as_vtk_object_base(), indent)
    }

    #[cfg(feature = "v9.4")]
    fn uses_garbage_collector(&self) -> bool {
        ffi::vtk_object_base_uses_garbage_collector(&self.as_vtk_object_base())
    }
}
