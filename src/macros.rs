macro_rules! implement_class(
    (
        $link:literal,
        @name $name:ident, $ptr_type:ty,
        @new $new_func:expr,
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
    }
);

pub(crate) use implement_class;
