pub mod object_base {
    unsafe extern "C" {
        fn object_delete(object: *mut isize);
        fn object_fast_delete(object: *mut isize);
        fn object_new() -> *mut isize;
    }

    pub struct ObjectBase {
        // Raw pointer to the vtkObjectBase
        object: *mut isize,
    }

    impl ObjectBase {
        #[allow(non_snake_case)]
        pub fn New() -> Self {
            Self {
                object: unsafe { object_new() },
            }
        }

        pub fn fast_delete_object(self) {
            unsafe { object_fast_delete(self.object) }
        }
    }

    impl Drop for ObjectBase {
        fn drop(&mut self) {
            println!("{:p}", self.object);
            unsafe { object_delete(self.object) };
        }
    }
}
