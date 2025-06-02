macro_rules! define_object(
    (
        $link:literal,
        @name $name:ident, $ptr_type:ty,
        @new $new_func:expr,
        $(@clone $clone_func:expr,)?
        @delete $drop_func:expr
    ) => {
        #[doc = concat!("[`vtk", stringify!($name), "`](", $link, ")")]
        #[repr(transparent)]
        pub struct $name {
            ptr: core::pin::Pin<&'static mut $ptr_type>,
        }

        impl $name {
            #[doc(alias = "New")]
            pub fn new() -> Self {
                let pinned = unsafe { core::pin::Pin::new_unchecked(&mut *($new_func)()) };
                Self {
                    ptr: pinned,
                }
            }
        }

        impl core::default::Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl core::ops::Drop for $name {
            #[doc(alias = "Delete")]
            fn drop(&mut self) {
                ($drop_func)(self.ptr.as_mut());
            }
        }

        #[test]
        fn create_delete() {
            let obj1 = $name::new();
            let obj2 = $name::default();
            core::mem::drop(obj1);
            core::mem::drop(obj2);
        }

        $(impl core::clone::Clone for $name {
            fn clone(&self) -> Self {
                #[allow(unused_unsafe)]
                Self {
                    ptr: unsafe { ($clone_func)(&*self.ptr) }
                }
            }
        }

        #[test]
        fn clone() {
            let obj1 = $name::new();
            let _obj2 = obj1.clone();
        })?
    }
);

macro_rules! impl_as_ref_mut(
    ($pin_type:ty, $ptr_type:ty) => {
        impl core::convert::AsRef<$pin_type> for $ptr_type {
            fn as_ref(&self) -> &$pin_type {
                let x = self as *const $ptr_type;
                let x = x as *const $pin_type;
                unsafe { &*x }
            }
        }

        impl core::convert::AsMut<$pin_type> for $ptr_type {
            fn as_mut(&mut self) -> &mut $pin_type {
                let x = self as *mut $ptr_type;
                let x = x as *mut $pin_type;
                unsafe { &mut *x }
            }
        }


    };

);

macro_rules! inherit(
    ($name:ident vtkObjectBase $ptr_type:ty) => {
        crate::impl_as_ref_mut!(crate::vtk_object_base::ffi::vtkObjectBase, $ptr_type);

        impl crate::vtk_object_base::private::Sealed for $name {}
        impl crate::vtk_object_base::vtkObjectBase for $name {
            fn as_vtk_object_base(
                &self
            ) -> core::pin::Pin<&crate::vtk_object_base::ffi::vtkObjectBase> {
                unsafe { self.ptr.as_ref().map_unchecked(|x| x.as_ref()) }
            }

            fn as_vtk_object_base_mut(&mut self) ->
                core::pin::Pin<&mut crate::vtk_object_base::ffi::vtkObjectBase>
            {
                unsafe { self.ptr.as_mut().map_unchecked_mut(|x| x.as_mut()) }
            }
        }

        #[cfg(test)]
        mod vtk_object_base {
            use super::*;
            use crate::vtk_object_base::*;

            #[test]
            fn get_class_name() {
                let obj = $name::new();
                let class_name = obj.get_class_name();
                assert!(!class_name.is_empty());
            }

            #[cfg(feature = "v094")]
            #[test]
            fn get_object_description() {
                let obj = $name::new();
                let obj_descr = obj.get_object_description();
                assert!(!obj_descr.is_empty());
            }

            #[test]
            fn is_a() {
                let obj = $name::new();
                let res = obj.is_a(&stringify!($name));
                // This test does always pass as long as linking is fine
                // Can we actually include some logic in here?
                assert!(res || !res);
            }

            #[test]
            fn get_number_of_generations_from_base() {
                let obj = $name::new();
                let n_generations = obj.get_number_of_generations_from_base("very_wrong_name");
                assert!(n_generations < 0);
            }

            /* See Module vtk_object_base
            #[test]
            fn fast_delete() {
                let obj = $name::new();
                obj.fast_delete();
            }*/

            #[test]
            fn get_set_reference_count() {
                let mut obj = $name::new();
                let ref_count = obj.get_reference_count();
                assert!(ref_count > 0);
                obj.set_reference_count(ref_count + 1);
                let ref_count2 = obj.get_reference_count();
                assert!(ref_count2 == ref_count + 1);
            }

            #[test]
            fn is_in_memkind() {
                // Object should be in normal memkind by default
                let obj = $name::new();
                assert!(!obj.get_is_in_memkind());
            }

            #[test]
            fn print_self_header_trailer() {
                // Just for linking
                let obj = $name::new();
                let pself = obj.print_self(1);
                let pheader = obj.print_header(2);
                let ptrailer = obj.print_trailer(3);
                assert!(!pself.is_empty());
                assert!(!pheader.is_empty());
                assert!(!ptrailer.is_empty());
            }

            #[cfg(feature = "v094")]
            #[test]
            fn uses_garbage_collector() {
                // Just for linking
                let obj = $name::new();
                let res = obj.uses_garbage_collector();
                assert!(res || !res);
            }
        }
    };

    ($name:ident vtkObject $ptr_type:ty) => {
        crate::inherit!($name vtkObjectBase $ptr_type);
        impl $name {

        }
        crate::impl_as_ref_mut!(crate::vtk_object::ffi::vtkObject, $ptr_type);

        impl crate::vtk_object::private::Sealed for $name {}

        impl crate::vtk_object::vtkObject for $name {
            fn as_vtk_object(&self) -> core::pin::Pin<&crate::vtk_object::ffi::vtkObject> {
                unsafe { self.ptr.as_ref().map_unchecked(|x| x.as_ref()) }
            }

            fn as_vtk_object_mut(&mut self) ->
                core::pin::Pin<&mut crate::vtk_object::ffi::vtkObject>
            {
                unsafe { self.ptr.as_mut().map_unchecked_mut(|x| x.as_mut()) }
            }
        }

        #[cfg(test)]
        mod test_vtkobject {
            use super::*;
            use crate::vtkObject;

            #[test]
            fn debug_on_off() {
                let mut obj = $name::new();
                obj.set_debug(true);
                assert_eq!(true, obj.get_debug());
                obj.debug_off();
                assert_eq!(false, obj.get_debug());
                obj.debug_on();
                assert_eq!(true, obj.get_debug());
                obj.set_debug(false);
                assert_eq!(false, obj.get_debug());
            }

            #[test]
            fn observers() {
                let obj = $name::new();
                assert_eq!(obj.has_observer(0), 0);
                // TODO
                // obj.add_observer();
            }
        }
    };
    ($name:ident vtkAlgorithm $ptr_type:ty) => {
        impl crate::vtk_algorithm::private::Sealed for $name {}
        impl crate::vtk_algorithm::vtkAlgorithm for $name {}
        crate::inherit!($name vtkObject $ptr_type);
    };
    ($name:ident vtkPolyData $ptr_type:ty) => {
        impl crate::vtk_poly_data::private::Sealed for $name {}
        impl crate::vtk_poly_data::vtkPolyData for $name {}
        crate::inherit!($name vtkPointSet $ptr_type);
    };
    ($name:ident vtkPolyDataAlgorithm $ptr_type:ty) => {
        impl crate::vtk_poly_data_algorithm::private::Sealed for $name {}

        crate::impl_as_ref_mut!(crate::vtk_poly_data_algorithm::ffi::vtkPolyDataAlgorithm, $ptr_type);

        impl crate::vtk_poly_data_algorithm::vtkPolyDataAlgorithm for $name {
            fn as_vtk_poly_data_algorithm(&self) ->
                core::pin::Pin<&crate::vtk_poly_data_algorithm::ffi::vtkPolyDataAlgorithm> {
                unsafe { self.ptr.as_ref().map_unchecked(|x| x.as_ref()) }
            }

            fn as_vtk_poly_data_algorithm_mut(&mut self) ->
                core::pin::Pin<&mut crate::vtk_poly_data_algorithm::ffi::vtkPolyDataAlgorithm> {
                unsafe { self.ptr.as_mut().map_unchecked_mut(|x| x.as_mut()) }
            }

        }

        #[cfg(test)]
        mod vtk_poly_data_algorithm {
            use super::*;
            #[allow(unused)]
            use crate::vtk_poly_data_algorithm::*;

            #[test]
            fn create_delete() {
                let obj = $name::new();
                core::mem::drop(obj);
            }

            /* #[test]
            fn input_data() {
                use crate::vtk_data_object::*;
                use crate::vtk_algorithm::*;
                let mut pda = $name::new();
                // pda.set_number_of_input_ports(1);
                // assert!(pda.get_input(0).is_none());
                // let obj = DataObject::new();
                // pda.add_input_data(0, &obj);
                // let obj2 = DataObject::new();
                // pda.set_input_data(0, &obj2);
            }*/
        }

        crate::inherit!($name vtkAlgorithm $ptr_type);
    };
    ($name:ident vtkDataObject $ptr_type:ty) => {
        impl crate::vtk_data_object::private::Sealed for $name {}

        crate::impl_as_ref_mut!(crate::vtk_data_object::ffi::vtkDataObject, $ptr_type);

        impl crate::vtk_data_object::vtkDataObject for $name {
            fn as_vtk_data_object(&self) ->
                core::pin::Pin<&crate::vtk_data_object::ffi::vtkDataObject> {
                unsafe { self.ptr.as_ref().map_unchecked(|x| x.as_ref()) }
            }

            fn as_vtk_data_object_mut(&mut self) ->
                core::pin::Pin<&mut crate::vtk_data_object::ffi::vtkDataObject> {
                unsafe { self.ptr.as_mut().map_unchecked_mut(|x| x.as_mut()) }
            }

            unsafe fn cast_to_pointer<T>(&self) -> core::pin::Pin<&T> {
                self.ptr.as_ref().map_unchecked(|x| {
                    let x = x as *const $ptr_type;
                    let x = x as *const T;
                    unsafe { &*x }
                })
            }

            unsafe fn cast_to_pointer_mut<T>(&mut self) -> core::pin::Pin<&mut T> {
                self.ptr.as_mut().map_unchecked_mut(|x| {
                    let x = x as *mut $ptr_type;
                    let x = x as *mut T;
                    unsafe { &mut *x }
                })
            }
        }

        #[cfg(test)]
        mod vtk_data_object {
            use super::*;
            #[allow(unused)]
            use crate::vtk_data_object::*;

            #[test]
            fn initialize_release() {
                let mut obj = $name::new();
                obj.initialize();
                obj.release_data();
            }
        }

        crate::inherit!($name vtkObject $ptr_type);
    };
    ($name:ident vtkAbstractMapper $ptr_type:ty) => {
        impl crate::vtk_abstract_mapper::private::Sealed for $name {}
        impl crate::vtk_abstract_mapper::vtkAbstractMapper for $name {}

        crate::inherit!($name vtkAlgorithm $ptr_type);
    };
    ($name:ident vtkAbstractMapper3D $ptr_type:ty) => {
        crate::inherit!($name vtkAbstractMapper $ptr_type);
    };
    ($name:ident vtkMapper $ptr_type:ty) => {
        crate::inherit!($name vtkAbstractMapper3D $ptr_type);
    };
    ($name:ident vtkPolyDataMapper $ptr_type:ty) => {
        impl crate::vtk_poly_data_mapper::private::Sealed for $name {}
        impl crate::vtk_poly_data_mapper::vtkPolyDataMapper for $name {}

        crate::inherit!($name vtkMapper $ptr_type);
    };
    ($name:ident vtkDataSet $ptr_type:ty) => {
        crate::inherit!($name vtkDataObject $ptr_type);
    };
    ($name:ident vtkCommand $ptr_type:ty) => {
        crate::inherit!($name vtkObjectBase $ptr_type);
    };
    ($name:ident vtkImageData $ptr_type:ty) => {
        crate::inherit!($name vtkDataSet $ptr_type);
    };
    ($name:ident vtkPointSet $ptr_type:ty) => {
        crate::inherit!($name vtkDataSet $ptr_type);
    };
    ($name:ident vtkUnstructuredGridBase $ptr_type:ty) => {
        crate::inherit!($name vtkPointSet $ptr_type);
    };
    ($name:ident vtkGraph $ptr_type:ty) => {
        crate::inherit!($name vtkDataObject $ptr_type);
    };
    ($name:ident vtkImplicitFunction $ptr_type:ty) => {
        crate::inherit!($name vtkObject $ptr_type);
    };
);

pub(crate) use define_object;
pub(crate) use impl_as_ref_mut;
pub(crate) use inherit;
