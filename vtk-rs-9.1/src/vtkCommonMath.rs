pub trait VtkAmoebaMinimizer {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_function_arg_delete(&mut self, f: *mut core::ffi::c_void) -> ();
    fn set_parameter_value(&mut self, name: &str, value: core::ffi::c_double) -> ();
    fn set_parameter_scale(&mut self, name: &str, scale: core::ffi::c_double) -> ();
    fn get_parameter_scale(&mut self, name: &str) -> core::ffi::c_double;
    fn get_parameter_value(&mut self, name: &str) -> core::ffi::c_double;
    fn get_parameter_name(&mut self, i: core::ffi::c_int) -> &str;
    fn get_number_of_parameters(&mut self) -> core::ffi::c_int;
    fn initialize(&mut self) -> ();
    fn minimize(&mut self) -> ();
    fn iterate(&mut self) -> core::ffi::c_int;
    fn set_function_value(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_function_value(&mut self) -> core::ffi::c_double;
    fn set_contraction_ratio(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_contraction_ratio_min_value(&mut self) -> core::ffi::c_double;
    fn get_contraction_ratio_max_value(&mut self) -> core::ffi::c_double;
    fn get_contraction_ratio(&mut self) -> core::ffi::c_double;
    fn set_expansion_ratio(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_expansion_ratio_min_value(&mut self) -> core::ffi::c_double;
    fn get_expansion_ratio_max_value(&mut self) -> core::ffi::c_double;
    fn get_expansion_ratio(&mut self) -> core::ffi::c_double;
    fn set_tolerance(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_tolerance(&mut self) -> core::ffi::c_double;
    fn set_parameter_tolerance(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_parameter_tolerance(&mut self) -> core::ffi::c_double;
    fn set_max_iterations(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_max_iterations(&mut self) -> core::ffi::c_int;
    fn get_iterations(&mut self) -> core::ffi::c_int;
    fn get_function_evaluations(&mut self) -> core::ffi::c_int;
    fn evaluate_function(&mut self) -> ();
}
pub trait VtkFFT {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn hanning_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double;
    fn bartlett_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double;
    fn sine_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double;
    fn blackman_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double;
    fn rectangular_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double;
}
pub trait VtkFunctionSet {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_functions(&mut self) -> core::ffi::c_int;
    fn get_number_of_independent_variables(&mut self) -> core::ffi::c_int;
}
pub trait VtkInitialValueProblemSolver {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_function_set(&mut self, fset: *mut core::ffi::c_void) -> ();
    fn get_function_set(&mut self) -> *mut core::ffi::c_void;
    fn is_adaptive(&mut self) -> core::ffi::c_int;
}
pub trait VtkMatrix3x3 {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn deep_copy(&mut self, source: *mut core::ffi::c_void) -> ();
    fn zero(&mut self) -> ();
    fn identity(&mut self) -> ();
    fn invert(&mut self, in_: *mut core::ffi::c_void, out: *mut core::ffi::c_void) -> ();
    fn transpose(
        &mut self,
        in_: *mut core::ffi::c_void,
        out: *mut core::ffi::c_void,
    ) -> ();
    fn multiply_3_x_3(
        &mut self,
        a: *mut core::ffi::c_void,
        b: *mut core::ffi::c_void,
        c: *mut core::ffi::c_void,
    ) -> ();
    fn adjoint(
        &mut self,
        in_: *mut core::ffi::c_void,
        out: *mut core::ffi::c_void,
    ) -> ();
    fn determinant(&mut self) -> core::ffi::c_double;
    fn set_element(
        &mut self,
        i: core::ffi::c_int,
        j: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> ();
    fn get_element(
        &mut self,
        i: core::ffi::c_int,
        j: core::ffi::c_int,
    ) -> core::ffi::c_double;
    fn is_identity(&mut self) -> bool;
}
pub trait VtkMatrix4x4 {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn zero(&mut self) -> ();
    fn identity(&mut self) -> ();
    fn is_identity(&mut self) -> bool;
    fn determinant(&mut self) -> core::ffi::c_double;
    fn set_element(
        &mut self,
        i: core::ffi::c_int,
        j: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> ();
    fn get_element(
        &mut self,
        i: core::ffi::c_int,
        j: core::ffi::c_int,
    ) -> core::ffi::c_double;
}
pub trait VtkPolynomialSolversUnivariate {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_division_tolerance(&mut self, tol: core::ffi::c_double) -> ();
    fn get_division_tolerance(&mut self) -> core::ffi::c_double;
}
pub trait VtkQuaternion {
    fn to_identity(&mut self) -> ();
    fn conjugate(&mut self) -> ();
    fn invert(&mut self) -> ();
    fn to_unit_log(&mut self) -> ();
    fn to_unit_exp(&mut self) -> ();
    fn normalize_with_angle_in_degrees(&mut self) -> ();
}
pub trait VtkQuaternionInterpolator {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_number_of_quaternions(&mut self) -> core::ffi::c_int;
    fn get_minimum_t(&mut self) -> core::ffi::c_double;
    fn get_maximum_t(&mut self) -> core::ffi::c_double;
    fn initialize(&mut self) -> ();
    fn remove_quaternion(&mut self, t: core::ffi::c_double) -> ();
    fn get_search_method(&mut self) -> core::ffi::c_int;
    fn set_search_method(&mut self, type_: core::ffi::c_int) -> ();
    fn set_interpolation_type(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_interpolation_type_min_value(&mut self) -> core::ffi::c_int;
    fn get_interpolation_type_max_value(&mut self) -> core::ffi::c_int;
    fn get_interpolation_type(&mut self) -> core::ffi::c_int;
    fn set_interpolation_type_to_linear(&mut self) -> ();
    fn set_interpolation_type_to_spline(&mut self) -> ();
}
pub trait VtkQuaterniond {
    fn squared_norm(&mut self) -> core::ffi::c_double;
    fn norm(&mut self) -> core::ffi::c_double;
    fn normalize(&mut self) -> core::ffi::c_double;
    fn set(
        &mut self,
        w: &core::ffi::c_double,
        x: &core::ffi::c_double,
        y: &core::ffi::c_double,
        z: &core::ffi::c_double,
    ) -> ();
    fn set_w(&mut self, w: &core::ffi::c_double) -> ();
    fn get_w(&mut self) -> core::ffi::c_double;
    fn set_x(&mut self, x: &core::ffi::c_double) -> ();
    fn get_x(&mut self) -> core::ffi::c_double;
    fn set_y(&mut self, y: &core::ffi::c_double) -> ();
    fn get_y(&mut self) -> core::ffi::c_double;
    fn set_z(&mut self, z: &core::ffi::c_double) -> ();
    fn get_z(&mut self) -> core::ffi::c_double;
    fn set_rotation_angle_and_axis(
        &mut self,
        angle: &core::ffi::c_double,
        x: &core::ffi::c_double,
        y: &core::ffi::c_double,
        z: &core::ffi::c_double,
    ) -> ();
}
pub trait VtkQuaternionf {
    fn squared_norm(&mut self) -> core::ffi::c_float;
    fn norm(&mut self) -> core::ffi::c_float;
    fn normalize(&mut self) -> core::ffi::c_float;
    fn set(
        &mut self,
        w: &core::ffi::c_float,
        x: &core::ffi::c_float,
        y: &core::ffi::c_float,
        z: &core::ffi::c_float,
    ) -> ();
    fn set_w(&mut self, w: &core::ffi::c_float) -> ();
    fn get_w(&mut self) -> core::ffi::c_float;
    fn set_x(&mut self, x: &core::ffi::c_float) -> ();
    fn get_x(&mut self) -> core::ffi::c_float;
    fn set_y(&mut self, y: &core::ffi::c_float) -> ();
    fn get_y(&mut self) -> core::ffi::c_float;
    fn set_z(&mut self, z: &core::ffi::c_float) -> ();
    fn get_z(&mut self) -> core::ffi::c_float;
    fn set_rotation_angle_and_axis(
        &mut self,
        angle: &core::ffi::c_float,
        x: &core::ffi::c_float,
        y: &core::ffi::c_float,
        z: &core::ffi::c_float,
    ) -> ();
}
pub trait VtkRungeKutta2 {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkRungeKutta4 {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkRungeKutta45 {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkTuple {
    fn get_size(&mut self) -> core::ffi::c_int;
    fn get_data(&mut self) -> *mut core::ffi::c_void;
}
impl VtkAmoebaMinimizer for vtkAmoebaMinimizer {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_amoeba_minimizer_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_amoeba_minimizer_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_amoeba_minimizer_new_instance(self.0) }
    }
    fn set_function_arg_delete(&mut self, f: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_function_arg_delete(
                sself: *mut core::ffi::c_void,
                f: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_amoeba_minimizer_set_function_arg_delete(self.0, f) }
    }
    fn set_parameter_value(&mut self, name: &str, value: core::ffi::c_double) -> () {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_parameter_value(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
                value: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_amoeba_minimizer_set_parameter_value(self.0, c_name.as_ptr(), value)
        }
    }
    fn set_parameter_scale(&mut self, name: &str, scale: core::ffi::c_double) -> () {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_parameter_scale(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
                scale: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_amoeba_minimizer_set_parameter_scale(self.0, c_name.as_ptr(), scale)
        }
    }
    fn get_parameter_scale(&mut self, name: &str) -> core::ffi::c_double {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_parameter_scale(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_parameter_scale(self.0, c_name.as_ptr()) }
    }
    fn get_parameter_value(&mut self, name: &str) -> core::ffi::c_double {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_parameter_value(
                sself: *mut core::ffi::c_void,
                name: *const core::ffi::c_char,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_parameter_value(self.0, c_name.as_ptr()) }
    }
    fn get_parameter_name(&mut self, i: core::ffi::c_int) -> &str {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_parameter_name(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_amoeba_minimizer_get_parameter_name(self.0, i) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn get_number_of_parameters(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_number_of_parameters(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_amoeba_minimizer_get_number_of_parameters(self.0) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_amoeba_minimizer_initialize(self.0) }
    }
    fn minimize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_minimize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_amoeba_minimizer_minimize(self.0) }
    }
    fn iterate(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_iterate(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_amoeba_minimizer_iterate(self.0) }
    }
    fn set_function_value(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_function_value(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_amoeba_minimizer_set_function_value(self.0, _arg) }
    }
    fn get_function_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_function_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_function_value(self.0) }
    }
    fn set_contraction_ratio(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_contraction_ratio(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_amoeba_minimizer_set_contraction_ratio(self.0, _arg) }
    }
    fn get_contraction_ratio_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_contraction_ratio_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_contraction_ratio_min_value(self.0) }
    }
    fn get_contraction_ratio_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_contraction_ratio_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_contraction_ratio_max_value(self.0) }
    }
    fn get_contraction_ratio(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_contraction_ratio(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_contraction_ratio(self.0) }
    }
    fn set_expansion_ratio(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_expansion_ratio(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_amoeba_minimizer_set_expansion_ratio(self.0, _arg) }
    }
    fn get_expansion_ratio_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_expansion_ratio_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_expansion_ratio_min_value(self.0) }
    }
    fn get_expansion_ratio_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_expansion_ratio_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_expansion_ratio_max_value(self.0) }
    }
    fn get_expansion_ratio(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_expansion_ratio(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_expansion_ratio(self.0) }
    }
    fn set_tolerance(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_tolerance(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_amoeba_minimizer_set_tolerance(self.0, _arg) }
    }
    fn get_tolerance(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_tolerance(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_tolerance(self.0) }
    }
    fn set_parameter_tolerance(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_parameter_tolerance(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_amoeba_minimizer_set_parameter_tolerance(self.0, _arg) }
    }
    fn get_parameter_tolerance(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_parameter_tolerance(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_amoeba_minimizer_get_parameter_tolerance(self.0) }
    }
    fn set_max_iterations(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_set_max_iterations(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_amoeba_minimizer_set_max_iterations(self.0, _arg) }
    }
    fn get_max_iterations(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_max_iterations(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_amoeba_minimizer_get_max_iterations(self.0) }
    }
    fn get_iterations(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_iterations(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_amoeba_minimizer_get_iterations(self.0) }
    }
    fn get_function_evaluations(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_get_function_evaluations(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_amoeba_minimizer_get_function_evaluations(self.0) }
    }
    fn evaluate_function(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_amoeba_minimizer_evaluate_function(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_amoeba_minimizer_evaluate_function(self.0) }
    }
}
impl VtkFFT for vtkFFT {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_fft_new(sself: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_fft_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_fft_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_fft_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_fft_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_fft_new_instance(self.0) }
    }
    fn hanning_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_fft_hanning_generator(
                sself: *mut core::ffi::c_void,
                x: usize,
                size: usize,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_fft_hanning_generator(self.0, x, size) }
    }
    fn bartlett_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_fft_bartlett_generator(
                sself: *mut core::ffi::c_void,
                x: usize,
                size: usize,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_fft_bartlett_generator(self.0, x, size) }
    }
    fn sine_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_fft_sine_generator(
                sself: *mut core::ffi::c_void,
                x: usize,
                size: usize,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_fft_sine_generator(self.0, x, size) }
    }
    fn blackman_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_fft_blackman_generator(
                sself: *mut core::ffi::c_void,
                x: usize,
                size: usize,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_fft_blackman_generator(self.0, x, size) }
    }
    fn rectangular_generator(&mut self, x: usize, size: usize) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_fft_rectangular_generator(
                sself: *mut core::ffi::c_void,
                x: usize,
                size: usize,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_fft_rectangular_generator(self.0, x, size) }
    }
}
impl VtkMatrix3x3 for vtkMatrix3x3 {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_3_x_3_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_3_x_3_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_3_x_3_new_instance(self.0) }
    }
    fn deep_copy(&mut self, source: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_deep_copy(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_matrix_3_x_3_deep_copy(self.0, source) }
    }
    fn zero(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_zero(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_matrix_3_x_3_zero(self.0) }
    }
    fn identity(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_identity(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_matrix_3_x_3_identity(self.0) }
    }
    fn invert(
        &mut self,
        in_: *mut core::ffi::c_void,
        out: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_invert(
                sself: *mut core::ffi::c_void,
                in_: *mut core::ffi::c_void,
                out: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_matrix_3_x_3_invert(self.0, in_, out) }
    }
    fn transpose(
        &mut self,
        in_: *mut core::ffi::c_void,
        out: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_transpose(
                sself: *mut core::ffi::c_void,
                in_: *mut core::ffi::c_void,
                out: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_matrix_3_x_3_transpose(self.0, in_, out) }
    }
    fn multiply_3_x_3(
        &mut self,
        a: *mut core::ffi::c_void,
        b: *mut core::ffi::c_void,
        c: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_multiply_3_x_3(
                sself: *mut core::ffi::c_void,
                a: *mut core::ffi::c_void,
                b: *mut core::ffi::c_void,
                c: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_matrix_3_x_3_multiply_3_x_3(self.0, a, b, c) }
    }
    fn adjoint(
        &mut self,
        in_: *mut core::ffi::c_void,
        out: *mut core::ffi::c_void,
    ) -> () {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_adjoint(
                sself: *mut core::ffi::c_void,
                in_: *mut core::ffi::c_void,
                out: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_matrix_3_x_3_adjoint(self.0, in_, out) }
    }
    fn determinant(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_determinant(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_matrix_3_x_3_determinant(self.0) }
    }
    fn set_element(
        &mut self,
        i: core::ffi::c_int,
        j: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_set_element(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
                j: core::ffi::c_int,
                value: core::ffi::c_double,
            );
        }
        unsafe { vtk_matrix_3_x_3_set_element(self.0, i, j, value) }
    }
    fn get_element(
        &mut self,
        i: core::ffi::c_int,
        j: core::ffi::c_int,
    ) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_get_element(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
                j: core::ffi::c_int,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_matrix_3_x_3_get_element(self.0, i, j) }
    }
    fn is_identity(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_matrix_3_x_3_is_identity(sself: *mut core::ffi::c_void) -> bool;
        }
        unsafe { vtk_matrix_3_x_3_is_identity(self.0) }
    }
}
impl VtkMatrix4x4 for vtkMatrix4x4 {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_4_x_4_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_4_x_4_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_4_x_4_new_instance(self.0) }
    }
    fn zero(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_zero(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_matrix_4_x_4_zero(self.0) }
    }
    fn identity(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_identity(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_matrix_4_x_4_identity(self.0) }
    }
    fn is_identity(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_is_identity(sself: *mut core::ffi::c_void) -> bool;
        }
        unsafe { vtk_matrix_4_x_4_is_identity(self.0) }
    }
    fn determinant(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_determinant(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_matrix_4_x_4_determinant(self.0) }
    }
    fn set_element(
        &mut self,
        i: core::ffi::c_int,
        j: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_set_element(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
                j: core::ffi::c_int,
                value: core::ffi::c_double,
            );
        }
        unsafe { vtk_matrix_4_x_4_set_element(self.0, i, j, value) }
    }
    fn get_element(
        &mut self,
        i: core::ffi::c_int,
        j: core::ffi::c_int,
    ) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_matrix_4_x_4_get_element(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
                j: core::ffi::c_int,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_matrix_4_x_4_get_element(self.0, i, j) }
    }
}
impl VtkPolynomialSolversUnivariate for vtkPolynomialSolversUnivariate {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_polynomial_solvers_univariate_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_polynomial_solvers_univariate_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_polynomial_solvers_univariate_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_polynomial_solvers_univariate_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_polynomial_solvers_univariate_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_polynomial_solvers_univariate_new_instance(self.0) }
    }
    fn set_division_tolerance(&mut self, tol: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_polynomial_solvers_univariate_set_division_tolerance(
                sself: *mut core::ffi::c_void,
                tol: core::ffi::c_double,
            );
        }
        unsafe { vtk_polynomial_solvers_univariate_set_division_tolerance(self.0, tol) }
    }
    fn get_division_tolerance(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_polynomial_solvers_univariate_get_division_tolerance(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_polynomial_solvers_univariate_get_division_tolerance(self.0) }
    }
}
impl VtkQuaternionInterpolator for vtkQuaternionInterpolator {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_quaternion_interpolator_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_quaternion_interpolator_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_quaternion_interpolator_new(self.0) }
    }
    fn get_number_of_quaternions(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_get_number_of_quaternions(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_quaternion_interpolator_get_number_of_quaternions(self.0) }
    }
    fn get_minimum_t(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_get_minimum_t(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_quaternion_interpolator_get_minimum_t(self.0) }
    }
    fn get_maximum_t(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_get_maximum_t(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_quaternion_interpolator_get_maximum_t(self.0) }
    }
    fn initialize(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_initialize(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_quaternion_interpolator_initialize(self.0) }
    }
    fn remove_quaternion(&mut self, t: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_remove_quaternion(
                sself: *mut core::ffi::c_void,
                t: core::ffi::c_double,
            );
        }
        unsafe { vtk_quaternion_interpolator_remove_quaternion(self.0, t) }
    }
    fn get_search_method(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_get_search_method(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_quaternion_interpolator_get_search_method(self.0) }
    }
    fn set_search_method(&mut self, type_: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_set_search_method(
                sself: *mut core::ffi::c_void,
                type_: core::ffi::c_int,
            );
        }
        unsafe { vtk_quaternion_interpolator_set_search_method(self.0, type_) }
    }
    fn set_interpolation_type(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_set_interpolation_type(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_quaternion_interpolator_set_interpolation_type(self.0, _arg) }
    }
    fn get_interpolation_type_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_get_interpolation_type_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_quaternion_interpolator_get_interpolation_type_min_value(self.0) }
    }
    fn get_interpolation_type_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_get_interpolation_type_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_quaternion_interpolator_get_interpolation_type_max_value(self.0) }
    }
    fn get_interpolation_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_get_interpolation_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_quaternion_interpolator_get_interpolation_type(self.0) }
    }
    fn set_interpolation_type_to_linear(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_set_interpolation_type_to_linear(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_quaternion_interpolator_set_interpolation_type_to_linear(self.0) }
    }
    fn set_interpolation_type_to_spline(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_quaternion_interpolator_set_interpolation_type_to_spline(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_quaternion_interpolator_set_interpolation_type_to_spline(self.0) }
    }
}
impl VtkRungeKutta2 for vtkRungeKutta2 {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_2_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_2_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_2_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_2_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_2_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_2_new(self.0) }
    }
}
impl VtkRungeKutta4 for vtkRungeKutta4 {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_4_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_4_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_4_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_4_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_4_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_4_new(self.0) }
    }
}
impl VtkRungeKutta45 for vtkRungeKutta45 {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_45_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_45_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_45_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_45_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_runge_kutta_45_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_runge_kutta_45_new(self.0) }
    }
}
/// nonlinear optimization with a simplex
///
///
/// vtkAmoebaMinimizer will modify a set of parameters in order to find
/// the minimum of a specified function.  The method used is commonly
/// known as the amoeba method, it constructs an n-dimensional simplex
/// in parameter space (i.e. a tetrahedron if the number or parameters
/// is 3) and moves the vertices around parameter space until a local
/// minimum is found.  The amoeba method is robust, reasonably efficient,
/// but is not guaranteed to find the global minimum if several local
/// minima exist.
#[allow(non_camel_case_types)]
pub struct vtkAmoebaMinimizer(*mut core::ffi::c_void);
impl vtkAmoebaMinimizer {
    /// Creates a new [vtkAmoebaMinimizer] via `vtkAmoebaMinimizer::New()`
    #[doc(alias = "vtkAmoebaMinimizer")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkAmoebaMinimizer_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkAmoebaMinimizer_new() })
    }
}
impl std::default::Default for vtkAmoebaMinimizer {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkAmoebaMinimizer {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkAmoebaMinimizer_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkAmoebaMinimizer_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkAmoebaMinimizer_create_drop() {
    let obj = vtkAmoebaMinimizer::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// perform Discrete Fourier Transforms
///
///
/// vtkFFT provides methods to perform Discrete Fourier Transforms (DFT).
/// These include providing forward and reverse Fourier transforms.
/// The current implementation uses the third-party library kissfft.
///
/// The terminology tries to follow the Numpy terminology, that is :
/// - Fft means the Fast Fourier Tranform algorithm
/// - Prefix `R` stands for Real (meaning optimized function for real inputs)
/// - Prefix `I` stands for Inverse
#[allow(non_camel_case_types)]
pub struct vtkFFT(*mut core::ffi::c_void);
impl vtkFFT {
    /// Creates a new [vtkFFT] via `vtkFFT::New()`
    #[doc(alias = "vtkFFT")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFFT_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkFFT_new() })
    }
}
impl std::default::Default for vtkFFT {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFFT {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFFT_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFFT_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFFT_create_drop() {
    let obj = vtkFFT::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// represent and manipulate 3x3 transformation matrices
///
///
/// vtkMatrix3x3 is a class to represent and manipulate 3x3 matrices.
/// Specifically, it is designed to work on 3x3 transformation matrices
/// found in 2D rendering using homogeneous coordinates [x y w].
///
/// @sa
/// vtkTransform2D
#[allow(non_camel_case_types)]
pub struct vtkMatrix3x3(*mut core::ffi::c_void);
impl vtkMatrix3x3 {
    /// Creates a new [vtkMatrix3x3] via `vtkMatrix3x3::New()`
    #[doc(alias = "vtkMatrix3x3")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMatrix3x3_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMatrix3x3_new() })
    }
}
impl std::default::Default for vtkMatrix3x3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMatrix3x3 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMatrix3x3_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMatrix3x3_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMatrix3x3_create_drop() {
    let obj = vtkMatrix3x3::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// represent and manipulate 4x4 transformation matrices
///
///
/// vtkMatrix4x4 is a class to represent and manipulate 4x4 matrices.
/// Specifically, it is designed to work on 4x4 transformation matrices
/// found in 3D rendering using homogeneous coordinates [x y z w].
/// Many of the methods take an array of 16 doubles in row-major format.
/// Note that OpenGL stores matrices in column-major format, so the matrix
/// contents must be transposed when they are moved between OpenGL and VTK.
/// @sa
/// vtkTransform
#[allow(non_camel_case_types)]
pub struct vtkMatrix4x4(*mut core::ffi::c_void);
impl vtkMatrix4x4 {
    /// Creates a new [vtkMatrix4x4] via `vtkMatrix4x4::New()`
    #[doc(alias = "vtkMatrix4x4")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMatrix4x4_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMatrix4x4_new() })
    }
}
impl std::default::Default for vtkMatrix4x4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMatrix4x4 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMatrix4x4_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMatrix4x4_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMatrix4x4_create_drop() {
    let obj = vtkMatrix4x4::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// polynomial solvers
///
///
/// vtkPolynomialSolversUnivariate provides solvers for
/// univariate polynomial equations with real coefficients.
/// The Tartaglia-Cardan and Ferrari solvers work on polynomials of fixed
/// degree 3 and 4, respectively.
/// The Lin-Bairstow and Sturm solvers work on polynomials of arbitrary
/// degree. The Sturm solver is the most robust solver but only reports
/// roots within an interval and does not report multiplicities.
/// The Lin-Bairstow solver reports multiplicities.
///
/// For difficult polynomials, you may wish to use FilterRoots to
/// eliminate some of the roots reported by the Sturm solver.
/// FilterRoots evaluates the derivatives near each root to
/// eliminate cases where a local minimum or maximum is close
/// to zero.
///
/// @par Thanks:
/// Thanks to Philippe Pebay, Korben Rusek, David Thompson, and Maurice Rojas
/// for implementing these solvers.
#[allow(non_camel_case_types)]
pub struct vtkPolynomialSolversUnivariate(*mut core::ffi::c_void);
impl vtkPolynomialSolversUnivariate {
    /// Creates a new [vtkPolynomialSolversUnivariate] via `vtkPolynomialSolversUnivariate::New()`
    #[doc(alias = "vtkPolynomialSolversUnivariate")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolynomialSolversUnivariate_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPolynomialSolversUnivariate_new() })
    }
}
impl std::default::Default for vtkPolynomialSolversUnivariate {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolynomialSolversUnivariate {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolynomialSolversUnivariate_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolynomialSolversUnivariate_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolynomialSolversUnivariate_create_drop() {
    let obj = vtkPolynomialSolversUnivariate::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// interpolate a quaternion
///
///
/// This class is used to interpolate a series of quaternions representing
/// the rotations of a 3D object.  The interpolation may be linear in form
/// (using spherical linear interpolation SLERP), or via spline interpolation
/// (using SQUAD). In either case the interpolation is specialized to
/// quaternions since the interpolation occurs on the surface of the unit
/// quaternion sphere.
///
/// To use this class, specify at least two pairs of (t,q[4]) with the
/// AddQuaternion() method.  Next interpolate the tuples with the
/// InterpolateQuaternion(t,q[4]) method, where "t" must be in the range of
/// (t_min,t_max) parameter values specified by the AddQuaternion() method (t
/// is clamped otherwise), and q[4] is filled in by the method.
///
/// There are several important background references. Ken Shoemake described
/// the practical application of quaternions for the interpolation of rotation
/// (K. Shoemake, "Animating rotation with quaternion curves", Computer
/// Graphics (Siggraph '85) 19(3):245--254, 1985). Another fine reference
/// (available on-line) is E. B. Dam, M. Koch, and M. Lillholm, Technical
/// Report DIKU-TR-98/5, Dept. of Computer Science, University of Copenhagen,
/// Denmark.
///
/// @warning
/// Note that for two or less quaternions, Slerp (linear) interpolation is
/// performed even if spline interpolation is requested. Also, the tangents to
/// the first and last segments of spline interpolation are (arbitrarily)
/// defined by repeating the first and last quaternions.
///
/// @warning
/// There are several methods particular to quaternions (norms, products,
/// etc.) implemented interior to this class. These may be moved to a separate
/// quaternion class at some point.
///
/// @sa
/// vtkQuaternion
#[allow(non_camel_case_types)]
pub struct vtkQuaternionInterpolator(*mut core::ffi::c_void);
impl vtkQuaternionInterpolator {
    /// Creates a new [vtkQuaternionInterpolator] via `vtkQuaternionInterpolator::New()`
    #[doc(alias = "vtkQuaternionInterpolator")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkQuaternionInterpolator_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkQuaternionInterpolator_new() })
    }
}
impl std::default::Default for vtkQuaternionInterpolator {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkQuaternionInterpolator {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkQuaternionInterpolator_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkQuaternionInterpolator_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkQuaternionInterpolator_create_drop() {
    let obj = vtkQuaternionInterpolator::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Integrate an initial value problem using 2nd
///
/// order Runge-Kutta method.
///
///
/// This is a concrete sub-class of vtkInitialValueProblemSolver.
/// It uses a 2nd order Runge-Kutta method to obtain the values of
/// a set of functions at the next time step.
///
/// @sa
/// vtkInitialValueProblemSolver vtkRungeKutta4 vtkRungeKutta45 vtkFunctionSet
#[allow(non_camel_case_types)]
pub struct vtkRungeKutta2(*mut core::ffi::c_void);
impl vtkRungeKutta2 {
    /// Creates a new [vtkRungeKutta2] via `vtkRungeKutta2::New()`
    #[doc(alias = "vtkRungeKutta2")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRungeKutta2_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkRungeKutta2_new() })
    }
}
impl std::default::Default for vtkRungeKutta2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRungeKutta2 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRungeKutta2_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRungeKutta2_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRungeKutta2_create_drop() {
    let obj = vtkRungeKutta2::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Integrate an initial value problem using 4th
///
/// order Runge-Kutta method.
///
///
/// This is a concrete sub-class of vtkInitialValueProblemSolver.
/// It uses a 4th order Runge-Kutta method to obtain the values of
/// a set of functions at the next time step.
///
/// @sa
/// vtkInitialValueProblemSolver vtkRungeKutta45 vtkRungeKutta2 vtkFunctionSet
#[allow(non_camel_case_types)]
pub struct vtkRungeKutta4(*mut core::ffi::c_void);
impl vtkRungeKutta4 {
    /// Creates a new [vtkRungeKutta4] via `vtkRungeKutta4::New()`
    #[doc(alias = "vtkRungeKutta4")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRungeKutta4_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkRungeKutta4_new() })
    }
}
impl std::default::Default for vtkRungeKutta4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRungeKutta4 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRungeKutta4_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRungeKutta4_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRungeKutta4_create_drop() {
    let obj = vtkRungeKutta4::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Integrate an initial value problem using 5th
///
/// order Runge-Kutta method with adaptive stepsize control.
///
///
/// This is a concrete sub-class of vtkInitialValueProblemSolver.
/// It uses a 5th order Runge-Kutta method with stepsize control to obtain
/// the values of a set of functions at the next time step. The stepsize
/// is adjusted by calculating an estimated error using an embedded 4th
/// order Runge-Kutta formula:
/// Press, W. H. et al., 1992, Numerical Recipes in Fortran, Second
/// Edition, Cambridge University Press
/// Cash, J.R. and Karp, A.H. 1990, ACM Transactions on Mathematical
/// Software, vol 16, pp 201-222
///
/// @sa
/// vtkInitialValueProblemSolver vtkRungeKutta4 vtkRungeKutta2 vtkFunctionSet
#[allow(non_camel_case_types)]
pub struct vtkRungeKutta45(*mut core::ffi::c_void);
impl vtkRungeKutta45 {
    /// Creates a new [vtkRungeKutta45] via `vtkRungeKutta45::New()`
    #[doc(alias = "vtkRungeKutta45")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRungeKutta45_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkRungeKutta45_new() })
    }
}
impl std::default::Default for vtkRungeKutta45 {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRungeKutta45 {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRungeKutta45_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRungeKutta45_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRungeKutta45_create_drop() {
    let obj = vtkRungeKutta45::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
