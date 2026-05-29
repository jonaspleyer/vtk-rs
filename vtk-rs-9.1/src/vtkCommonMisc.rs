pub trait VtkContourValues {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_value(&mut self, i: core::ffi::c_int, value: core::ffi::c_double) -> ();
    fn get_value(&mut self, i: core::ffi::c_int) -> core::ffi::c_double;
    fn set_number_of_contours(&mut self, number: core::ffi::c_int) -> ();
    fn get_number_of_contours(&mut self) -> core::ffi::c_int;
    fn generate_values(
        &mut self,
        numContours: core::ffi::c_int,
        rangeStart: core::ffi::c_double,
        rangeEnd: core::ffi::c_double,
    ) -> ();
    fn deep_copy(&mut self, other: *mut core::ffi::c_void) -> ();
}
pub trait VtkErrorCode {
    fn get_string_from_error_code(&mut self, error: core::ffi::c_ulong) -> &str;
    fn get_error_code_from_string(&mut self, error: &str) -> core::ffi::c_ulong;
    fn get_last_system_error(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkExprTkFunctionParser {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn set_function(&mut self, function: &str) -> ();
    fn get_function(&mut self) -> &str;
    fn is_scalar_result(&mut self) -> core::ffi::c_int;
    fn is_vector_result(&mut self) -> core::ffi::c_int;
    fn get_scalar_result(&mut self) -> core::ffi::c_double;
    fn set_scalar_variable_value(
        &mut self,
        variableName: &str,
        value: core::ffi::c_double,
    ) -> ();
    fn get_scalar_variable_value(&mut self, variableName: &str) -> core::ffi::c_double;
    fn set_vector_variable_value(
        &mut self,
        variableName: &str,
        xValue: core::ffi::c_double,
        yValue: core::ffi::c_double,
        zValue: core::ffi::c_double,
    ) -> ();
    fn get_number_of_scalar_variables(&mut self) -> core::ffi::c_int;
    fn get_scalar_variable_index(&mut self, name: &str) -> core::ffi::c_int;
    fn get_number_of_vector_variables(&mut self) -> core::ffi::c_int;
    fn get_vector_variable_index(&mut self, name: &str) -> core::ffi::c_int;
    fn get_scalar_variable_needed(&mut self, i: core::ffi::c_int) -> bool;
    fn get_vector_variable_needed(&mut self, i: core::ffi::c_int) -> bool;
    fn remove_all_variables(&mut self) -> ();
    fn remove_scalar_variables(&mut self) -> ();
    fn remove_vector_variables(&mut self) -> ();
    fn set_replace_invalid_values(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_replace_invalid_values(&mut self) -> core::ffi::c_int;
    fn replace_invalid_values_on(&mut self) -> ();
    fn replace_invalid_values_off(&mut self) -> ();
    fn set_replacement_value(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_replacement_value(&mut self) -> core::ffi::c_double;
    fn invalidate_function(&mut self) -> ();
}
pub trait VtkFunctionParser {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn set_function(&mut self, function: &str) -> ();
    fn is_scalar_result(&mut self) -> core::ffi::c_int;
    fn is_vector_result(&mut self) -> core::ffi::c_int;
    fn get_scalar_result(&mut self) -> core::ffi::c_double;
    fn set_scalar_variable_value(
        &mut self,
        variableName: &str,
        value: core::ffi::c_double,
    ) -> ();
    fn get_scalar_variable_value(&mut self, variableName: &str) -> core::ffi::c_double;
    fn set_vector_variable_value(
        &mut self,
        variableName: &str,
        xValue: core::ffi::c_double,
        yValue: core::ffi::c_double,
        zValue: core::ffi::c_double,
    ) -> ();
    fn get_number_of_scalar_variables(&mut self) -> core::ffi::c_int;
    fn get_scalar_variable_index(&mut self, name: &str) -> core::ffi::c_int;
    fn get_number_of_vector_variables(&mut self) -> core::ffi::c_int;
    fn get_vector_variable_index(&mut self, name: &str) -> core::ffi::c_int;
    fn get_scalar_variable_name(&mut self, i: core::ffi::c_int) -> &str;
    fn get_vector_variable_name(&mut self, i: core::ffi::c_int) -> &str;
    fn get_scalar_variable_needed(&mut self, i: core::ffi::c_int) -> bool;
    fn get_vector_variable_needed(&mut self, i: core::ffi::c_int) -> bool;
    fn remove_all_variables(&mut self) -> ();
    fn remove_scalar_variables(&mut self) -> ();
    fn remove_vector_variables(&mut self) -> ();
    fn set_replace_invalid_values(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_replace_invalid_values(&mut self) -> core::ffi::c_int;
    fn replace_invalid_values_on(&mut self) -> ();
    fn replace_invalid_values_off(&mut self) -> ();
    fn set_replacement_value(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_replacement_value(&mut self) -> core::ffi::c_double;
    fn invalidate_function(&mut self) -> ();
}
pub trait VtkHeap {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_block_size(&mut self, p0: usize) -> ();
    fn get_block_size(&mut self) -> usize;
    fn get_number_of_blocks(&mut self) -> core::ffi::c_int;
    fn get_number_of_allocations(&mut self) -> core::ffi::c_int;
    fn reset(&mut self) -> ();
}
pub trait VtkPolygonBuilder {
    fn get_polygons(&mut self, polys: *mut core::ffi::c_void) -> ();
    fn reset(&mut self) -> ();
}
pub trait VtkResourceFileLocator {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_print_debug_information(&mut self, p0: bool) -> ();
    fn get_print_debug_information(&mut self) -> bool;
    fn print_debug_information_on(&mut self) -> ();
    fn print_debug_information_off(&mut self) -> ();
    fn set_log_verbosity(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_log_verbosity(&mut self) -> core::ffi::c_int;
}
impl VtkContourValues for vtkContourValues {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_contour_values_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_contour_values_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_contour_values_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_contour_values_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_contour_values_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_contour_values_new_instance(self.0) }
    }
    fn set_value(&mut self, i: core::ffi::c_int, value: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_contour_values_set_value(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
                value: core::ffi::c_double,
            );
        }
        unsafe { vtk_contour_values_set_value(self.0, i, value) }
    }
    fn get_value(&mut self, i: core::ffi::c_int) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_contour_values_get_value(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_contour_values_get_value(self.0, i) }
    }
    fn set_number_of_contours(&mut self, number: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_contour_values_set_number_of_contours(
                sself: *mut core::ffi::c_void,
                number: core::ffi::c_int,
            );
        }
        unsafe { vtk_contour_values_set_number_of_contours(self.0, number) }
    }
    fn get_number_of_contours(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_contour_values_get_number_of_contours(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_contour_values_get_number_of_contours(self.0) }
    }
    fn generate_values(
        &mut self,
        numContours: core::ffi::c_int,
        rangeStart: core::ffi::c_double,
        rangeEnd: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_contour_values_generate_values(
                sself: *mut core::ffi::c_void,
                numContours: core::ffi::c_int,
                rangeStart: core::ffi::c_double,
                rangeEnd: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_contour_values_generate_values(self.0, numContours, rangeStart, rangeEnd)
        }
    }
    fn deep_copy(&mut self, other: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_contour_values_deep_copy(
                sself: *mut core::ffi::c_void,
                other: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_contour_values_deep_copy(self.0, other) }
    }
}
impl VtkExprTkFunctionParser for vtkExprTkFunctionParser {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_expr_tk_function_parser_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_expr_tk_function_parser_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_expr_tk_function_parser_new_instance(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_expr_tk_function_parser_get_m_time(self.0) }
    }
    fn set_function(&mut self, function: &str) -> () {
        let c_function = std::ffi::CString::new(function).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_set_function(
                sself: *mut core::ffi::c_void,
                function: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_expr_tk_function_parser_set_function(self.0, c_function.as_ptr()) }
    }
    fn get_function(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_function(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_expr_tk_function_parser_get_function(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn is_scalar_result(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_is_scalar_result(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_expr_tk_function_parser_is_scalar_result(self.0) }
    }
    fn is_vector_result(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_is_vector_result(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_expr_tk_function_parser_is_vector_result(self.0) }
    }
    fn get_scalar_result(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_scalar_result(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_expr_tk_function_parser_get_scalar_result(self.0) }
    }
    fn set_scalar_variable_value(
        &mut self,
        variableName: &str,
        value: core::ffi::c_double,
    ) -> () {
        let c_variableName = std::ffi::CString::new(variableName)
            .expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_set_scalar_variable_value(
                sself: *mut core::ffi::c_void,
                variableName: *const core::ffi::c_char,
                value: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_expr_tk_function_parser_set_scalar_variable_value(
                self.0,
                c_variableName.as_ptr(),
                value,
            )
        }
    }
    fn get_scalar_variable_value(&mut self, variableName: &str) -> core::ffi::c_double {
        let c_variableName = std::ffi::CString::new(variableName)
            .expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_scalar_variable_value(
                sself: *mut core::ffi::c_void,
                variableName: *const core::ffi::c_char,
            ) -> core::ffi::c_double;
        }
        unsafe {
            vtk_expr_tk_function_parser_get_scalar_variable_value(
                self.0,
                c_variableName.as_ptr(),
            )
        }
    }
    fn set_vector_variable_value(
        &mut self,
        variableName: &str,
        xValue: core::ffi::c_double,
        yValue: core::ffi::c_double,
        zValue: core::ffi::c_double,
    ) -> () {
        let c_variableName = std::ffi::CString::new(variableName)
            .expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_set_vector_variable_value(
                sself: *mut core::ffi::c_void,
                variableName: *const core::ffi::c_char,
                xValue: core::ffi::c_double,
                yValue: core::ffi::c_double,
                zValue: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_expr_tk_function_parser_set_vector_variable_value(
                self.0,
                c_variableName.as_ptr(),
                xValue,
                yValue,
                zValue,
            )
        }
    }
    fn get_number_of_scalar_variables(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_number_of_scalar_variables(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_expr_tk_function_parser_get_number_of_scalar_variables(self.0) }
    }
    fn get_scalar_variable_index(&mut self, name: &str) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_scalar_variable_index(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_expr_tk_function_parser_get_scalar_variable_index(
                self.0,
                c_name.as_ptr(),
            )
        }
    }
    fn get_number_of_vector_variables(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_number_of_vector_variables(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_expr_tk_function_parser_get_number_of_vector_variables(self.0) }
    }
    fn get_vector_variable_index(&mut self, name: &str) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_vector_variable_index(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_expr_tk_function_parser_get_vector_variable_index(
                self.0,
                c_name.as_ptr(),
            )
        }
    }
    fn get_scalar_variable_needed(&mut self, i: core::ffi::c_int) -> bool {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_scalar_variable_needed(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> bool;
        }
        unsafe { vtk_expr_tk_function_parser_get_scalar_variable_needed(self.0, i) }
    }
    fn get_vector_variable_needed(&mut self, i: core::ffi::c_int) -> bool {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_vector_variable_needed(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> bool;
        }
        unsafe { vtk_expr_tk_function_parser_get_vector_variable_needed(self.0, i) }
    }
    fn remove_all_variables(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_remove_all_variables(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_expr_tk_function_parser_remove_all_variables(self.0) }
    }
    fn remove_scalar_variables(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_remove_scalar_variables(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_expr_tk_function_parser_remove_scalar_variables(self.0) }
    }
    fn remove_vector_variables(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_remove_vector_variables(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_expr_tk_function_parser_remove_vector_variables(self.0) }
    }
    fn set_replace_invalid_values(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_set_replace_invalid_values(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_expr_tk_function_parser_set_replace_invalid_values(self.0, _arg) }
    }
    fn get_replace_invalid_values(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_replace_invalid_values(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_expr_tk_function_parser_get_replace_invalid_values(self.0) }
    }
    fn replace_invalid_values_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_replace_invalid_values_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_expr_tk_function_parser_replace_invalid_values_on(self.0) }
    }
    fn replace_invalid_values_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_replace_invalid_values_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_expr_tk_function_parser_replace_invalid_values_off(self.0) }
    }
    fn set_replacement_value(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_set_replacement_value(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_expr_tk_function_parser_set_replacement_value(self.0, _arg) }
    }
    fn get_replacement_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_get_replacement_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_expr_tk_function_parser_get_replacement_value(self.0) }
    }
    fn invalidate_function(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_expr_tk_function_parser_invalidate_function(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_expr_tk_function_parser_invalidate_function(self.0) }
    }
}
impl VtkFunctionParser for vtkFunctionParser {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_function_parser_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_function_parser_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_function_parser_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_function_parser_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_function_parser_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_function_parser_new_instance(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_function_parser_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_function_parser_get_m_time(self.0) }
    }
    fn set_function(&mut self, function: &str) -> () {
        let c_function = std::ffi::CString::new(function).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_function_parser_set_function(
                sself: *mut core::ffi::c_void,
                function: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_function_parser_set_function(self.0, c_function.as_ptr()) }
    }
    fn is_scalar_result(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_function_parser_is_scalar_result(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_function_parser_is_scalar_result(self.0) }
    }
    fn is_vector_result(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_function_parser_is_vector_result(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_function_parser_is_vector_result(self.0) }
    }
    fn get_scalar_result(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_function_parser_get_scalar_result(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_function_parser_get_scalar_result(self.0) }
    }
    fn set_scalar_variable_value(
        &mut self,
        variableName: &str,
        value: core::ffi::c_double,
    ) -> () {
        let c_variableName = std::ffi::CString::new(variableName)
            .expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_function_parser_set_scalar_variable_value(
                sself: *mut core::ffi::c_void,
                variableName: *const core::ffi::c_char,
                value: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_function_parser_set_scalar_variable_value(
                self.0,
                c_variableName.as_ptr(),
                value,
            )
        }
    }
    fn get_scalar_variable_value(&mut self, variableName: &str) -> core::ffi::c_double {
        let c_variableName = std::ffi::CString::new(variableName)
            .expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_function_parser_get_scalar_variable_value(
                sself: *mut core::ffi::c_void,
                variableName: *const core::ffi::c_char,
            ) -> core::ffi::c_double;
        }
        unsafe {
            vtk_function_parser_get_scalar_variable_value(
                self.0,
                c_variableName.as_ptr(),
            )
        }
    }
    fn set_vector_variable_value(
        &mut self,
        variableName: &str,
        xValue: core::ffi::c_double,
        yValue: core::ffi::c_double,
        zValue: core::ffi::c_double,
    ) -> () {
        let c_variableName = std::ffi::CString::new(variableName)
            .expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_function_parser_set_vector_variable_value(
                sself: *mut core::ffi::c_void,
                variableName: *const core::ffi::c_char,
                xValue: core::ffi::c_double,
                yValue: core::ffi::c_double,
                zValue: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_function_parser_set_vector_variable_value(
                self.0,
                c_variableName.as_ptr(),
                xValue,
                yValue,
                zValue,
            )
        }
    }
    fn get_number_of_scalar_variables(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_function_parser_get_number_of_scalar_variables(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_function_parser_get_number_of_scalar_variables(self.0) }
    }
    fn get_scalar_variable_index(&mut self, name: &str) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_function_parser_get_scalar_variable_index(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_function_parser_get_scalar_variable_index(self.0, c_name.as_ptr()) }
    }
    fn get_number_of_vector_variables(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_function_parser_get_number_of_vector_variables(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_function_parser_get_number_of_vector_variables(self.0) }
    }
    fn get_vector_variable_index(&mut self, name: &str) -> core::ffi::c_int {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_function_parser_get_vector_variable_index(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_function_parser_get_vector_variable_index(self.0, c_name.as_ptr()) }
    }
    fn get_scalar_variable_name(&mut self, i: core::ffi::c_int) -> &str {
        unsafe extern "C" {
            fn vtk_function_parser_get_scalar_variable_name(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_function_parser_get_scalar_variable_name(self.0, i) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn get_vector_variable_name(&mut self, i: core::ffi::c_int) -> &str {
        unsafe extern "C" {
            fn vtk_function_parser_get_vector_variable_name(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_function_parser_get_vector_variable_name(self.0, i) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn get_scalar_variable_needed(&mut self, i: core::ffi::c_int) -> bool {
        unsafe extern "C" {
            fn vtk_function_parser_get_scalar_variable_needed(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> bool;
        }
        unsafe { vtk_function_parser_get_scalar_variable_needed(self.0, i) }
    }
    fn get_vector_variable_needed(&mut self, i: core::ffi::c_int) -> bool {
        unsafe extern "C" {
            fn vtk_function_parser_get_vector_variable_needed(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> bool;
        }
        unsafe { vtk_function_parser_get_vector_variable_needed(self.0, i) }
    }
    fn remove_all_variables(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_function_parser_remove_all_variables(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_function_parser_remove_all_variables(self.0) }
    }
    fn remove_scalar_variables(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_function_parser_remove_scalar_variables(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_function_parser_remove_scalar_variables(self.0) }
    }
    fn remove_vector_variables(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_function_parser_remove_vector_variables(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_function_parser_remove_vector_variables(self.0) }
    }
    fn set_replace_invalid_values(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_function_parser_set_replace_invalid_values(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_function_parser_set_replace_invalid_values(self.0, _arg) }
    }
    fn get_replace_invalid_values(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_function_parser_get_replace_invalid_values(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_function_parser_get_replace_invalid_values(self.0) }
    }
    fn replace_invalid_values_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_function_parser_replace_invalid_values_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_function_parser_replace_invalid_values_on(self.0) }
    }
    fn replace_invalid_values_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_function_parser_replace_invalid_values_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_function_parser_replace_invalid_values_off(self.0) }
    }
    fn set_replacement_value(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_function_parser_set_replacement_value(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_function_parser_set_replacement_value(self.0, _arg) }
    }
    fn get_replacement_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_function_parser_get_replacement_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_function_parser_get_replacement_value(self.0) }
    }
    fn invalidate_function(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_function_parser_invalidate_function(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_function_parser_invalidate_function(self.0) }
    }
}
impl VtkHeap for vtkHeap {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_heap_new(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_heap_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_heap_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_heap_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_heap_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_heap_new_instance(self.0) }
    }
    fn set_block_size(&mut self, p0: usize) -> () {
        unsafe extern "C" {
            fn vtk_heap_set_block_size(sself: *mut core::ffi::c_void, p0: usize);
        }
        unsafe { vtk_heap_set_block_size(self.0, p0) }
    }
    fn get_block_size(&mut self) -> usize {
        unsafe extern "C" {
            fn vtk_heap_get_block_size(sself: *mut core::ffi::c_void) -> usize;
        }
        unsafe { vtk_heap_get_block_size(self.0) }
    }
    fn get_number_of_blocks(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_heap_get_number_of_blocks(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_heap_get_number_of_blocks(self.0) }
    }
    fn get_number_of_allocations(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_heap_get_number_of_allocations(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_heap_get_number_of_allocations(self.0) }
    }
    fn reset(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_heap_reset(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_heap_reset(self.0) }
    }
}
impl VtkResourceFileLocator for vtkResourceFileLocator {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_resource_file_locator_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_resource_file_locator_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_resource_file_locator_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_resource_file_locator_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_resource_file_locator_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_resource_file_locator_new_instance(self.0) }
    }
    fn set_print_debug_information(&mut self, p0: bool) -> () {
        unsafe extern "C" {
            fn vtk_resource_file_locator_set_print_debug_information(
                sself: *mut core::ffi::c_void,
                p0: bool,
            );
        }
        unsafe { vtk_resource_file_locator_set_print_debug_information(self.0, p0) }
    }
    fn get_print_debug_information(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_resource_file_locator_get_print_debug_information(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_resource_file_locator_get_print_debug_information(self.0) }
    }
    fn print_debug_information_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_resource_file_locator_print_debug_information_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_resource_file_locator_print_debug_information_on(self.0) }
    }
    fn print_debug_information_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_resource_file_locator_print_debug_information_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_resource_file_locator_print_debug_information_off(self.0) }
    }
    fn set_log_verbosity(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_resource_file_locator_set_log_verbosity(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_resource_file_locator_set_log_verbosity(self.0, _arg) }
    }
    fn get_log_verbosity(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_resource_file_locator_get_log_verbosity(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_resource_file_locator_get_log_verbosity(self.0) }
    }
}
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
    /// Creates a new [vtkContourValues] via `vtkContourValues::New()`
    #[doc(alias = "vtkContourValues")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkContourValues_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkContourValues_new() })
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
    assert!(!obj.0.is_null());
    drop(obj);
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
    /// Creates a new [vtkExprTkFunctionParser] via `vtkExprTkFunctionParser::New()`
    #[doc(alias = "vtkExprTkFunctionParser")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkExprTkFunctionParser_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkExprTkFunctionParser_new() })
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
    assert!(!obj.0.is_null());
    drop(obj);
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
    /// Creates a new [vtkFunctionParser] via `vtkFunctionParser::New()`
    #[doc(alias = "vtkFunctionParser")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFunctionParser_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkFunctionParser_new() })
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
    assert!(!obj.0.is_null());
    drop(obj);
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
    /// Creates a new [vtkHeap] via `vtkHeap::New()`
    #[doc(alias = "vtkHeap")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHeap_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkHeap_new() })
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
    assert!(!obj.0.is_null());
    drop(obj);
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
    /// Creates a new [vtkResourceFileLocator] via `vtkResourceFileLocator::New()`
    #[doc(alias = "vtkResourceFileLocator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkResourceFileLocator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkResourceFileLocator_new() })
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
    assert!(!obj.0.is_null());
    drop(obj);
}
