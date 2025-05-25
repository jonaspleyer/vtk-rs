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
            ptr: $ptr_type,
        }

        impl $name {
            #[doc(alias = "New")]
            pub fn new() -> Self {
                Self {
                    ptr: ($new_func)()
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
                #[allow(unused_unsafe)]
                unsafe { ($drop_func)(self.ptr) };
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
    ($name:ident vtkObjectBase) => {};
    ($name:ident vtkObject) => {
        crate::inherit!($name vtkObjectBase);

        impl crate::vtk_object::private::Sealed for $name {}

        impl crate::vtk_object::Object for $name {
            #[doc(alias = "SetDebug")]
            fn set_debug(&mut self, status: bool) {
                unsafe { crate::vtk_object::ffi::set_debug(
                    self.ptr as *mut std::ffi::c_void,
                    status
                ) };
            }

            #[doc(alias = "GetDebug")]
            fn get_debug(&self) -> bool {
                unsafe {crate::vtk_object::ffi::get_debug(
                    self.ptr as *const std::ffi::c_void,
                )}
            }

            #[doc(alias = "DebugOn")]
            fn debug_on(&mut self) {
                unsafe {crate::vtk_object::ffi::debug_on(
                    self.ptr as *mut std::ffi::c_void,
                )}
            }

            #[doc(alias = "DebugOff")]
            fn debug_off(&mut self) {
                unsafe {crate::vtk_object::ffi::debug_off(
                    self.ptr as *mut std::ffi::c_void,
                )}
            }

            #[doc(alias = "Modified")]
            fn modified(&mut self) {
                unsafe {crate::vtk_object::ffi::modified(
                    self.ptr as *mut std::ffi::c_void,
                )}
            }

            #[doc(alias = "RemoveObserver")]
            fn remove_observer(&mut self, tag: u64) {
                unsafe {crate::vtk_object::ffi::remove_observer(
                    self.ptr as *mut std::ffi::c_void,
                    tag,
                )}
            }

            #[doc(alias = "RemoveObservers")]
            fn remove_observers(&mut self, event: u64) {
                unsafe {crate::vtk_object::ffi::remove_observers(
                    self.ptr as *mut std::ffi::c_void,
                    event,
                )}
            }

            #[doc(alias = "RemoveAllObservers")]
            fn remove_all_observers(&mut self) {
                unsafe {crate::vtk_object::ffi::remove_all_observers(
                    self.ptr as *mut std::ffi::c_void,
                )}
            }

            #[doc(alias = "HasObserver")]
            fn has_observer(&self, event: core::ffi::c_ulong) -> core::ffi::c_int {
                unsafe {crate::vtk_object::ffi::has_observer(
                    self.ptr as *const std::ffi::c_void,
                    event,
                )}
            }
        }

        #[cfg(test)]
        mod test_vtkobject {
            use super::*;
            use crate::Object;

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
    ($name:ident vtkAlgorithm) => {
        crate::inherit!($name vtkObject);
    };
    ($name:ident vtkPolyData) => {
        crate::inherit!($name vtkPointSet);
    };
    ($name:ident vtkPolyDataAlgorithm) => {
        crate::inherit!($name vtkAlgorithm);
    };
    ($name:ident vtkDataObject) => {
        crate::inherit!($name vtkObject);
    };
    ($name:ident vtkDataSet) => {
        crate::inherit!($name vtkDataObject);
    };
    ($name:ident vtkCommand) => {
        crate::inherit!($name vtkObjectBase);
    };
    ($name:ident vtkImageData) => {
        crate::inherit!($name vtkDataSet);
    };
    ($name:ident vtkPointSet) => {
        crate::inherit!($name vtkDataSet);
    };
    ($name:ident vtkUnstructuredGridBase) => {
        crate::inherit!($name vtkPointSet);
    };
    ($name:ident vtkGraph) => {
        crate::inherit!($name vtkDataObject);
    }
);

pub(crate) use define_object;
pub(crate) use inherit;
