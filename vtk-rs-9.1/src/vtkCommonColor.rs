/// stores a list of colors.
///
///
///
/// The vtkColorSeries stores palettes of colors. There are several default
/// palettes (or schemes) available and functions to control several aspects
/// of what colors are returned. In essence a color scheme is set and then
/// the number of colors and individual color values may be requested.
///
/// For a web page showcasing the default palettes, see:
/// <a
/// href="https://htmlpreview.github.io/?https://github.com/Kitware/vtk-examples/blob/gh-pages/VTKColorSeriesPatches.html">VTKColorSeriesPatches</a>;
/// <a
/// href="https://kitware.github.io/vtk-examples/site/Python/Visualization/ColorSeriesPatches/">ColorSeriesPatches</a>
/// was used to generate this table.
///
/// It is also possible to add schemes beyond the default palettes.
/// Whenever \a SetColorScheme is called with a string for which no palette
/// already exists, a new, empty palette is created.
/// You may then use \a SetNumberOfColors and \a SetColor to populate the
/// palette.
/// You may not extend default palettes by calling functions that alter
/// a scheme; if called while a predefined palette is in use, they
/// will create a new non-default scheme and populate it with the current
/// palette before continuing.
///
/// The "Brewer" palettes are courtesy of
/// Cynthia A. Brewer (Dept. of Geography, Pennsylvania State University)
/// and under the Apache License. See the source code for details.
#[allow(non_camel_case_types)]
pub struct vtkColorSeries(*mut core::ffi::c_void);
impl vtkColorSeries {
    /// Creates a new [vtkColorSeries] wrapped inside `vtkNew`
    #[doc(alias = "vtkColorSeries")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkColorSeries_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkColorSeries_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkColorSeries_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkColorSeries_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkColorSeries {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkColorSeries {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkColorSeries_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkColorSeries_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkColorSeries_create_drop() {
    let obj = vtkColorSeries::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkColorSeries(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
/// A class holding colors and their names.
///
///
/// For a web page showcasing VTK Named Colors and their RGB values, see:
/// <a
/// href="https://htmlpreview.github.io/?https://github.com/Kitware/vtk-examples/blob/gh-pages/VTKNamedColorPatches.html">VTKNamedColorPatches</a>;
/// <a
/// href="https://kitware.github.io/vtk-examples/site/Python/Visualization/NamedColorPatches/">NamedColorPatches</a>
/// was used to generate this table.
///
/// Color names are case insensitive and are stored as lower-case names
/// along with a 4-element array whose elements are red, green, blue and alpha,
/// in that order, corresponding to the RGBA value of the color.
///
/// It is assumed that if the RGBA values are unsigned char then each element
/// lies in the range 0...255 and if the RGBA values are double then each
/// element lies in the range 0...1.
///
/// The colors and names are those in <a href="https://en.wikipedia.org/wiki/Web_colors">Web
/// colors</a> that are derived from the CSS3 specification: <a
/// href="https://www.w3.org/TR/css-color-3/">CSS Color Module Level 3</a> In this table
/// common synonyms such as cyan/aqua and magenta/fuchsia are also included.
///
/// Also included in this class are names and colors taken from
/// <em>Wrapping/Python/vtkmodules/util/colors.py</em> that were originally taken from
/// <em>Wrapping/Tcl/vtktesting/colors.tcl</em> (no longer in the VTK source files - deleted
/// 06-Dec-2017).
///
/// Web colors and names in <a href="https://en.wikipedia.org/wiki/Web_colors">Web colors</a> take
/// precedence over those in <em>colors.py</em>. One consequence of this
/// is that while <em>colors.py</em> specifies green as equivalent to
/// (0,255,0), the web color standard defines it as (0,128,0).
///
/// The \a SetColor methods will overwrite existing colors if the name of the
/// color being set matches an existing color. Note that ColorExists() can be
/// used to test for existence of the color being set.
///
/// In the case of the \a GetColor methods returning doubles, alternative versions,
/// identified by the letters RGB in the names, are provided.
/// These get functions return just the red, green and blue components of
/// a color.
///
/// The class also provides methods for defining a color through an HTML color
/// string. The following formats are supported:
///
/// - \#RGB                 (3-digit hexadecimal number, where #4F2 is a shortcut for #44FF22)
/// - \#RRGGBB              (6-digit hexadecimal number)
/// - rgb(r, g, b)          (where r, g, b are in 0..255 or percentage values)
/// - rgba(r, g, b, a)      (where r, g, b, are in 0..255 or percentage values, a is in 0.0..1.0)
/// - a CSS3 color name     (e.g. "steelblue")
#[allow(non_camel_case_types)]
pub struct vtkNamedColors(*mut core::ffi::c_void);
impl vtkNamedColors {
    /// Creates a new [vtkNamedColors] wrapped inside `vtkNew`
    #[doc(alias = "vtkNamedColors")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkNamedColors_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { &mut *vtkNamedColors_new() })
    }
    #[cfg(test)]
    unsafe fn _get_ptr(&self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtkNamedColors_get_ptr(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtkNamedColors_get_ptr(self.0) }
    }
}
impl std::default::Default for vtkNamedColors {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkNamedColors {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkNamedColors_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkNamedColors_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkNamedColors_create_drop() {
    let obj = vtkNamedColors::new();
    let ptr = obj.0;
    assert!(!ptr.is_null());
    assert!(unsafe { !obj._get_ptr().is_null() });
    drop(obj);
    let new_obj = vtkNamedColors(ptr);
    assert!(unsafe { new_obj._get_ptr().is_null() });
}
