pub trait VtkBilinearQuadIntersection {}
pub trait VtkCardinalSpline {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn compute(&mut self) -> ();
    fn evaluate(&mut self, t: core::ffi::c_double) -> core::ffi::c_double;
    fn deep_copy(&mut self, s: *mut core::ffi::c_void) -> ();
}
pub trait VtkKochanekSpline {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn compute(&mut self) -> ();
    fn evaluate(&mut self, t: core::ffi::c_double) -> core::ffi::c_double;
    fn set_default_bias(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_default_bias(&mut self) -> core::ffi::c_double;
    fn set_default_tension(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_default_tension(&mut self) -> core::ffi::c_double;
    fn set_default_continuity(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_default_continuity(&mut self) -> core::ffi::c_double;
    fn deep_copy(&mut self, s: *mut core::ffi::c_void) -> ();
}
pub trait VtkParametricBohemianDome {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_a(&mut self) -> core::ffi::c_double;
    fn set_a(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_b(&mut self) -> core::ffi::c_double;
    fn set_b(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_c(&mut self) -> core::ffi::c_double;
    fn set_c(&mut self, _arg: core::ffi::c_double) -> ();
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricBour {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricBoy {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_z_scale(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_z_scale(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricCatalanMinimal {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricConicSpiral {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_a(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_a(&mut self) -> core::ffi::c_double;
    fn set_b(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_b(&mut self) -> core::ffi::c_double;
    fn set_c(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_c(&mut self) -> core::ffi::c_double;
    fn set_n(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_n(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricCrossCap {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricDini {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_a(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_a(&mut self) -> core::ffi::c_double;
    fn set_b(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_b(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricEllipsoid {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_x_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_x_radius(&mut self) -> core::ffi::c_double;
    fn set_y_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_y_radius(&mut self) -> core::ffi::c_double;
    fn set_z_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_z_radius(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricEnneper {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricFigure8Klein {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricFunction {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_minimum_u(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_minimum_u(&mut self) -> core::ffi::c_double;
    fn set_maximum_u(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_maximum_u(&mut self) -> core::ffi::c_double;
    fn set_minimum_v(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_minimum_v(&mut self) -> core::ffi::c_double;
    fn set_maximum_v(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_maximum_v(&mut self) -> core::ffi::c_double;
    fn set_minimum_w(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_minimum_w(&mut self) -> core::ffi::c_double;
    fn set_maximum_w(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_maximum_w(&mut self) -> core::ffi::c_double;
    fn set_join_u(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_join_u_min_value(&mut self) -> core::ffi::c_int;
    fn get_join_u_max_value(&mut self) -> core::ffi::c_int;
    fn get_join_u(&mut self) -> core::ffi::c_int;
    fn join_u_on(&mut self) -> ();
    fn join_u_off(&mut self) -> ();
    fn set_join_v(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_join_v_min_value(&mut self) -> core::ffi::c_int;
    fn get_join_v_max_value(&mut self) -> core::ffi::c_int;
    fn get_join_v(&mut self) -> core::ffi::c_int;
    fn join_v_on(&mut self) -> ();
    fn join_v_off(&mut self) -> ();
    fn set_join_w(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_join_w_min_value(&mut self) -> core::ffi::c_int;
    fn get_join_w_max_value(&mut self) -> core::ffi::c_int;
    fn get_join_w(&mut self) -> core::ffi::c_int;
    fn join_w_on(&mut self) -> ();
    fn join_w_off(&mut self) -> ();
    fn set_twist_u(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_twist_u_min_value(&mut self) -> core::ffi::c_int;
    fn get_twist_u_max_value(&mut self) -> core::ffi::c_int;
    fn get_twist_u(&mut self) -> core::ffi::c_int;
    fn twist_u_on(&mut self) -> ();
    fn twist_u_off(&mut self) -> ();
    fn set_twist_v(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_twist_v_min_value(&mut self) -> core::ffi::c_int;
    fn get_twist_v_max_value(&mut self) -> core::ffi::c_int;
    fn get_twist_v(&mut self) -> core::ffi::c_int;
    fn twist_v_on(&mut self) -> ();
    fn twist_v_off(&mut self) -> ();
    fn set_twist_w(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_twist_w_min_value(&mut self) -> core::ffi::c_int;
    fn get_twist_w_max_value(&mut self) -> core::ffi::c_int;
    fn get_twist_w(&mut self) -> core::ffi::c_int;
    fn twist_w_on(&mut self) -> ();
    fn twist_w_off(&mut self) -> ();
    fn set_clockwise_ordering(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_clockwise_ordering_min_value(&mut self) -> core::ffi::c_int;
    fn get_clockwise_ordering_max_value(&mut self) -> core::ffi::c_int;
    fn get_clockwise_ordering(&mut self) -> core::ffi::c_int;
    fn clockwise_ordering_on(&mut self) -> ();
    fn clockwise_ordering_off(&mut self) -> ();
    fn set_derivatives_available(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_derivatives_available_min_value(&mut self) -> core::ffi::c_int;
    fn get_derivatives_available_max_value(&mut self) -> core::ffi::c_int;
    fn get_derivatives_available(&mut self) -> core::ffi::c_int;
    fn derivatives_available_on(&mut self) -> ();
    fn derivatives_available_off(&mut self) -> ();
}
pub trait VtkParametricHenneberg {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricKlein {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricKuen {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_delta_v_0(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_delta_v_0(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricMobius {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricPluckerConoid {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_n(&mut self) -> core::ffi::c_int;
    fn set_n(&mut self, _arg: core::ffi::c_int) -> ();
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricPseudosphere {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricRandomHills {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_number_of_hills(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_hills(&mut self) -> core::ffi::c_int;
    fn set_hill_x_variance(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_hill_x_variance(&mut self) -> core::ffi::c_double;
    fn set_hill_y_variance(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_hill_y_variance(&mut self) -> core::ffi::c_double;
    fn set_hill_amplitude(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_hill_amplitude(&mut self) -> core::ffi::c_double;
    fn set_random_seed(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_random_seed(&mut self) -> core::ffi::c_int;
    fn set_allow_random_generation(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_allow_random_generation_min_value(&mut self) -> core::ffi::c_int;
    fn get_allow_random_generation_max_value(&mut self) -> core::ffi::c_int;
    fn get_allow_random_generation(&mut self) -> core::ffi::c_int;
    fn allow_random_generation_on(&mut self) -> ();
    fn allow_random_generation_off(&mut self) -> ();
    fn set_x_variance_scale_factor(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_x_variance_scale_factor(&mut self) -> core::ffi::c_double;
    fn set_y_variance_scale_factor(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_y_variance_scale_factor(&mut self) -> core::ffi::c_double;
    fn set_amplitude_scale_factor(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_amplitude_scale_factor(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricRoman {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricSpline {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_x_spline(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn set_y_spline(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn set_z_spline(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_x_spline(&mut self) -> *mut core::ffi::c_void;
    fn get_y_spline(&mut self) -> *mut core::ffi::c_void;
    fn get_z_spline(&mut self) -> *mut core::ffi::c_void;
    fn set_points(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_points(&mut self) -> *mut core::ffi::c_void;
    fn set_number_of_points(&mut self, numPts: core::ffi::c_longlong) -> ();
    fn set_point(
        &mut self,
        index: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn set_closed(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_closed(&mut self) -> core::ffi::c_int;
    fn closed_on(&mut self) -> ();
    fn closed_off(&mut self) -> ();
    fn set_parameterize_by_length(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_parameterize_by_length(&mut self) -> core::ffi::c_int;
    fn parameterize_by_length_on(&mut self) -> ();
    fn parameterize_by_length_off(&mut self) -> ();
    fn set_left_constraint(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_left_constraint_min_value(&mut self) -> core::ffi::c_int;
    fn get_left_constraint_max_value(&mut self) -> core::ffi::c_int;
    fn get_left_constraint(&mut self) -> core::ffi::c_int;
    fn set_right_constraint(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_right_constraint_min_value(&mut self) -> core::ffi::c_int;
    fn get_right_constraint_max_value(&mut self) -> core::ffi::c_int;
    fn get_right_constraint(&mut self) -> core::ffi::c_int;
    fn set_left_value(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_left_value(&mut self) -> core::ffi::c_double;
    fn set_right_value(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_right_value(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricSuperEllipsoid {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_x_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_x_radius(&mut self) -> core::ffi::c_double;
    fn set_y_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_y_radius(&mut self) -> core::ffi::c_double;
    fn set_z_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_z_radius(&mut self) -> core::ffi::c_double;
    fn set_n_1(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_n_1(&mut self) -> core::ffi::c_double;
    fn set_n_2(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_n_2(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricSuperToroid {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_dimension(&mut self) -> core::ffi::c_int;
    fn set_ring_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_ring_radius(&mut self) -> core::ffi::c_double;
    fn set_cross_section_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_cross_section_radius(&mut self) -> core::ffi::c_double;
    fn set_x_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_x_radius(&mut self) -> core::ffi::c_double;
    fn set_y_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_y_radius(&mut self) -> core::ffi::c_double;
    fn set_z_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_z_radius(&mut self) -> core::ffi::c_double;
    fn set_n_1(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_n_1(&mut self) -> core::ffi::c_double;
    fn set_n_2(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_n_2(&mut self) -> core::ffi::c_double;
}
pub trait VtkParametricTorus {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_ring_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_ring_radius(&mut self) -> core::ffi::c_double;
    fn set_cross_section_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_cross_section_radius(&mut self) -> core::ffi::c_double;
    fn get_dimension(&mut self) -> core::ffi::c_int;
}
impl VtkCardinalSpline for vtkCardinalSpline {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cardinal_spline_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cardinal_spline_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cardinal_spline_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cardinal_spline_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cardinal_spline_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cardinal_spline_new_instance(self.0) }
    }
    fn compute(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_cardinal_spline_compute(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_cardinal_spline_compute(self.0) }
    }
    fn evaluate(&mut self, t: core::ffi::c_double) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cardinal_spline_evaluate(
                sself: *mut core::ffi::c_void,
                t: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cardinal_spline_evaluate(self.0, t) }
    }
    fn deep_copy(&mut self, s: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_cardinal_spline_deep_copy(
                sself: *mut core::ffi::c_void,
                s: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_cardinal_spline_deep_copy(self.0, s) }
    }
}
impl VtkKochanekSpline for vtkKochanekSpline {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_kochanek_spline_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_kochanek_spline_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_kochanek_spline_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_kochanek_spline_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_kochanek_spline_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_kochanek_spline_new(self.0) }
    }
    fn compute(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_kochanek_spline_compute(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_kochanek_spline_compute(self.0) }
    }
    fn evaluate(&mut self, t: core::ffi::c_double) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_kochanek_spline_evaluate(
                sself: *mut core::ffi::c_void,
                t: core::ffi::c_double,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_kochanek_spline_evaluate(self.0, t) }
    }
    fn set_default_bias(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_kochanek_spline_set_default_bias(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_kochanek_spline_set_default_bias(self.0, _arg) }
    }
    fn get_default_bias(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_kochanek_spline_get_default_bias(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_kochanek_spline_get_default_bias(self.0) }
    }
    fn set_default_tension(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_kochanek_spline_set_default_tension(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_kochanek_spline_set_default_tension(self.0, _arg) }
    }
    fn get_default_tension(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_kochanek_spline_get_default_tension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_kochanek_spline_get_default_tension(self.0) }
    }
    fn set_default_continuity(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_kochanek_spline_set_default_continuity(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_kochanek_spline_set_default_continuity(self.0, _arg) }
    }
    fn get_default_continuity(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_kochanek_spline_get_default_continuity(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_kochanek_spline_get_default_continuity(self.0) }
    }
    fn deep_copy(&mut self, s: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_kochanek_spline_deep_copy(
                sself: *mut core::ffi::c_void,
                s: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_kochanek_spline_deep_copy(self.0, s) }
    }
}
impl VtkParametricBohemianDome for vtkParametricBohemianDome {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_bohemian_dome_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_bohemian_dome_new_instance(self.0) }
    }
    fn get_a(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_get_a(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_bohemian_dome_get_a(self.0) }
    }
    fn set_a(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_set_a(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_bohemian_dome_set_a(self.0, _arg) }
    }
    fn get_b(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_get_b(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_bohemian_dome_get_b(self.0) }
    }
    fn set_b(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_set_b(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_bohemian_dome_set_b(self.0, _arg) }
    }
    fn get_c(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_get_c(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_bohemian_dome_get_c(self.0) }
    }
    fn set_c(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_set_c(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_bohemian_dome_set_c(self.0, _arg) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_bohemian_dome_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_bohemian_dome_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_bohemian_dome_get_dimension(self.0) }
    }
}
impl VtkParametricBour for vtkParametricBour {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_bour_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_bour_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_bour_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_bour_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_bour_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_bour_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_bour_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_bour_get_dimension(self.0) }
    }
}
impl VtkParametricBoy for vtkParametricBoy {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_boy_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_boy_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_boy_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_boy_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_boy_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_boy_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_boy_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_boy_get_dimension(self.0) }
    }
    fn set_z_scale(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_boy_set_z_scale(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_boy_set_z_scale(self.0, _arg) }
    }
    fn get_z_scale(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_boy_get_z_scale(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_boy_get_z_scale(self.0) }
    }
}
impl VtkParametricCatalanMinimal for vtkParametricCatalanMinimal {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_catalan_minimal_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_catalan_minimal_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_catalan_minimal_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_catalan_minimal_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_catalan_minimal_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_catalan_minimal_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_catalan_minimal_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_catalan_minimal_get_dimension(self.0) }
    }
}
impl VtkParametricConicSpiral for vtkParametricConicSpiral {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_conic_spiral_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_conic_spiral_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_conic_spiral_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_conic_spiral_get_dimension(self.0) }
    }
    fn set_a(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_set_a(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_conic_spiral_set_a(self.0, _arg) }
    }
    fn get_a(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_get_a(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_conic_spiral_get_a(self.0) }
    }
    fn set_b(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_set_b(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_conic_spiral_set_b(self.0, _arg) }
    }
    fn get_b(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_get_b(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_conic_spiral_get_b(self.0) }
    }
    fn set_c(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_set_c(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_conic_spiral_set_c(self.0, _arg) }
    }
    fn get_c(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_get_c(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_conic_spiral_get_c(self.0) }
    }
    fn set_n(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_set_n(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_conic_spiral_set_n(self.0, _arg) }
    }
    fn get_n(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_conic_spiral_get_n(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_conic_spiral_get_n(self.0) }
    }
}
impl VtkParametricCrossCap for vtkParametricCrossCap {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_cross_cap_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_cross_cap_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_cross_cap_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_cross_cap_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_cross_cap_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_cross_cap_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_cross_cap_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_cross_cap_get_dimension(self.0) }
    }
}
impl VtkParametricDini for vtkParametricDini {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_dini_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_dini_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_dini_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_dini_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_dini_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_dini_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_dini_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_dini_get_dimension(self.0) }
    }
    fn set_a(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_dini_set_a(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_dini_set_a(self.0, _arg) }
    }
    fn get_a(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_dini_get_a(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_dini_get_a(self.0) }
    }
    fn set_b(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_dini_set_b(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_dini_set_b(self.0, _arg) }
    }
    fn get_b(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_dini_get_b(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_dini_get_b(self.0) }
    }
}
impl VtkParametricEllipsoid for vtkParametricEllipsoid {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_ellipsoid_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_ellipsoid_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_ellipsoid_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_ellipsoid_get_dimension(self.0) }
    }
    fn set_x_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_set_x_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_ellipsoid_set_x_radius(self.0, _arg) }
    }
    fn get_x_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_get_x_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_ellipsoid_get_x_radius(self.0) }
    }
    fn set_y_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_set_y_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_ellipsoid_set_y_radius(self.0, _arg) }
    }
    fn get_y_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_get_y_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_ellipsoid_get_y_radius(self.0) }
    }
    fn set_z_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_set_z_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_ellipsoid_set_z_radius(self.0, _arg) }
    }
    fn get_z_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_ellipsoid_get_z_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_ellipsoid_get_z_radius(self.0) }
    }
}
impl VtkParametricEnneper for vtkParametricEnneper {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_enneper_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_enneper_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_enneper_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_enneper_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_enneper_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_enneper_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_enneper_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_enneper_get_dimension(self.0) }
    }
}
impl VtkParametricFigure8Klein for vtkParametricFigure8Klein {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_figure_8_klein_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_figure_8_klein_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_figure_8_klein_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_figure_8_klein_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_figure_8_klein_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_figure_8_klein_new(self.0) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_figure_8_klein_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_figure_8_klein_set_radius(self.0, _arg) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_figure_8_klein_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_figure_8_klein_get_radius(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_figure_8_klein_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_figure_8_klein_get_dimension(self.0) }
    }
}
impl VtkParametricHenneberg for vtkParametricHenneberg {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_henneberg_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_henneberg_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_henneberg_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_henneberg_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_henneberg_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_henneberg_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_henneberg_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_henneberg_get_dimension(self.0) }
    }
}
impl VtkParametricKlein for vtkParametricKlein {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_klein_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_klein_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_klein_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_klein_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_klein_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_klein_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_klein_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_klein_get_dimension(self.0) }
    }
}
impl VtkParametricKuen for vtkParametricKuen {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_kuen_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_kuen_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_kuen_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_kuen_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_kuen_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_kuen_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_kuen_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_kuen_get_dimension(self.0) }
    }
    fn set_delta_v_0(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_kuen_set_delta_v_0(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_kuen_set_delta_v_0(self.0, _arg) }
    }
    fn get_delta_v_0(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_kuen_get_delta_v_0(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_kuen_get_delta_v_0(self.0) }
    }
}
impl VtkParametricMobius for vtkParametricMobius {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_mobius_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_mobius_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_mobius_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_mobius_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_mobius_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_mobius_new(self.0) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_mobius_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_mobius_set_radius(self.0, _arg) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_mobius_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_mobius_get_radius(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_mobius_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_mobius_get_dimension(self.0) }
    }
}
impl VtkParametricPluckerConoid for vtkParametricPluckerConoid {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_plucker_conoid_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_plucker_conoid_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_plucker_conoid_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_plucker_conoid_new_instance(self.0) }
    }
    fn get_n(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_plucker_conoid_get_n(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_plucker_conoid_get_n(self.0) }
    }
    fn set_n(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_plucker_conoid_set_n(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_plucker_conoid_set_n(self.0, _arg) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_plucker_conoid_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_plucker_conoid_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_plucker_conoid_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_plucker_conoid_get_dimension(self.0) }
    }
}
impl VtkParametricPseudosphere for vtkParametricPseudosphere {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_pseudosphere_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_pseudosphere_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_pseudosphere_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_pseudosphere_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_pseudosphere_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_pseudosphere_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_pseudosphere_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_pseudosphere_get_dimension(self.0) }
    }
}
impl VtkParametricRandomHills for vtkParametricRandomHills {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_random_hills_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_random_hills_new_instance(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_random_hills_get_dimension(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_random_hills_new(self.0) }
    }
    fn set_number_of_hills(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_number_of_hills(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_random_hills_set_number_of_hills(self.0, _arg) }
    }
    fn get_number_of_hills(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_number_of_hills(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_random_hills_get_number_of_hills(self.0) }
    }
    fn set_hill_x_variance(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_hill_x_variance(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_random_hills_set_hill_x_variance(self.0, _arg) }
    }
    fn get_hill_x_variance(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_hill_x_variance(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_random_hills_get_hill_x_variance(self.0) }
    }
    fn set_hill_y_variance(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_hill_y_variance(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_random_hills_set_hill_y_variance(self.0, _arg) }
    }
    fn get_hill_y_variance(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_hill_y_variance(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_random_hills_get_hill_y_variance(self.0) }
    }
    fn set_hill_amplitude(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_hill_amplitude(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_random_hills_set_hill_amplitude(self.0, _arg) }
    }
    fn get_hill_amplitude(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_hill_amplitude(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_random_hills_get_hill_amplitude(self.0) }
    }
    fn set_random_seed(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_random_seed(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_random_hills_set_random_seed(self.0, _arg) }
    }
    fn get_random_seed(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_random_seed(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_random_hills_get_random_seed(self.0) }
    }
    fn set_allow_random_generation(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_allow_random_generation(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_random_hills_set_allow_random_generation(self.0, _arg) }
    }
    fn get_allow_random_generation_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_allow_random_generation_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_parametric_random_hills_get_allow_random_generation_min_value(self.0)
        }
    }
    fn get_allow_random_generation_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_allow_random_generation_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_parametric_random_hills_get_allow_random_generation_max_value(self.0)
        }
    }
    fn get_allow_random_generation(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_allow_random_generation(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_random_hills_get_allow_random_generation(self.0) }
    }
    fn allow_random_generation_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_allow_random_generation_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_random_hills_allow_random_generation_on(self.0) }
    }
    fn allow_random_generation_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_allow_random_generation_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_random_hills_allow_random_generation_off(self.0) }
    }
    fn set_x_variance_scale_factor(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_x_variance_scale_factor(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_random_hills_set_x_variance_scale_factor(self.0, _arg) }
    }
    fn get_x_variance_scale_factor(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_x_variance_scale_factor(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_random_hills_get_x_variance_scale_factor(self.0) }
    }
    fn set_y_variance_scale_factor(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_y_variance_scale_factor(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_random_hills_set_y_variance_scale_factor(self.0, _arg) }
    }
    fn get_y_variance_scale_factor(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_y_variance_scale_factor(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_random_hills_get_y_variance_scale_factor(self.0) }
    }
    fn set_amplitude_scale_factor(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_set_amplitude_scale_factor(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_random_hills_set_amplitude_scale_factor(self.0, _arg) }
    }
    fn get_amplitude_scale_factor(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_random_hills_get_amplitude_scale_factor(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_random_hills_get_amplitude_scale_factor(self.0) }
    }
}
impl VtkParametricRoman for vtkParametricRoman {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_roman_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_roman_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_roman_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_roman_new_instance(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_roman_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_roman_get_dimension(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_roman_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_roman_new(self.0) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_roman_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_roman_set_radius(self.0, _arg) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_roman_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_roman_get_radius(self.0) }
    }
}
impl VtkParametricSpline for vtkParametricSpline {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_spline_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_spline_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_spline_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_spline_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_spline_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_spline_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_dimension(self.0) }
    }
    fn set_x_spline(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_x_spline(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_spline_set_x_spline(self.0, p0) }
    }
    fn set_y_spline(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_y_spline(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_spline_set_y_spline(self.0, p0) }
    }
    fn set_z_spline(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_z_spline(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_spline_set_z_spline(self.0, p0) }
    }
    fn get_x_spline(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_x_spline(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_spline_get_x_spline(self.0) }
    }
    fn get_y_spline(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_y_spline(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_spline_get_y_spline(self.0) }
    }
    fn get_z_spline(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_z_spline(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_spline_get_z_spline(self.0) }
    }
    fn set_points(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_points(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_spline_set_points(self.0, p0) }
    }
    fn get_points(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_points(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_spline_get_points(self.0) }
    }
    fn set_number_of_points(&mut self, numPts: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_number_of_points(
                sself: *mut core::ffi::c_void,
                numPts: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_parametric_spline_set_number_of_points(self.0, numPts) }
    }
    fn set_point(
        &mut self,
        index: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_point(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_longlong,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_spline_set_point(self.0, index, x, y, z) }
    }
    fn set_closed(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_closed(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_spline_set_closed(self.0, _arg) }
    }
    fn get_closed(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_closed(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_closed(self.0) }
    }
    fn closed_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_closed_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_parametric_spline_closed_on(self.0) }
    }
    fn closed_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_closed_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_parametric_spline_closed_off(self.0) }
    }
    fn set_parameterize_by_length(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_parameterize_by_length(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_spline_set_parameterize_by_length(self.0, _arg) }
    }
    fn get_parameterize_by_length(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_parameterize_by_length(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_parameterize_by_length(self.0) }
    }
    fn parameterize_by_length_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_parameterize_by_length_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_spline_parameterize_by_length_on(self.0) }
    }
    fn parameterize_by_length_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_parameterize_by_length_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_spline_parameterize_by_length_off(self.0) }
    }
    fn set_left_constraint(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_left_constraint(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_spline_set_left_constraint(self.0, _arg) }
    }
    fn get_left_constraint_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_left_constraint_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_left_constraint_min_value(self.0) }
    }
    fn get_left_constraint_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_left_constraint_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_left_constraint_max_value(self.0) }
    }
    fn get_left_constraint(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_left_constraint(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_left_constraint(self.0) }
    }
    fn set_right_constraint(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_right_constraint(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_spline_set_right_constraint(self.0, _arg) }
    }
    fn get_right_constraint_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_right_constraint_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_right_constraint_min_value(self.0) }
    }
    fn get_right_constraint_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_right_constraint_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_right_constraint_max_value(self.0) }
    }
    fn get_right_constraint(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_right_constraint(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_spline_get_right_constraint(self.0) }
    }
    fn set_left_value(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_left_value(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_spline_set_left_value(self.0, _arg) }
    }
    fn get_left_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_left_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_spline_get_left_value(self.0) }
    }
    fn set_right_value(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_spline_set_right_value(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_spline_set_right_value(self.0, _arg) }
    }
    fn get_right_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_spline_get_right_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_spline_get_right_value(self.0) }
    }
}
impl VtkParametricSuperEllipsoid for vtkParametricSuperEllipsoid {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_super_ellipsoid_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_super_ellipsoid_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_super_ellipsoid_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_super_ellipsoid_get_dimension(self.0) }
    }
    fn set_x_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_set_x_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_ellipsoid_set_x_radius(self.0, _arg) }
    }
    fn get_x_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_get_x_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_ellipsoid_get_x_radius(self.0) }
    }
    fn set_y_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_set_y_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_ellipsoid_set_y_radius(self.0, _arg) }
    }
    fn get_y_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_get_y_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_ellipsoid_get_y_radius(self.0) }
    }
    fn set_z_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_set_z_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_ellipsoid_set_z_radius(self.0, _arg) }
    }
    fn get_z_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_get_z_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_ellipsoid_get_z_radius(self.0) }
    }
    fn set_n_1(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_set_n_1(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_ellipsoid_set_n_1(self.0, _arg) }
    }
    fn get_n_1(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_get_n_1(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_ellipsoid_get_n_1(self.0) }
    }
    fn set_n_2(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_set_n_2(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_ellipsoid_set_n_2(self.0, _arg) }
    }
    fn get_n_2(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_ellipsoid_get_n_2(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_ellipsoid_get_n_2(self.0) }
    }
}
impl VtkParametricSuperToroid for vtkParametricSuperToroid {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_super_toroid_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_super_toroid_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_super_toroid_new(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_super_toroid_get_dimension(self.0) }
    }
    fn set_ring_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_set_ring_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_toroid_set_ring_radius(self.0, _arg) }
    }
    fn get_ring_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_get_ring_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_toroid_get_ring_radius(self.0) }
    }
    fn set_cross_section_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_set_cross_section_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_toroid_set_cross_section_radius(self.0, _arg) }
    }
    fn get_cross_section_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_get_cross_section_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_toroid_get_cross_section_radius(self.0) }
    }
    fn set_x_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_set_x_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_toroid_set_x_radius(self.0, _arg) }
    }
    fn get_x_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_get_x_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_toroid_get_x_radius(self.0) }
    }
    fn set_y_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_set_y_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_toroid_set_y_radius(self.0, _arg) }
    }
    fn get_y_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_get_y_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_toroid_get_y_radius(self.0) }
    }
    fn set_z_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_set_z_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_toroid_set_z_radius(self.0, _arg) }
    }
    fn get_z_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_get_z_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_toroid_get_z_radius(self.0) }
    }
    fn set_n_1(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_set_n_1(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_toroid_set_n_1(self.0, _arg) }
    }
    fn get_n_1(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_get_n_1(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_toroid_get_n_1(self.0) }
    }
    fn set_n_2(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_set_n_2(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_super_toroid_set_n_2(self.0, _arg) }
    }
    fn get_n_2(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_super_toroid_get_n_2(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_super_toroid_get_n_2(self.0) }
    }
}
impl VtkParametricTorus for vtkParametricTorus {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_torus_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_torus_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_torus_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_torus_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_torus_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_torus_new(self.0) }
    }
    fn set_ring_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_torus_set_ring_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_torus_set_ring_radius(self.0, _arg) }
    }
    fn get_ring_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_torus_get_ring_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_torus_get_ring_radius(self.0) }
    }
    fn set_cross_section_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_parametric_torus_set_cross_section_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_parametric_torus_set_cross_section_radius(self.0, _arg) }
    }
    fn get_cross_section_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_parametric_torus_get_cross_section_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_parametric_torus_get_cross_section_radius(self.0) }
    }
    fn get_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_torus_get_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_torus_get_dimension(self.0) }
    }
}
/// computes an interpolating spline using a
///
/// a Cardinal basis.
///
///
/// vtkCardinalSpline is a concrete implementation of vtkSpline using a
/// Cardinal basis.
///
/// @sa
/// vtkSpline vtkKochanekSpline
#[allow(non_camel_case_types)]
pub struct vtkCardinalSpline(*mut core::ffi::c_void);
impl vtkCardinalSpline {
    /// Creates a new [vtkCardinalSpline] via `vtkCardinalSpline::New()`
    #[doc(alias = "vtkCardinalSpline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCardinalSpline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCardinalSpline_new() })
    }
}
impl std::default::Default for vtkCardinalSpline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCardinalSpline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCardinalSpline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCardinalSpline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCardinalSpline_create_drop() {
    let obj = vtkCardinalSpline::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// computes an interpolating spline using a Kochanek basis.
///
///
/// Implements the Kochanek interpolating spline described in: Kochanek, D.,
/// Bartels, R., "Interpolating Splines with Local Tension, Continuity, and
/// Bias Control," Computer Graphics, vol. 18, no. 3, pp. 33-41, July 1984.
/// These splines give the user more control over the shape of the curve than
/// the cardinal splines implemented in vtkCardinalSpline. Three parameters
/// can be specified. All have a range from -1 to 1.
///
/// Tension controls how sharply the curve bends at an input point. A
/// value of -1 produces more slack in the curve. A value of 1 tightens
/// the curve.
///
/// Continuity controls the continuity of the first derivative at input
/// points.
///
/// Bias controls the direction of the curve at it passes through an input
/// point. A value of -1 undershoots the point while a value of 1
/// overshoots the point.
///
/// These three parameters give the user broad control over the shape of
/// the interpolating spline. The original Kochanek paper describes the
/// effects nicely and is recommended reading.
///
/// @sa
/// vtkSpline vtkCardinalSpline
#[allow(non_camel_case_types)]
pub struct vtkKochanekSpline(*mut core::ffi::c_void);
impl vtkKochanekSpline {
    /// Creates a new [vtkKochanekSpline] via `vtkKochanekSpline::New()`
    #[doc(alias = "vtkKochanekSpline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkKochanekSpline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkKochanekSpline_new() })
    }
}
impl std::default::Default for vtkKochanekSpline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkKochanekSpline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkKochanekSpline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkKochanekSpline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkKochanekSpline_create_drop() {
    let obj = vtkKochanekSpline::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a Bohemian dome.
///
///
/// vtkParametricBohemianDome generates a parametric Bohemian dome. The Bohemian
/// dome is a quartic surface, and is described in much better detail at
/// <a href="https://www.math.hmc.edu/math142-01/mellon/curves_and_surfaces/surfaces/bohdom.html">HMC
/// page</a>.
/// @warning
/// I haven't set any restrictions on the A, B, or C values.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricBohemianDome(*mut core::ffi::c_void);
impl vtkParametricBohemianDome {
    /// Creates a new [vtkParametricBohemianDome] via `vtkParametricBohemianDome::New()`
    #[doc(alias = "vtkParametricBohemianDome")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricBohemianDome_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricBohemianDome_new() })
    }
}
impl std::default::Default for vtkParametricBohemianDome {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricBohemianDome {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricBohemianDome_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricBohemianDome_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricBohemianDome_create_drop() {
    let obj = vtkParametricBohemianDome::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Bour's minimal surface.
///
///
/// vtkParametricBour generates Bour's minimal surface parametrically. More
/// information can be found at
/// <a href="http://en.wikipedia.org/wiki/Bour%27s_minimal_surface">Wikipedia</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricBour(*mut core::ffi::c_void);
impl vtkParametricBour {
    /// Creates a new [vtkParametricBour] via `vtkParametricBour::New()`
    #[doc(alias = "vtkParametricBour")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricBour_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricBour_new() })
    }
}
impl std::default::Default for vtkParametricBour {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricBour {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricBour_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricBour_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricBour_create_drop() {
    let obj = vtkParametricBour::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Boy's surface.
///
///
/// vtkParametricBoy generates Boy's surface.
/// This is a Model of the projective plane without singularities.
/// It was found by Werner Boy on assignment from David Hilbert.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricBoy(*mut core::ffi::c_void);
impl vtkParametricBoy {
    /// Creates a new [vtkParametricBoy] via `vtkParametricBoy::New()`
    #[doc(alias = "vtkParametricBoy")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricBoy_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricBoy_new() })
    }
}
impl std::default::Default for vtkParametricBoy {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricBoy {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricBoy_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricBoy_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricBoy_create_drop() {
    let obj = vtkParametricBoy::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Catalan's minimal surface.
///
///
/// vtkParametricCatalanMinimal generates Catalan's minimal surface
/// parametrically. This minimal surface contains the cycloid as a geodesic.
/// More information about it can be found at
/// <a href="https://en.wikipedia.org/wiki/Catalan%27s_minimal_surface">Wikipedia</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricCatalanMinimal(*mut core::ffi::c_void);
impl vtkParametricCatalanMinimal {
    /// Creates a new [vtkParametricCatalanMinimal] via `vtkParametricCatalanMinimal::New()`
    #[doc(alias = "vtkParametricCatalanMinimal")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricCatalanMinimal_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricCatalanMinimal_new() })
    }
}
impl std::default::Default for vtkParametricCatalanMinimal {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricCatalanMinimal {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricCatalanMinimal_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricCatalanMinimal_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricCatalanMinimal_create_drop() {
    let obj = vtkParametricCatalanMinimal::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate conic spiral surfaces that resemble sea-shells.
///
///
/// vtkParametricConicSpiral generates conic spiral surfaces. These can resemble sea shells, or
/// may look like a torus "eating" its own tail.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricConicSpiral(*mut core::ffi::c_void);
impl vtkParametricConicSpiral {
    /// Creates a new [vtkParametricConicSpiral] via `vtkParametricConicSpiral::New()`
    #[doc(alias = "vtkParametricConicSpiral")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricConicSpiral_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricConicSpiral_new() })
    }
}
impl std::default::Default for vtkParametricConicSpiral {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricConicSpiral {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricConicSpiral_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricConicSpiral_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricConicSpiral_create_drop() {
    let obj = vtkParametricConicSpiral::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a cross-cap.
///
///
/// vtkParametricCrossCap generates a cross-cap which is a
/// non-orientable self-intersecting single-sided surface.
/// This is one possible image of a projective plane in three-space.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricCrossCap(*mut core::ffi::c_void);
impl vtkParametricCrossCap {
    /// Creates a new [vtkParametricCrossCap] via `vtkParametricCrossCap::New()`
    #[doc(alias = "vtkParametricCrossCap")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricCrossCap_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricCrossCap_new() })
    }
}
impl std::default::Default for vtkParametricCrossCap {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricCrossCap {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricCrossCap_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricCrossCap_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricCrossCap_create_drop() {
    let obj = vtkParametricCrossCap::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Dini's surface.
///
///
/// vtkParametricDini generates Dini's surface.
/// Dini's surface is a surface that possesses constant negative
/// Gaussian curvature
///
/// For further information about this surface, please consult
/// https://en.wikipedia.org/wiki/Dini%27s_surface
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricDini(*mut core::ffi::c_void);
impl vtkParametricDini {
    /// Creates a new [vtkParametricDini] via `vtkParametricDini::New()`
    #[doc(alias = "vtkParametricDini")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricDini_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricDini_new() })
    }
}
impl std::default::Default for vtkParametricDini {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricDini {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricDini_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricDini_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricDini_create_drop() {
    let obj = vtkParametricDini::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate an ellipsoid.
///
///
/// vtkParametricEllipsoid generates an ellipsoid.
/// If all the radii are the same, we have a sphere.
/// An oblate spheroid occurs if RadiusX = RadiusY > RadiusZ.
/// Here the Z-axis forms the symmetry axis. To a first
/// approximation, this is the shape of the earth.
/// A prolate spheroid occurs if RadiusX = RadiusY < RadiusZ.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricEllipsoid(*mut core::ffi::c_void);
impl vtkParametricEllipsoid {
    /// Creates a new [vtkParametricEllipsoid] via `vtkParametricEllipsoid::New()`
    #[doc(alias = "vtkParametricEllipsoid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricEllipsoid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricEllipsoid_new() })
    }
}
impl std::default::Default for vtkParametricEllipsoid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricEllipsoid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricEllipsoid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricEllipsoid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricEllipsoid_create_drop() {
    let obj = vtkParametricEllipsoid::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Enneper's surface.
///
///
/// vtkParametricEnneper generates Enneper's surface.
/// Enneper's surface is a a self-intersecting minimal surface
/// possessing constant negative Gaussian curvature
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricEnneper(*mut core::ffi::c_void);
impl vtkParametricEnneper {
    /// Creates a new [vtkParametricEnneper] via `vtkParametricEnneper::New()`
    #[doc(alias = "vtkParametricEnneper")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricEnneper_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricEnneper_new() })
    }
}
impl std::default::Default for vtkParametricEnneper {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricEnneper {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricEnneper_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricEnneper_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricEnneper_create_drop() {
    let obj = vtkParametricEnneper::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a figure-8 Klein bottle.
///
///
/// vtkParametricFigure8Klein generates a figure-8 Klein bottle.  A Klein bottle
/// is a closed surface with no interior and only one surface.  It is
/// unrealisable in 3 dimensions without intersecting surfaces.  It can be
/// realised in 4 dimensions by considering the map \f$F:R^2 \rightarrow R^4\f$  given by:
///
/// - \f$f(u,v) = ((r*cos(v)+a)*cos(u),(r*cos(v)+a)*sin(u),r*sin(v)*cos(u/2),r*sin(v)*sin(u/2))\f$
///
/// This representation of the immersion in \f$R^3\f$ is formed by taking two Mobius
/// strips and joining them along their boundaries, this is the so called
/// "Figure-8 Klein Bottle"
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricFigure8Klein(*mut core::ffi::c_void);
impl vtkParametricFigure8Klein {
    /// Creates a new [vtkParametricFigure8Klein] via `vtkParametricFigure8Klein::New()`
    #[doc(alias = "vtkParametricFigure8Klein")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricFigure8Klein_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricFigure8Klein_new() })
    }
}
impl std::default::Default for vtkParametricFigure8Klein {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricFigure8Klein {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricFigure8Klein_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricFigure8Klein_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricFigure8Klein_create_drop() {
    let obj = vtkParametricFigure8Klein::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Henneberg's minimal surface.
///
///
/// vtkParametricHenneberg generates Henneberg's minimal surface parametrically.
/// Henneberg's minimal surface is discussed further at
/// <a href="http://mathworld.wolfram.com/HennebergsMinimalSurface.html">Math World</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricHenneberg(*mut core::ffi::c_void);
impl vtkParametricHenneberg {
    /// Creates a new [vtkParametricHenneberg] via `vtkParametricHenneberg::New()`
    #[doc(alias = "vtkParametricHenneberg")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricHenneberg_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricHenneberg_new() })
    }
}
impl std::default::Default for vtkParametricHenneberg {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricHenneberg {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricHenneberg_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricHenneberg_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricHenneberg_create_drop() {
    let obj = vtkParametricHenneberg::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generates a "classical" representation of a Klein bottle.
///
///
/// vtkParametricKlein generates a "classical" representation of a Klein
/// bottle.  A Klein bottle is a closed surface with no interior and only one
/// surface.  It is unrealisable in 3 dimensions without intersecting
/// surfaces.  It can be
/// realised in 4 dimensions by considering the map \f$F:R^2 \rightarrow R^4\f$  given by:
///
/// - \f$f(u,v) = ((r*cos(v)+a)*cos(u),(r*cos(v)+a)*sin(u),r*sin(v)*cos(u/2),r*sin(v)*sin(u/2))\f$
///
/// The classical representation of the immersion in \f$R^3\f$ is returned by this function.
///
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricKlein(*mut core::ffi::c_void);
impl vtkParametricKlein {
    /// Creates a new [vtkParametricKlein] via `vtkParametricKlein::New()`
    #[doc(alias = "vtkParametricKlein")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricKlein_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricKlein_new() })
    }
}
impl std::default::Default for vtkParametricKlein {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricKlein {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricKlein_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricKlein_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricKlein_create_drop() {
    let obj = vtkParametricKlein::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Kuens' surface.
///
///
/// vtkParametricKuen generates Kuens' surface. This surface has a constant
/// negative gaussian curvature. For more information about this surface, see
/// Dr. O'Niell's page at the
/// <a href="http://www.math.ucla.edu/~bon/kuen.html">UCLA Mathematics Department</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricKuen(*mut core::ffi::c_void);
impl vtkParametricKuen {
    /// Creates a new [vtkParametricKuen] via `vtkParametricKuen::New()`
    #[doc(alias = "vtkParametricKuen")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricKuen_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricKuen_new() })
    }
}
impl std::default::Default for vtkParametricKuen {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricKuen {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricKuen_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricKuen_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricKuen_create_drop() {
    let obj = vtkParametricKuen::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a Mobius strip.
///
///
/// vtkParametricMobius generates a Mobius strip.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricMobius(*mut core::ffi::c_void);
impl vtkParametricMobius {
    /// Creates a new [vtkParametricMobius] via `vtkParametricMobius::New()`
    #[doc(alias = "vtkParametricMobius")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricMobius_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricMobius_new() })
    }
}
impl std::default::Default for vtkParametricMobius {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricMobius {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricMobius_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricMobius_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricMobius_create_drop() {
    let obj = vtkParametricMobius::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Plucker's conoid surface.
///
///
/// vtkParametricPluckerConoid generates Plucker's conoid surface parametrically.
/// Plucker's conoid is a ruled surface, named after Julius Plucker. It is
/// possible to set the number of folds in this class via the parameter 'N'.
///
/// For more information, see the Wikipedia page on
/// <a href="https://en.wikipedia.org/wiki/Pl%c3%bccker%27s_conoid">Plucker's Conoid</a>.
/// @warning
/// I haven't done any special checking on the number of folds parameter, N.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricPluckerConoid(*mut core::ffi::c_void);
impl vtkParametricPluckerConoid {
    /// Creates a new [vtkParametricPluckerConoid] via `vtkParametricPluckerConoid::New()`
    #[doc(alias = "vtkParametricPluckerConoid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricPluckerConoid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricPluckerConoid_new() })
    }
}
impl std::default::Default for vtkParametricPluckerConoid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricPluckerConoid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricPluckerConoid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricPluckerConoid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricPluckerConoid_create_drop() {
    let obj = vtkParametricPluckerConoid::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a pseudosphere.
///
///
/// vtkParametricPseudosphere generates a parametric pseudosphere. The
/// pseudosphere is generated as a surface of revolution of the tractrix about
/// it's asymptote, and is a surface of constant negative Gaussian curvature.
/// You can find out more about this interesting surface at
/// <a href="http://mathworld.wolfram.com/Pseudosphere.html">Math World</a>.
/// @par Thanks:
/// Tim Meehan
#[allow(non_camel_case_types)]
pub struct vtkParametricPseudosphere(*mut core::ffi::c_void);
impl vtkParametricPseudosphere {
    /// Creates a new [vtkParametricPseudosphere] via `vtkParametricPseudosphere::New()`
    #[doc(alias = "vtkParametricPseudosphere")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricPseudosphere_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricPseudosphere_new() })
    }
}
impl std::default::Default for vtkParametricPseudosphere {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricPseudosphere {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricPseudosphere_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricPseudosphere_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricPseudosphere_create_drop() {
    let obj = vtkParametricPseudosphere::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a surface covered with randomly placed hills.
///
///
/// vtkParametricRandomHills generates a surface covered with randomly placed
/// hills. Hills will vary in shape and height since the presence
/// of nearby hills will contribute to the shape and height of a given hill.
/// An option is provided for placing hills on a regular grid on the surface.
/// In this case the hills will all have the same shape and height.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricRandomHills(*mut core::ffi::c_void);
impl vtkParametricRandomHills {
    /// Creates a new [vtkParametricRandomHills] via `vtkParametricRandomHills::New()`
    #[doc(alias = "vtkParametricRandomHills")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricRandomHills_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricRandomHills_new() })
    }
}
impl std::default::Default for vtkParametricRandomHills {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricRandomHills {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricRandomHills_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricRandomHills_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricRandomHills_create_drop() {
    let obj = vtkParametricRandomHills::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate Steiner's Roman Surface.
///
///
/// vtkParametricRoman generates Steiner's Roman Surface.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricRoman(*mut core::ffi::c_void);
impl vtkParametricRoman {
    /// Creates a new [vtkParametricRoman] via `vtkParametricRoman::New()`
    #[doc(alias = "vtkParametricRoman")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricRoman_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricRoman_new() })
    }
}
impl std::default::Default for vtkParametricRoman {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricRoman {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricRoman_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricRoman_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricRoman_create_drop() {
    let obj = vtkParametricRoman::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// parametric function for 1D interpolating splines
///
///
/// vtkParametricSpline is a parametric function for 1D interpolating splines.
/// vtkParametricSpline maps the single parameter u into a 3D point (x,y,z)
/// using three instances of interpolating splines.  This family of 1D splines
/// is guaranteed to be parameterized in the interval [0,1].  Attempting to
/// evaluate outside this interval will cause the parameter u to be clamped in
/// the range [0,1].
///
/// When constructed, this class creates instances of vtkCardinalSpline for
/// each of the x-y-z coordinates. The user may choose to replace these with
/// their own instances of subclasses of vtkSpline.
///
/// @warning
/// If you wish to tessellate the spline, use the class
/// vtkParametricFunctionSource.
///
/// @sa
/// vtkSpline vtkKochanekSpline vtkCardinalSpline
#[allow(non_camel_case_types)]
pub struct vtkParametricSpline(*mut core::ffi::c_void);
impl vtkParametricSpline {
    /// Creates a new [vtkParametricSpline] via `vtkParametricSpline::New()`
    #[doc(alias = "vtkParametricSpline")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricSpline_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricSpline_new() })
    }
}
impl std::default::Default for vtkParametricSpline {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricSpline {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricSpline_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricSpline_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricSpline_create_drop() {
    let obj = vtkParametricSpline::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a superellipsoid.
///
///
/// vtkParametricSuperEllipsoid generates a superellipsoid.  A superellipsoid
/// is a versatile primitive that is controlled by two parameters n1 and
/// n2. As special cases it can represent a sphere, square box, and closed
/// cylindrical can.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// Also see: http://paulbourke.net/geometry/superellipse/
///
/// @warning
/// Care needs to be taken specifying the bounds correctly. You may need to
/// carefully adjust MinimumU, MinimumV, MaximumU, MaximumV.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricSuperEllipsoid(*mut core::ffi::c_void);
impl vtkParametricSuperEllipsoid {
    /// Creates a new [vtkParametricSuperEllipsoid] via `vtkParametricSuperEllipsoid::New()`
    #[doc(alias = "vtkParametricSuperEllipsoid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricSuperEllipsoid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricSuperEllipsoid_new() })
    }
}
impl std::default::Default for vtkParametricSuperEllipsoid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricSuperEllipsoid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricSuperEllipsoid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricSuperEllipsoid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricSuperEllipsoid_create_drop() {
    let obj = vtkParametricSuperEllipsoid::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a supertoroid.
///
///
/// vtkParametricSuperToroid generates a supertoroid.  Essentially a
/// supertoroid is a torus with the sine and cosine terms raised to a power.
/// A supertoroid is a versatile primitive that is controlled by four
/// parameters r0, r1, n1 and n2. r0, r1 determine the type of torus whilst
/// the value of n1 determines the shape of the torus ring and n2 determines
/// the shape of the cross section of the ring. It is the different values of
/// these powers which give rise to a family of 3D shapes that are all
/// basically toroidal in shape.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// Also see: http://paulbourke.net/geometry/torus/#super.
///
/// @warning
/// Care needs to be taken specifying the bounds correctly. You may need to
/// carefully adjust MinimumU, MinimumV, MaximumU, MaximumV.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricSuperToroid(*mut core::ffi::c_void);
impl vtkParametricSuperToroid {
    /// Creates a new [vtkParametricSuperToroid] via `vtkParametricSuperToroid::New()`
    #[doc(alias = "vtkParametricSuperToroid")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricSuperToroid_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricSuperToroid_new() })
    }
}
impl std::default::Default for vtkParametricSuperToroid {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricSuperToroid {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricSuperToroid_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricSuperToroid_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricSuperToroid_create_drop() {
    let obj = vtkParametricSuperToroid::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a torus.
///
///
/// vtkParametricTorus generates a torus.
///
/// For further information about this surface, please consult the
/// technical description "Parametric surfaces" in http://www.vtk.org/publications
/// in the "VTK Technical Documents" section in the VTk.org web pages.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing the
/// class.
#[allow(non_camel_case_types)]
pub struct vtkParametricTorus(*mut core::ffi::c_void);
impl vtkParametricTorus {
    /// Creates a new [vtkParametricTorus] via `vtkParametricTorus::New()`
    #[doc(alias = "vtkParametricTorus")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricTorus_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricTorus_new() })
    }
}
impl std::default::Default for vtkParametricTorus {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricTorus {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricTorus_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricTorus_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricTorus_create_drop() {
    let obj = vtkParametricTorus::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
