/// helper object to manage setting and generating contour values
///
///
/// vtkContourValues is a general class to manage the creation, generation,
/// and retrieval of contour values. This class serves as a helper class for
/// contouring classes, or those classes operating on lists of contour values.
///
/// @sa
/// vtkContourFilter
#[allow(non_camel_case_types)]
pub struct vtkContourValues(*mut core::ffi::c_void);
impl vtkContourValues {
    /// Creates a new [vtkContourValues] wrapped inside `vtkNew`
    #[doc(alias = "vtkContourValues")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkContourValues_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkContourValues_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkContourValues_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkContourValues_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkContourValues {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkContourValues {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkContourValues_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkContourValues_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkContourValues_create_drop() {
    let obj = vtkContourValues::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkContourValues(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Parse and evaluate a mathematical expression
///
///
/// vtkExprTkFunctionParser is a wrapper class of the ExprTK library that takes
/// in a mathematical expression as a char string, parses it, and evaluates it
/// at the specified values of the variables in the input string.
///
/// The detailed documentation of the supported functionality is described in
/// https://github.com/ArashPartow/exprtk. In addition to the documented
/// functionality, the following vector operations have been implemented:
/// 1) cross(v1, v2), cross product of two vectors,
/// 2) mag(v), magnitude of a vector,
/// 3) norm(v), the normalized version of a vector.
///
/// @par Thanks:
/// Arash Partow for implementing the ExprTk library.
#[allow(non_camel_case_types)]
pub struct vtkExprTkFunctionParser(*mut core::ffi::c_void);
impl vtkExprTkFunctionParser {
    /// Creates a new [vtkExprTkFunctionParser] wrapped inside `vtkNew`
    #[doc(alias = "vtkExprTkFunctionParser")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExprTkFunctionParser_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkExprTkFunctionParser_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkExprTkFunctionParser_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkExprTkFunctionParser_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkExprTkFunctionParser {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkExprTkFunctionParser {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkExprTkFunctionParser_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkExprTkFunctionParser_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkExprTkFunctionParser_create_drop() {
    let obj = vtkExprTkFunctionParser::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkExprTkFunctionParser(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// Parse and evaluate a mathematical expression
///
///
/// vtkFunctionParser is a class that takes in a mathematical expression as
/// a char string, parses it, and evaluates it at the specified values of
/// the variables in the input string.
///
/// You can use the "if" operator to create conditional expressions
/// such as if ( test, trueresult, falseresult). These evaluate the boolean
/// valued test expression and then evaluate either the trueresult or the
/// falseresult expression to produce a final (scalar or vector valued) value.
/// "test" may contain <,>,=,|,&, and () and all three subexpressions can
/// evaluate arbitrary function operators (ln, cos, +, if, etc)
///
/// @par Thanks:
/// Juha Nieminen (juha.nieminen@gmail.com) for relicensing this branch of the
/// function parser code that this class is based upon under the new BSD license
/// so that it could be used in VTK. Note, the BSD license applies to this
/// version of the function parser only (by permission of the author), and not
/// the original library.
///
/// @par Thanks:
/// Thomas Dunne (thomas.dunne@iwr.uni-heidelberg.de) for adding code for
/// two-parameter-parsing and a few functions (sign, min, max).
///
/// @par Thanks:
/// Sid Sydoriak (sxs@lanl.gov) for adding boolean operations and
/// conditional expressions and for fixing a variety of bugs.
#[allow(non_camel_case_types)]
pub struct vtkFunctionParser(*mut core::ffi::c_void);
impl vtkFunctionParser {
    /// Creates a new [vtkFunctionParser] wrapped inside `vtkNew`
    #[doc(alias = "vtkFunctionParser")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFunctionParser_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkFunctionParser_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkFunctionParser_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkFunctionParser_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkFunctionParser {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFunctionParser {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFunctionParser_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFunctionParser_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFunctionParser_create_drop() {
    let obj = vtkFunctionParser::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkFunctionParser(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// replacement for malloc/free and new/delete
///
///
/// This class is a replacement for malloc/free and new/delete for software
/// that has inherent memory leak or performance problems. For example,
/// external software such as the PLY library (vtkPLY) and VRML importer
/// (vtkVRMLImporter) are often written with lots of malloc() calls but
/// without the corresponding free() invocations. The class
/// vtkOrderedTriangulator may create and delete millions of new/delete calls.
/// This class allows the overloading of the C++ new operator (or other memory
/// allocation requests) by using the method AllocateMemory(). Memory is
/// deleted with an invocation of CleanAll() (which deletes ALL memory; any
/// given memory allocation cannot be deleted). Note: a block size can be used
/// to control the size of each memory allocation. Requests for memory are
/// fulfilled from the block until the block runs out, then a new block is
/// created.
///
/// @warning
/// Do not use this class as a general replacement for system memory
/// allocation.  This class should be used only as a last resort if memory
/// leaks cannot be tracked down and eliminated by conventional means. Also,
/// deleting memory from vtkHeap is not supported. Only the deletion of
/// the entire heap is. (A Reset() method allows you to reuse previously
/// allocated memory.)
///
/// @sa
/// vtkVRMLImporter vtkPLY vtkOrderedTriangulator
#[allow(non_camel_case_types)]
pub struct vtkHeap(*mut core::ffi::c_void);
impl vtkHeap {
    /// Creates a new [vtkHeap] wrapped inside `vtkNew`
    #[doc(alias = "vtkHeap")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHeap_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkHeap_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkHeap_get_ptr(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtkHeap_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkHeap {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHeap {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHeap_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkHeap_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHeap_create_drop() {
    let obj = vtkHeap::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkHeap(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// utility to locate resource files.
///
///
/// VTK based application often need to locate resource files, such configuration
/// files, Python modules, etc. vtkResourceFileLocator provides methods that can
/// be used to locate such resource files at runtime.
///
/// Using `Locate`, one can locate files relative to an
/// anchor directory such as the executable directory, or the library directory.
///
/// `GetLibraryPathForSymbolUnix` and `GetLibraryPathForSymbolWin32` methods can
/// be used to locate the library that provides a particular symbol. For example,
/// this is used by `vtkPythonInterpreter` to ensure that the `vtk` Python package
/// is located relative the VTK libraries, irrespective of the application location.
#[allow(non_camel_case_types)]
pub struct vtkResourceFileLocator(*mut core::ffi::c_void);
impl vtkResourceFileLocator {
    /// Creates a new [vtkResourceFileLocator] wrapped inside `vtkNew`
    #[doc(alias = "vtkResourceFileLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkResourceFileLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkResourceFileLocator_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkResourceFileLocator_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkResourceFileLocator_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkResourceFileLocator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkResourceFileLocator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkResourceFileLocator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkResourceFileLocator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkResourceFileLocator_create_drop() {
    let obj = vtkResourceFileLocator::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkResourceFileLocator(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
