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
            fn drop(&mut self) {
                #[allow(unused_unsafe)]
                unsafe { ($drop_func)(self.ptr) };
            }
        }
    }
);

pub(crate) use implement_class;
