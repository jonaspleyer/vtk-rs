macro_rules! define_object(
    (
        $link:literal,
        @name $name:ident, $ptr_type:ty,
        @new $new_func:expr,
        $(@clone $clone_func:expr,)?
        @delete $drop_func:expr
    ) => {
        #[doc = concat!("[`vtk", stringify!($name), "`](", $link, ")")]
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

macro_rules! inherit(
    ($name:ident vtkObjectBase $ptr_type:ty) => {};
    ($name:ident vtkObject $ptr_type:ty) => {
        crate::inherit!($name vtkObjectBase $ptr_type);
        impl $name {

        }
        impl core::convert::AsRef<crate::vtk_object::ffi::vtkObject> for $ptr_type {
            fn as_ref(&self) -> &crate::vtk_object::ffi::vtkObject {
                let x = self as *const $ptr_type;
                let x = x as *const crate::vtk_object::ffi::vtkObject;
                unsafe { &*x }
            }
        }

        impl core::convert::AsMut<crate::vtk_object::ffi::vtkObject> for $ptr_type {
            fn as_mut(&mut self) -> &mut crate::vtk_object::ffi::vtkObject {
                let x = self as *mut $ptr_type;
                let x = x as *mut crate::vtk_object::ffi::vtkObject;
                unsafe { &mut *x }
            }
        }

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
        impl crate::vtk_poly_data_algorithm::vtkPolyDataAlgorithm for $name {}
        crate::inherit!($name vtkAlgorithm $ptr_type);
    };
    ($name:ident vtkDataObject $ptr_type:ty) => {
        impl crate::vtk_data_object::private::Sealed for $name {}
        impl crate::vtk_data_object::vtkDataObject for $name {}
        crate::inherit!($name vtkObject $ptr_type);
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
pub(crate) use inherit;
