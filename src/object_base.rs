unsafe extern "C" {
    fn object_delete(object: *mut isize);
    fn object_fast_delete(object: *mut isize);
    fn object_new() -> *mut isize;
}

pub struct ObjectBase {
    // Raw pointer to the vtkObjectBase
    object: *mut isize,
}

#[allow(non_snake_case)]
impl ObjectBase {
    pub fn New() -> Self {
        Self {
            object: unsafe { object_new() },
        }
    }

    pub fn FastDelete(self) {
        unsafe { object_fast_delete(self.object) }
    }
}

impl Drop for ObjectBase {
    fn drop(&mut self) {
        unsafe { object_delete(self.object) };
    }
}
