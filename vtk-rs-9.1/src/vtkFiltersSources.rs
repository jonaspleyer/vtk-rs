pub trait VtkArcSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_point_1(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_point_2(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_normal(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_polar_vector(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_angle(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_angle_min_value(&mut self) -> core::ffi::c_double;
    fn get_angle_max_value(&mut self) -> core::ffi::c_double;
    fn get_angle(&mut self) -> core::ffi::c_double;
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_resolution(&mut self) -> core::ffi::c_int;
    fn set_negative(&mut self, _arg: bool) -> ();
    fn get_negative(&mut self) -> bool;
    fn negative_on(&mut self) -> ();
    fn negative_off(&mut self) -> ();
    fn set_use_normal_and_angle(&mut self, _arg: bool) -> ();
    fn get_use_normal_and_angle(&mut self) -> bool;
    fn use_normal_and_angle_on(&mut self) -> ();
    fn use_normal_and_angle_off(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkArrowSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_tip_length(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_tip_length_min_value(&mut self) -> core::ffi::c_double;
    fn get_tip_length_max_value(&mut self) -> core::ffi::c_double;
    fn get_tip_length(&mut self) -> core::ffi::c_double;
    fn set_tip_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_tip_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_tip_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_tip_radius(&mut self) -> core::ffi::c_double;
    fn set_tip_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_tip_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_tip_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_tip_resolution(&mut self) -> core::ffi::c_int;
    fn set_shaft_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_shaft_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_shaft_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_shaft_radius(&mut self) -> core::ffi::c_double;
    fn set_shaft_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_shaft_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_shaft_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_shaft_resolution(&mut self) -> core::ffi::c_int;
    fn invert_on(&mut self) -> ();
    fn invert_off(&mut self) -> ();
    fn set_invert(&mut self, _arg: bool) -> ();
    fn get_invert(&mut self) -> bool;
    fn set_arrow_origin_to_default(&mut self) -> ();
    fn set_arrow_origin_to_center(&mut self) -> ();
}
pub trait VtkButtonSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_texture_style(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_texture_style_min_value(&mut self) -> core::ffi::c_int;
    fn get_texture_style_max_value(&mut self) -> core::ffi::c_int;
    fn get_texture_style(&mut self) -> core::ffi::c_int;
    fn set_texture_style_to_fit_image(&mut self) -> ();
    fn set_texture_style_to_proportional(&mut self) -> ();
    fn set_texture_dimensions(
        &mut self,
        _arg1: core::ffi::c_int,
        _arg2: core::ffi::c_int,
    ) -> ();
    fn set_shoulder_texture_coordinate(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
    ) -> ();
    fn set_two_sided(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_two_sided(&mut self) -> core::ffi::c_int;
    fn two_sided_on(&mut self) -> ();
    fn two_sided_off(&mut self) -> ();
}
pub trait VtkCapsuleSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_cylinder_length(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_cylinder_length_min_value(&mut self) -> core::ffi::c_double;
    fn get_cylinder_length_max_value(&mut self) -> core::ffi::c_double;
    fn get_cylinder_length(&mut self) -> core::ffi::c_double;
    fn set_theta_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_theta_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_theta_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_theta_resolution(&mut self) -> core::ffi::c_int;
    fn set_phi_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_phi_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_phi_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_phi_resolution(&mut self) -> core::ffi::c_int;
    fn set_lat_long_tessellation(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_lat_long_tessellation(&mut self) -> core::ffi::c_int;
    fn lat_long_tessellation_on(&mut self) -> ();
    fn lat_long_tessellation_off(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkCellTypeSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_cell_type(&mut self, cellType: core::ffi::c_int) -> ();
    fn get_cell_type(&mut self) -> core::ffi::c_int;
    fn set_cell_order(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_cell_order(&mut self) -> core::ffi::c_int;
    fn set_complete_quadratic_simplicial_elements(&mut self, _arg: bool) -> ();
    fn get_complete_quadratic_simplicial_elements(&mut self) -> bool;
    fn complete_quadratic_simplicial_elements_on(&mut self) -> ();
    fn complete_quadratic_simplicial_elements_off(&mut self) -> ();
    fn set_polynomial_field_order(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_polynomial_field_order_min_value(&mut self) -> core::ffi::c_int;
    fn get_polynomial_field_order_max_value(&mut self) -> core::ffi::c_int;
    fn get_polynomial_field_order(&mut self) -> core::ffi::c_int;
    fn get_cell_dimension(&mut self) -> core::ffi::c_int;
    fn set_output_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_precision_min_value(&mut self) -> core::ffi::c_int;
    fn get_output_precision_max_value(&mut self) -> core::ffi::c_int;
    fn get_output_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkConeSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_height(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_height_min_value(&mut self) -> core::ffi::c_double;
    fn get_height_max_value(&mut self) -> core::ffi::c_double;
    fn get_height(&mut self) -> core::ffi::c_double;
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_resolution(&mut self) -> core::ffi::c_int;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_direction(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_angle(&mut self, angle: core::ffi::c_double) -> ();
    fn get_angle(&mut self) -> core::ffi::c_double;
    fn set_capping(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_capping(&mut self) -> core::ffi::c_int;
    fn capping_on(&mut self) -> ();
    fn capping_off(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkCubeSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_x_length(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_x_length_min_value(&mut self) -> core::ffi::c_double;
    fn get_x_length_max_value(&mut self) -> core::ffi::c_double;
    fn get_x_length(&mut self) -> core::ffi::c_double;
    fn set_y_length(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_y_length_min_value(&mut self) -> core::ffi::c_double;
    fn get_y_length_max_value(&mut self) -> core::ffi::c_double;
    fn get_y_length(&mut self) -> core::ffi::c_double;
    fn set_z_length(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_z_length_min_value(&mut self) -> core::ffi::c_double;
    fn get_z_length_max_value(&mut self) -> core::ffi::c_double;
    fn get_z_length(&mut self) -> core::ffi::c_double;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_bounds(
        &mut self,
        xMin: core::ffi::c_double,
        xMax: core::ffi::c_double,
        yMin: core::ffi::c_double,
        yMax: core::ffi::c_double,
        zMin: core::ffi::c_double,
        zMax: core::ffi::c_double,
    ) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkCylinderSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_height(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_height_min_value(&mut self) -> core::ffi::c_double;
    fn get_height_max_value(&mut self) -> core::ffi::c_double;
    fn get_height(&mut self) -> core::ffi::c_double;
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_resolution(&mut self) -> core::ffi::c_int;
    fn set_capping(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_capping(&mut self) -> core::ffi::c_int;
    fn capping_on(&mut self) -> ();
    fn capping_off(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkDiagonalMatrixSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_array_type(&mut self) -> core::ffi::c_int;
    fn set_array_type(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_extents(&mut self) -> core::ffi::c_longlong;
    fn set_extents(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_diagonal(&mut self) -> core::ffi::c_double;
    fn set_diagonal(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_super_diagonal(&mut self) -> core::ffi::c_double;
    fn set_super_diagonal(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_sub_diagonal(&mut self) -> core::ffi::c_double;
    fn set_sub_diagonal(&mut self, _arg: core::ffi::c_double) -> ();
    fn set_row_label(&mut self, _arg: &str) -> ();
    fn set_column_label(&mut self, _arg: &str) -> ();
}
pub trait VtkDiskSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_inner_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_inner_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_inner_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_inner_radius(&mut self) -> core::ffi::c_double;
    fn set_outer_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_outer_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_outer_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_outer_radius(&mut self) -> core::ffi::c_double;
    fn set_radial_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_radial_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_radial_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_radial_resolution(&mut self) -> core::ffi::c_int;
    fn set_circumferential_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_circumferential_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_circumferential_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_circumferential_resolution(&mut self) -> core::ffi::c_int;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkEllipseArcSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_normal(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_major_radius_vector(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_start_angle(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_start_angle_min_value(&mut self) -> core::ffi::c_double;
    fn get_start_angle_max_value(&mut self) -> core::ffi::c_double;
    fn get_start_angle(&mut self) -> core::ffi::c_double;
    fn set_segment_angle(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_segment_angle_min_value(&mut self) -> core::ffi::c_double;
    fn get_segment_angle_max_value(&mut self) -> core::ffi::c_double;
    fn get_segment_angle(&mut self) -> core::ffi::c_double;
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_resolution(&mut self) -> core::ffi::c_int;
    fn set_close(&mut self, _arg: bool) -> ();
    fn get_close(&mut self) -> bool;
    fn close_on(&mut self) -> ();
    fn close_off(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
    fn set_ratio(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_ratio_min_value(&mut self) -> core::ffi::c_double;
    fn get_ratio_max_value(&mut self) -> core::ffi::c_double;
    fn get_ratio(&mut self) -> core::ffi::c_double;
}
pub trait VtkEllipticalButtonSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_width(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_width_min_value(&mut self) -> core::ffi::c_double;
    fn get_width_max_value(&mut self) -> core::ffi::c_double;
    fn get_width(&mut self) -> core::ffi::c_double;
    fn set_height(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_height_min_value(&mut self) -> core::ffi::c_double;
    fn get_height_max_value(&mut self) -> core::ffi::c_double;
    fn get_height(&mut self) -> core::ffi::c_double;
    fn set_depth(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_depth_min_value(&mut self) -> core::ffi::c_double;
    fn get_depth_max_value(&mut self) -> core::ffi::c_double;
    fn get_depth(&mut self) -> core::ffi::c_double;
    fn set_circumferential_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_circumferential_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_circumferential_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_circumferential_resolution(&mut self) -> core::ffi::c_int;
    fn set_texture_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_texture_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_texture_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_texture_resolution(&mut self) -> core::ffi::c_int;
    fn set_shoulder_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_shoulder_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_shoulder_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_shoulder_resolution(&mut self) -> core::ffi::c_int;
    fn set_radial_ratio(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radial_ratio_min_value(&mut self) -> core::ffi::c_double;
    fn get_radial_ratio_max_value(&mut self) -> core::ffi::c_double;
    fn get_radial_ratio(&mut self) -> core::ffi::c_double;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkFrustumSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_planes(&mut self) -> *mut core::ffi::c_void;
    fn set_planes(&mut self, planes: *mut core::ffi::c_void) -> ();
    fn get_show_lines(&mut self) -> bool;
    fn set_show_lines(&mut self, _arg: bool) -> ();
    fn show_lines_on(&mut self) -> ();
    fn show_lines_off(&mut self) -> ();
    fn get_lines_length(&mut self) -> core::ffi::c_double;
    fn set_lines_length(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkGlyphSource2D {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_scale(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_scale_min_value(&mut self) -> core::ffi::c_double;
    fn get_scale_max_value(&mut self) -> core::ffi::c_double;
    fn get_scale(&mut self) -> core::ffi::c_double;
    fn set_scale_2(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_scale_2_min_value(&mut self) -> core::ffi::c_double;
    fn get_scale_2_max_value(&mut self) -> core::ffi::c_double;
    fn get_scale_2(&mut self) -> core::ffi::c_double;
    fn set_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_filled(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_filled(&mut self) -> core::ffi::c_int;
    fn filled_on(&mut self) -> ();
    fn filled_off(&mut self) -> ();
    fn set_dash(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_dash(&mut self) -> core::ffi::c_int;
    fn dash_on(&mut self) -> ();
    fn dash_off(&mut self) -> ();
    fn set_cross(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_cross(&mut self) -> core::ffi::c_int;
    fn cross_on(&mut self) -> ();
    fn cross_off(&mut self) -> ();
    fn set_rotation_angle(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_rotation_angle(&mut self) -> core::ffi::c_double;
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_resolution(&mut self) -> core::ffi::c_int;
    fn set_glyph_type(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_glyph_type_min_value(&mut self) -> core::ffi::c_int;
    fn get_glyph_type_max_value(&mut self) -> core::ffi::c_int;
    fn get_glyph_type(&mut self) -> core::ffi::c_int;
    fn set_glyph_type_to_none(&mut self) -> ();
    fn set_glyph_type_to_vertex(&mut self) -> ();
    fn set_glyph_type_to_dash(&mut self) -> ();
    fn set_glyph_type_to_cross(&mut self) -> ();
    fn set_glyph_type_to_thick_cross(&mut self) -> ();
    fn set_glyph_type_to_triangle(&mut self) -> ();
    fn set_glyph_type_to_square(&mut self) -> ();
    fn set_glyph_type_to_circle(&mut self) -> ();
    fn set_glyph_type_to_diamond(&mut self) -> ();
    fn set_glyph_type_to_arrow(&mut self) -> ();
    fn set_glyph_type_to_thick_arrow(&mut self) -> ();
    fn set_glyph_type_to_hooked_arrow(&mut self) -> ();
    fn set_glyph_type_to_edge_arrow(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkGraphToPolyData {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_edge_glyph_output(&mut self, _arg: bool) -> ();
    fn get_edge_glyph_output(&mut self) -> bool;
    fn edge_glyph_output_on(&mut self) -> ();
    fn edge_glyph_output_off(&mut self) -> ();
    fn set_edge_glyph_position(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_edge_glyph_position(&mut self) -> core::ffi::c_double;
}
pub trait VtkHandleSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_directional(&mut self, _arg: bool) -> ();
    fn get_directional(&mut self) -> bool;
    fn directional_on(&mut self) -> ();
    fn directional_off(&mut self) -> ();
    fn set_position(
        &mut self,
        xPos: core::ffi::c_double,
        yPos: core::ffi::c_double,
        zPos: core::ffi::c_double,
    ) -> ();
    fn set_direction(
        &mut self,
        xDir: core::ffi::c_double,
        yDir: core::ffi::c_double,
        zDir: core::ffi::c_double,
    ) -> ();
    fn set_size(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_size(&mut self) -> core::ffi::c_double;
}
pub trait VtkHyperTreeGridSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_maximum_level(&mut self) -> core::ffi::c_uint;
    fn set_maximum_level(&mut self, levels: core::ffi::c_uint) -> ();
    fn get_max_depth(&mut self) -> core::ffi::c_uint;
    fn set_max_depth(&mut self, levels: core::ffi::c_uint) -> ();
    fn set_origin(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_grid_scale(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_transposed_root_indexing(&mut self, _arg: bool) -> ();
    fn get_transposed_root_indexing(&mut self) -> bool;
    fn set_indexing_mode_to_kji(&mut self) -> ();
    fn set_indexing_mode_to_ijk(&mut self) -> ();
    fn get_orientation(&mut self) -> core::ffi::c_uint;
    fn set_branch_factor(&mut self, _arg: core::ffi::c_uint) -> ();
    fn get_branch_factor_min_value(&mut self) -> core::ffi::c_uint;
    fn get_branch_factor_max_value(&mut self) -> core::ffi::c_uint;
    fn get_branch_factor(&mut self) -> core::ffi::c_uint;
    fn set_use_descriptor(&mut self, _arg: bool) -> ();
    fn get_use_descriptor(&mut self) -> bool;
    fn use_descriptor_on(&mut self) -> ();
    fn use_descriptor_off(&mut self) -> ();
    fn set_use_mask(&mut self, _arg: bool) -> ();
    fn get_use_mask(&mut self) -> bool;
    fn use_mask_on(&mut self) -> ();
    fn use_mask_off(&mut self) -> ();
    fn set_generate_interface_fields(&mut self, _arg: bool) -> ();
    fn get_generate_interface_fields(&mut self) -> bool;
    fn generate_interface_fields_on(&mut self) -> ();
    fn generate_interface_fields_off(&mut self) -> ();
    fn set_descriptor(&mut self, _arg: &str) -> ();
    fn set_mask(&mut self, _arg: &str) -> ();
    fn set_descriptor_bits(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_descriptor_bits(&mut self) -> *mut core::ffi::c_void;
    fn set_level_zero_material_index(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn set_mask_bits(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_mask_bits(&mut self) -> *mut core::ffi::c_void;
    fn set_quadric(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_quadric(&mut self) -> *mut core::ffi::c_void;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn convert_descriptor_string_to_bit_array(
        &mut self,
        p0: &str,
    ) -> *mut core::ffi::c_void;
    fn convert_mask_string_to_bit_array(&mut self, p0: &str) -> *mut core::ffi::c_void;
}
pub trait VtkLineSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_point_1(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_point_2(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_use_regular_refinement(&mut self, _arg: bool) -> ();
    fn get_use_regular_refinement(&mut self) -> bool;
    fn use_regular_refinement_on(&mut self) -> ();
    fn use_regular_refinement_off(&mut self) -> ();
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_resolution(&mut self) -> core::ffi::c_int;
    fn set_number_of_refinement_ratios(&mut self, p0: core::ffi::c_int) -> ();
    fn set_refinement_ratio(
        &mut self,
        index: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> ();
    fn get_number_of_refinement_ratios(&mut self) -> core::ffi::c_int;
    fn get_refinement_ratio(&mut self, index: core::ffi::c_int) -> core::ffi::c_double;
    fn set_points(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_points(&mut self) -> *mut core::ffi::c_void;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkOutlineCornerFilter {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_corner_factor(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_corner_factor_min_value(&mut self) -> core::ffi::c_double;
    fn get_corner_factor_max_value(&mut self) -> core::ffi::c_double;
    fn get_corner_factor(&mut self) -> core::ffi::c_double;
}
pub trait VtkOutlineCornerSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_corner_factor(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_corner_factor_min_value(&mut self) -> core::ffi::c_double;
    fn get_corner_factor_max_value(&mut self) -> core::ffi::c_double;
    fn get_corner_factor(&mut self) -> core::ffi::c_double;
}
pub trait VtkOutlineSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_box_type(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_box_type(&mut self) -> core::ffi::c_int;
    fn set_box_type_to_axis_aligned(&mut self) -> ();
    fn set_box_type_to_oriented(&mut self) -> ();
    fn set_bounds(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
        _arg5: core::ffi::c_double,
        _arg6: core::ffi::c_double,
    ) -> ();
    fn set_generate_faces(&mut self, _arg: core::ffi::c_int) -> ();
    fn generate_faces_on(&mut self) -> ();
    fn generate_faces_off(&mut self) -> ();
    fn get_generate_faces(&mut self) -> core::ffi::c_int;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkParametricFunctionSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_parametric_function(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_parametric_function(&mut self) -> *mut core::ffi::c_void;
    fn set_u_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_u_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_u_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_u_resolution(&mut self) -> core::ffi::c_int;
    fn set_v_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_v_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_v_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_v_resolution(&mut self) -> core::ffi::c_int;
    fn set_w_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_w_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_w_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_w_resolution(&mut self) -> core::ffi::c_int;
    fn generate_texture_coordinates_on(&mut self) -> ();
    fn generate_texture_coordinates_off(&mut self) -> ();
    fn set_generate_texture_coordinates(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_generate_texture_coordinates_min_value(&mut self) -> core::ffi::c_int;
    fn get_generate_texture_coordinates_max_value(&mut self) -> core::ffi::c_int;
    fn get_generate_texture_coordinates(&mut self) -> core::ffi::c_int;
    fn generate_normals_on(&mut self) -> ();
    fn generate_normals_off(&mut self) -> ();
    fn set_generate_normals(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_generate_normals_min_value(&mut self) -> core::ffi::c_int;
    fn get_generate_normals_max_value(&mut self) -> core::ffi::c_int;
    fn get_generate_normals(&mut self) -> core::ffi::c_int;
    fn set_scalar_mode(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_scalar_mode_min_value(&mut self) -> core::ffi::c_int;
    fn get_scalar_mode_max_value(&mut self) -> core::ffi::c_int;
    fn get_scalar_mode(&mut self) -> core::ffi::c_int;
    fn set_scalar_mode_to_none(&mut self) -> ();
    fn set_scalar_mode_to_u(&mut self) -> ();
    fn set_scalar_mode_to_v(&mut self) -> ();
    fn set_scalar_mode_to_u_0(&mut self) -> ();
    fn set_scalar_mode_to_v_0(&mut self) -> ();
    fn set_scalar_mode_to_u_0_v_0(&mut self) -> ();
    fn set_scalar_mode_to_modulus(&mut self) -> ();
    fn set_scalar_mode_to_phase(&mut self) -> ();
    fn set_scalar_mode_to_quadrant(&mut self) -> ();
    fn set_scalar_mode_to_x(&mut self) -> ();
    fn set_scalar_mode_to_y(&mut self) -> ();
    fn set_scalar_mode_to_z(&mut self) -> ();
    fn set_scalar_mode_to_distance(&mut self) -> ();
    fn set_scalar_mode_to_function_defined(&mut self) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkPartitionedDataSetCollectionSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_number_of_shapes(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_shapes_min_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_shapes_max_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_shapes(&mut self) -> core::ffi::c_int;
}
pub trait VtkPartitionedDataSetSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn enable_rank(&mut self, rank: core::ffi::c_int) -> ();
    fn enable_all_ranks(&mut self) -> ();
    fn disable_rank(&mut self, rank: core::ffi::c_int) -> ();
    fn disable_all_ranks(&mut self) -> ();
    fn is_enabled_rank(&mut self, rank: core::ffi::c_int) -> bool;
    fn set_number_of_partitions(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_partitions_min_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_partitions_max_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_partitions(&mut self) -> core::ffi::c_int;
    fn set_parametric_function(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_parametric_function(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkPlaneSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_x_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_x_resolution(&mut self) -> core::ffi::c_int;
    fn set_y_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_y_resolution(&mut self) -> core::ffi::c_int;
    fn set_resolution(&mut self, xR: core::ffi::c_int, yR: core::ffi::c_int) -> ();
    fn get_resolution(
        &mut self,
        xR: &mut core::ffi::c_int,
        yR: &mut core::ffi::c_int,
    ) -> ();
    fn set_origin(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_point_1(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn set_point_2(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn set_center(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn set_normal(
        &mut self,
        nx: core::ffi::c_double,
        ny: core::ffi::c_double,
        nz: core::ffi::c_double,
    ) -> ();
    fn push(&mut self, distance: core::ffi::c_double) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkPlatonicSolidSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_solid_type(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_solid_type_min_value(&mut self) -> core::ffi::c_int;
    fn get_solid_type_max_value(&mut self) -> core::ffi::c_int;
    fn get_solid_type(&mut self) -> core::ffi::c_int;
    fn set_solid_type_to_tetrahedron(&mut self) -> ();
    fn set_solid_type_to_cube(&mut self) -> ();
    fn set_solid_type_to_octahedron(&mut self) -> ();
    fn set_solid_type_to_icosahedron(&mut self) -> ();
    fn set_solid_type_to_dodecahedron(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkPointHandleSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_position(
        &mut self,
        xPos: core::ffi::c_double,
        yPos: core::ffi::c_double,
        zPos: core::ffi::c_double,
    ) -> ();
    fn set_direction(
        &mut self,
        xDir: core::ffi::c_double,
        yDir: core::ffi::c_double,
        zDir: core::ffi::c_double,
    ) -> ();
}
pub trait VtkPointSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_number_of_points(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_number_of_points_min_value(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_points_max_value(&mut self) -> core::ffi::c_longlong;
    fn get_number_of_points(&mut self) -> core::ffi::c_longlong;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn set_distribution(&mut self, _arg: core::ffi::c_int) -> ();
    fn set_distribution_to_uniform(&mut self) -> ();
    fn set_distribution_to_shell(&mut self) -> ();
    fn get_distribution(&mut self) -> core::ffi::c_int;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
    fn set_random_sequence(&mut self, randomSequence: *mut core::ffi::c_void) -> ();
    fn get_random_sequence(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkPolyLineSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_closed(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_closed(&mut self) -> core::ffi::c_int;
    fn closed_on(&mut self) -> ();
    fn closed_off(&mut self) -> ();
}
pub trait VtkPolyPointSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_number_of_points(&mut self, numPoints: core::ffi::c_longlong) -> ();
    fn get_number_of_points(&mut self) -> core::ffi::c_longlong;
    fn resize(&mut self, numPoints: core::ffi::c_longlong) -> ();
    fn set_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn set_points(&mut self, points: *mut core::ffi::c_void) -> ();
    fn get_points(&mut self) -> *mut core::ffi::c_void;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkProgrammableDataObjectSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_execute_method_arg_delete(&mut self, f: *mut core::ffi::c_void) -> ();
}
pub trait VtkProgrammableSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_execute_method_arg_delete(&mut self, f: *mut core::ffi::c_void) -> ();
    fn set_request_information_method(&mut self, f: *mut core::ffi::c_void) -> ();
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_points_output(&mut self) -> *mut core::ffi::c_void;
    fn get_structured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_rectilinear_grid_output(&mut self) -> *mut core::ffi::c_void;
    fn get_graph_output(&mut self) -> *mut core::ffi::c_void;
    fn get_molecule_output(&mut self) -> *mut core::ffi::c_void;
    fn get_table_output(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkRandomHyperTreeGridSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_dimensions(
        &mut self,
        _arg1: core::ffi::c_uint,
        _arg2: core::ffi::c_uint,
        _arg3: core::ffi::c_uint,
    ) -> ();
    fn set_output_bounds(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
        _arg5: core::ffi::c_double,
        _arg6: core::ffi::c_double,
    ) -> ();
    fn get_seed(&mut self) -> core::ffi::c_uint;
    fn set_seed(&mut self, _arg: core::ffi::c_uint) -> ();
    fn get_max_depth(&mut self) -> core::ffi::c_longlong;
    fn set_max_depth(&mut self, _arg: core::ffi::c_longlong) -> ();
    fn get_max_depth_min_value(&mut self) -> core::ffi::c_longlong;
    fn get_max_depth_max_value(&mut self) -> core::ffi::c_longlong;
    fn get_split_fraction(&mut self) -> core::ffi::c_double;
    fn set_split_fraction(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_split_fraction_min_value(&mut self) -> core::ffi::c_double;
    fn get_split_fraction_max_value(&mut self) -> core::ffi::c_double;
}
pub trait VtkRectangularButtonSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_width(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_width_min_value(&mut self) -> core::ffi::c_double;
    fn get_width_max_value(&mut self) -> core::ffi::c_double;
    fn get_width(&mut self) -> core::ffi::c_double;
    fn set_height(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_height_min_value(&mut self) -> core::ffi::c_double;
    fn get_height_max_value(&mut self) -> core::ffi::c_double;
    fn get_height(&mut self) -> core::ffi::c_double;
    fn set_depth(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_depth_min_value(&mut self) -> core::ffi::c_double;
    fn get_depth_max_value(&mut self) -> core::ffi::c_double;
    fn get_depth(&mut self) -> core::ffi::c_double;
    fn set_box_ratio(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_box_ratio_min_value(&mut self) -> core::ffi::c_double;
    fn get_box_ratio_max_value(&mut self) -> core::ffi::c_double;
    fn get_box_ratio(&mut self) -> core::ffi::c_double;
    fn set_texture_ratio(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_texture_ratio_min_value(&mut self) -> core::ffi::c_double;
    fn get_texture_ratio_max_value(&mut self) -> core::ffi::c_double;
    fn get_texture_ratio(&mut self) -> core::ffi::c_double;
    fn set_texture_height_ratio(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_texture_height_ratio_min_value(&mut self) -> core::ffi::c_double;
    fn get_texture_height_ratio_max_value(&mut self) -> core::ffi::c_double;
    fn get_texture_height_ratio(&mut self) -> core::ffi::c_double;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkRegularPolygonSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_number_of_sides(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_sides_min_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_sides_max_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_sides(&mut self) -> core::ffi::c_int;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_normal(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn set_generate_polygon(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_generate_polygon(&mut self) -> core::ffi::c_int;
    fn generate_polygon_on(&mut self) -> ();
    fn generate_polygon_off(&mut self) -> ();
    fn set_generate_polyline(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_generate_polyline(&mut self) -> core::ffi::c_int;
    fn generate_polyline_on(&mut self) -> ();
    fn generate_polyline_off(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkSelectionSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn add_id(&mut self, piece: core::ffi::c_longlong, id: core::ffi::c_longlong) -> ();
    fn add_string_id(&mut self, piece: core::ffi::c_longlong, id: &str) -> ();
    fn add_location(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn add_threshold(
        &mut self,
        min: core::ffi::c_double,
        max: core::ffi::c_double,
    ) -> ();
    fn add_block(&mut self, blockno: core::ffi::c_longlong) -> ();
    fn add_block_selector(&mut self, selector: &str) -> ();
    fn remove_all_block_selectors(&mut self) -> ();
    fn remove_all_i_ds(&mut self) -> ();
    fn remove_all_string_i_ds(&mut self) -> ();
    fn remove_all_thresholds(&mut self) -> ();
    fn remove_all_locations(&mut self) -> ();
    fn remove_all_blocks(&mut self) -> ();
    fn set_content_type(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_content_type(&mut self) -> core::ffi::c_int;
    fn set_field_type(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_field_type(&mut self) -> core::ffi::c_int;
    fn set_containing_cells(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_containing_cells(&mut self) -> core::ffi::c_int;
    fn set_number_of_layers(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_number_of_layers_min_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_layers_max_value(&mut self) -> core::ffi::c_int;
    fn get_number_of_layers(&mut self) -> core::ffi::c_int;
    fn set_inverse(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_inverse(&mut self) -> core::ffi::c_int;
    fn set_array_name(&mut self, _arg: &str) -> ();
    fn set_array_component(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_array_component(&mut self) -> core::ffi::c_int;
    fn set_composite_index(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_composite_index(&mut self) -> core::ffi::c_int;
    fn set_hierarchical_level(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_hierarchical_level(&mut self) -> core::ffi::c_int;
    fn set_hierarchical_index(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_hierarchical_index(&mut self) -> core::ffi::c_int;
    fn set_assembly_name(&mut self, _arg: &str) -> ();
    fn add_selector(&mut self, selector: &str) -> ();
    fn remove_all_selectors(&mut self) -> ();
    fn set_query_string(&mut self, _arg: &str) -> ();
}
pub trait VtkSphereSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_theta_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_theta_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_theta_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_theta_resolution(&mut self) -> core::ffi::c_int;
    fn set_phi_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_phi_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_phi_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_phi_resolution(&mut self) -> core::ffi::c_int;
    fn set_start_theta(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_start_theta_min_value(&mut self) -> core::ffi::c_double;
    fn get_start_theta_max_value(&mut self) -> core::ffi::c_double;
    fn get_start_theta(&mut self) -> core::ffi::c_double;
    fn set_end_theta(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_end_theta_min_value(&mut self) -> core::ffi::c_double;
    fn get_end_theta_max_value(&mut self) -> core::ffi::c_double;
    fn get_end_theta(&mut self) -> core::ffi::c_double;
    fn set_start_phi(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_start_phi_min_value(&mut self) -> core::ffi::c_double;
    fn get_start_phi_max_value(&mut self) -> core::ffi::c_double;
    fn get_start_phi(&mut self) -> core::ffi::c_double;
    fn set_end_phi(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_end_phi_min_value(&mut self) -> core::ffi::c_double;
    fn get_end_phi_max_value(&mut self) -> core::ffi::c_double;
    fn get_end_phi(&mut self) -> core::ffi::c_double;
    fn set_lat_long_tessellation(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_lat_long_tessellation(&mut self) -> core::ffi::c_int;
    fn lat_long_tessellation_on(&mut self) -> ();
    fn lat_long_tessellation_off(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
    fn set_generate_normals(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_generate_normals(&mut self) -> core::ffi::c_int;
    fn generate_normals_on(&mut self) -> ();
    fn generate_normals_off(&mut self) -> ();
}
pub trait VtkSuperquadricSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_scale(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn get_theta_resolution(&mut self) -> core::ffi::c_int;
    fn set_theta_resolution(&mut self, i: core::ffi::c_int) -> ();
    fn get_phi_resolution(&mut self) -> core::ffi::c_int;
    fn set_phi_resolution(&mut self, i: core::ffi::c_int) -> ();
    fn get_thickness(&mut self) -> core::ffi::c_double;
    fn set_thickness(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_thickness_min_value(&mut self) -> core::ffi::c_double;
    fn get_thickness_max_value(&mut self) -> core::ffi::c_double;
    fn get_phi_roundness(&mut self) -> core::ffi::c_double;
    fn set_phi_roundness(&mut self, e: core::ffi::c_double) -> ();
    fn get_theta_roundness(&mut self) -> core::ffi::c_double;
    fn set_theta_roundness(&mut self, e: core::ffi::c_double) -> ();
    fn set_size(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_size(&mut self) -> core::ffi::c_double;
    fn set_axis_of_symmetry(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_axis_of_symmetry(&mut self) -> core::ffi::c_int;
    fn set_x_axis_of_symmetry(&mut self) -> ();
    fn set_y_axis_of_symmetry(&mut self) -> ();
    fn set_z_axis_of_symmetry(&mut self) -> ();
    fn toroidal_on(&mut self) -> ();
    fn toroidal_off(&mut self) -> ();
    fn get_toroidal(&mut self) -> core::ffi::c_int;
    fn set_toroidal(&mut self, _arg: core::ffi::c_int) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkTessellatedBoxSource {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_bounds(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
        _arg5: core::ffi::c_double,
        _arg6: core::ffi::c_double,
    ) -> ();
    fn set_level(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_level(&mut self) -> core::ffi::c_int;
    fn set_duplicate_shared_points(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_duplicate_shared_points(&mut self) -> core::ffi::c_int;
    fn duplicate_shared_points_on(&mut self) -> ();
    fn duplicate_shared_points_off(&mut self) -> ();
    fn set_quads(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_quads(&mut self) -> core::ffi::c_int;
    fn quads_on(&mut self) -> ();
    fn quads_off(&mut self) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkTextSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_text(&mut self, _arg: &str) -> ();
    fn set_backing(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_backing(&mut self) -> core::ffi::c_int;
    fn backing_on(&mut self) -> ();
    fn backing_off(&mut self) -> ();
    fn set_foreground_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_background_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> ();
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkTexturedSphereSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_radius_min_value(&mut self) -> core::ffi::c_double;
    fn get_radius_max_value(&mut self) -> core::ffi::c_double;
    fn get_radius(&mut self) -> core::ffi::c_double;
    fn set_theta_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_theta_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_theta_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_theta_resolution(&mut self) -> core::ffi::c_int;
    fn set_phi_resolution(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_phi_resolution_min_value(&mut self) -> core::ffi::c_int;
    fn get_phi_resolution_max_value(&mut self) -> core::ffi::c_int;
    fn get_phi_resolution(&mut self) -> core::ffi::c_int;
    fn set_theta(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_theta_min_value(&mut self) -> core::ffi::c_double;
    fn get_theta_max_value(&mut self) -> core::ffi::c_double;
    fn get_theta(&mut self) -> core::ffi::c_double;
    fn set_phi(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_phi_min_value(&mut self) -> core::ffi::c_double;
    fn get_phi_max_value(&mut self) -> core::ffi::c_double;
    fn get_phi(&mut self) -> core::ffi::c_double;
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_output_points_precision(&mut self) -> core::ffi::c_int;
}
pub trait VtkUniformHyperTreeGridSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
}
impl VtkArcSource for vtkArcSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_arc_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_arc_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_arc_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_arc_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_arc_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_arc_source_new_instance(self.0) }
    }
    fn set_point_1(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_point_1(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_arc_source_set_point_1(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_point_2(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_point_2(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_arc_source_set_point_2(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_arc_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_normal(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_normal(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_arc_source_set_normal(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_polar_vector(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_polar_vector(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_arc_source_set_polar_vector(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_angle(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_angle(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_arc_source_set_angle(self.0, _arg) }
    }
    fn get_angle_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arc_source_get_angle_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arc_source_get_angle_min_value(self.0) }
    }
    fn get_angle_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arc_source_get_angle_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arc_source_get_angle_max_value(self.0) }
    }
    fn get_angle(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arc_source_get_angle(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arc_source_get_angle(self.0) }
    }
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_arc_source_set_resolution(self.0, _arg) }
    }
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arc_source_get_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arc_source_get_resolution_min_value(self.0) }
    }
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arc_source_get_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arc_source_get_resolution_max_value(self.0) }
    }
    fn get_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arc_source_get_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arc_source_get_resolution(self.0) }
    }
    fn set_negative(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_negative(sself: *mut core::ffi::c_void, _arg: bool);
        }
        unsafe { vtk_arc_source_set_negative(self.0, _arg) }
    }
    fn get_negative(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_arc_source_get_negative(sself: *mut core::ffi::c_void) -> bool;
        }
        unsafe { vtk_arc_source_get_negative(self.0) }
    }
    fn negative_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_negative_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_arc_source_negative_on(self.0) }
    }
    fn negative_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_negative_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_arc_source_negative_off(self.0) }
    }
    fn set_use_normal_and_angle(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_use_normal_and_angle(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_arc_source_set_use_normal_and_angle(self.0, _arg) }
    }
    fn get_use_normal_and_angle(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_arc_source_get_use_normal_and_angle(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_arc_source_get_use_normal_and_angle(self.0) }
    }
    fn use_normal_and_angle_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_use_normal_and_angle_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_arc_source_use_normal_and_angle_on(self.0) }
    }
    fn use_normal_and_angle_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_use_normal_and_angle_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_arc_source_use_normal_and_angle_off(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_arc_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_arc_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arc_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arc_source_get_output_points_precision(self.0) }
    }
}
impl VtkArrowSource for vtkArrowSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_arrow_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_arrow_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_arrow_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_arrow_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_arrow_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_arrow_source_new_instance(self.0) }
    }
    fn set_tip_length(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_set_tip_length(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_arrow_source_set_tip_length(self.0, _arg) }
    }
    fn get_tip_length_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_length_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_tip_length_min_value(self.0) }
    }
    fn get_tip_length_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_length_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_tip_length_max_value(self.0) }
    }
    fn get_tip_length(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_length(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_tip_length(self.0) }
    }
    fn set_tip_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_set_tip_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_arrow_source_set_tip_radius(self.0, _arg) }
    }
    fn get_tip_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_tip_radius_min_value(self.0) }
    }
    fn get_tip_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_tip_radius_max_value(self.0) }
    }
    fn get_tip_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_tip_radius(self.0) }
    }
    fn set_tip_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_set_tip_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_arrow_source_set_tip_resolution(self.0, _arg) }
    }
    fn get_tip_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arrow_source_get_tip_resolution_min_value(self.0) }
    }
    fn get_tip_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arrow_source_get_tip_resolution_max_value(self.0) }
    }
    fn get_tip_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arrow_source_get_tip_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arrow_source_get_tip_resolution(self.0) }
    }
    fn set_shaft_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_set_shaft_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_arrow_source_set_shaft_radius(self.0, _arg) }
    }
    fn get_shaft_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_shaft_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_shaft_radius_min_value(self.0) }
    }
    fn get_shaft_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_shaft_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_shaft_radius_max_value(self.0) }
    }
    fn get_shaft_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_arrow_source_get_shaft_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_arrow_source_get_shaft_radius(self.0) }
    }
    fn set_shaft_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_set_shaft_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_arrow_source_set_shaft_resolution(self.0, _arg) }
    }
    fn get_shaft_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arrow_source_get_shaft_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arrow_source_get_shaft_resolution_min_value(self.0) }
    }
    fn get_shaft_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arrow_source_get_shaft_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arrow_source_get_shaft_resolution_max_value(self.0) }
    }
    fn get_shaft_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_arrow_source_get_shaft_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_arrow_source_get_shaft_resolution(self.0) }
    }
    fn invert_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_invert_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_arrow_source_invert_on(self.0) }
    }
    fn invert_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_invert_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_arrow_source_invert_off(self.0) }
    }
    fn set_invert(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_set_invert(sself: *mut core::ffi::c_void, _arg: bool);
        }
        unsafe { vtk_arrow_source_set_invert(self.0, _arg) }
    }
    fn get_invert(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_arrow_source_get_invert(sself: *mut core::ffi::c_void) -> bool;
        }
        unsafe { vtk_arrow_source_get_invert(self.0) }
    }
    fn set_arrow_origin_to_default(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_set_arrow_origin_to_default(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_arrow_source_set_arrow_origin_to_default(self.0) }
    }
    fn set_arrow_origin_to_center(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_arrow_source_set_arrow_origin_to_center(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_arrow_source_set_arrow_origin_to_center(self.0) }
    }
}
impl VtkCapsuleSource for vtkCapsuleSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_capsule_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_capsule_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_capsule_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_capsule_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_capsule_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_capsule_source_new(self.0) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_capsule_source_set_radius(self.0, _arg) }
    }
    fn get_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_capsule_source_get_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_capsule_source_get_radius_min_value(self.0) }
    }
    fn get_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_capsule_source_get_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_capsule_source_get_radius_max_value(self.0) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_capsule_source_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_capsule_source_get_radius(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_capsule_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_cylinder_length(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_set_cylinder_length(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_capsule_source_set_cylinder_length(self.0, _arg) }
    }
    fn get_cylinder_length_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_capsule_source_get_cylinder_length_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_capsule_source_get_cylinder_length_min_value(self.0) }
    }
    fn get_cylinder_length_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_capsule_source_get_cylinder_length_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_capsule_source_get_cylinder_length_max_value(self.0) }
    }
    fn get_cylinder_length(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_capsule_source_get_cylinder_length(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_capsule_source_get_cylinder_length(self.0) }
    }
    fn set_theta_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_set_theta_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_capsule_source_set_theta_resolution(self.0, _arg) }
    }
    fn get_theta_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_capsule_source_get_theta_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_capsule_source_get_theta_resolution_min_value(self.0) }
    }
    fn get_theta_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_capsule_source_get_theta_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_capsule_source_get_theta_resolution_max_value(self.0) }
    }
    fn get_theta_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_capsule_source_get_theta_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_capsule_source_get_theta_resolution(self.0) }
    }
    fn set_phi_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_set_phi_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_capsule_source_set_phi_resolution(self.0, _arg) }
    }
    fn get_phi_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_capsule_source_get_phi_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_capsule_source_get_phi_resolution_min_value(self.0) }
    }
    fn get_phi_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_capsule_source_get_phi_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_capsule_source_get_phi_resolution_max_value(self.0) }
    }
    fn get_phi_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_capsule_source_get_phi_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_capsule_source_get_phi_resolution(self.0) }
    }
    fn set_lat_long_tessellation(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_set_lat_long_tessellation(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_capsule_source_set_lat_long_tessellation(self.0, _arg) }
    }
    fn get_lat_long_tessellation(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_capsule_source_get_lat_long_tessellation(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_capsule_source_get_lat_long_tessellation(self.0) }
    }
    fn lat_long_tessellation_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_lat_long_tessellation_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_capsule_source_lat_long_tessellation_on(self.0) }
    }
    fn lat_long_tessellation_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_lat_long_tessellation_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_capsule_source_lat_long_tessellation_off(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_capsule_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_capsule_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_capsule_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_capsule_source_get_output_points_precision(self.0) }
    }
}
impl VtkCellTypeSource for vtkCellTypeSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cell_type_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cell_type_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cell_type_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cell_type_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cell_type_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cell_type_source_new_instance(self.0) }
    }
    fn set_cell_type(&mut self, cellType: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cell_type_source_set_cell_type(
                sself: *mut core::ffi::c_void,
                cellType: core::ffi::c_int,
            );
        }
        unsafe { vtk_cell_type_source_set_cell_type(self.0, cellType) }
    }
    fn get_cell_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_cell_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_cell_type(self.0) }
    }
    fn set_cell_order(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cell_type_source_set_cell_order(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cell_type_source_set_cell_order(self.0, _arg) }
    }
    fn get_cell_order(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_cell_order(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_cell_order(self.0) }
    }
    fn set_complete_quadratic_simplicial_elements(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_cell_type_source_set_complete_quadratic_simplicial_elements(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe {
            vtk_cell_type_source_set_complete_quadratic_simplicial_elements(self.0, _arg)
        }
    }
    fn get_complete_quadratic_simplicial_elements(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_complete_quadratic_simplicial_elements(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe {
            vtk_cell_type_source_get_complete_quadratic_simplicial_elements(self.0)
        }
    }
    fn complete_quadratic_simplicial_elements_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_cell_type_source_complete_quadratic_simplicial_elements_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_cell_type_source_complete_quadratic_simplicial_elements_on(self.0) }
    }
    fn complete_quadratic_simplicial_elements_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_cell_type_source_complete_quadratic_simplicial_elements_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtk_cell_type_source_complete_quadratic_simplicial_elements_off(self.0)
        }
    }
    fn set_polynomial_field_order(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cell_type_source_set_polynomial_field_order(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cell_type_source_set_polynomial_field_order(self.0, _arg) }
    }
    fn get_polynomial_field_order_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_polynomial_field_order_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_polynomial_field_order_min_value(self.0) }
    }
    fn get_polynomial_field_order_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_polynomial_field_order_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_polynomial_field_order_max_value(self.0) }
    }
    fn get_polynomial_field_order(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_polynomial_field_order(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_polynomial_field_order(self.0) }
    }
    fn get_cell_dimension(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_cell_dimension(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_cell_dimension(self.0) }
    }
    fn set_output_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cell_type_source_set_output_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cell_type_source_set_output_precision(self.0, _arg) }
    }
    fn get_output_precision_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_output_precision_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_output_precision_min_value(self.0) }
    }
    fn get_output_precision_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_output_precision_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_output_precision_max_value(self.0) }
    }
    fn get_output_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cell_type_source_get_output_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cell_type_source_get_output_precision(self.0) }
    }
}
impl VtkConeSource for vtkConeSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cone_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cone_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cone_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cone_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cone_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cone_source_new(self.0) }
    }
    fn set_height(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_set_height(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_cone_source_set_height(self.0, _arg) }
    }
    fn get_height_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cone_source_get_height_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cone_source_get_height_min_value(self.0) }
    }
    fn get_height_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cone_source_get_height_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cone_source_get_height_max_value(self.0) }
    }
    fn get_height(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cone_source_get_height(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cone_source_get_height(self.0) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_cone_source_set_radius(self.0, _arg) }
    }
    fn get_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cone_source_get_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cone_source_get_radius_min_value(self.0) }
    }
    fn get_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cone_source_get_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cone_source_get_radius_max_value(self.0) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cone_source_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cone_source_get_radius(self.0) }
    }
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_set_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cone_source_set_resolution(self.0, _arg) }
    }
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cone_source_get_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cone_source_get_resolution_min_value(self.0) }
    }
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cone_source_get_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cone_source_get_resolution_max_value(self.0) }
    }
    fn get_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cone_source_get_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cone_source_get_resolution(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_cone_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_direction(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_set_direction(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_cone_source_set_direction(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_angle(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_set_angle(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_cone_source_set_angle(self.0, angle) }
    }
    fn get_angle(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cone_source_get_angle(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cone_source_get_angle(self.0) }
    }
    fn set_capping(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_set_capping(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cone_source_set_capping(self.0, _arg) }
    }
    fn get_capping(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cone_source_get_capping(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cone_source_get_capping(self.0) }
    }
    fn capping_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_capping_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_cone_source_capping_on(self.0) }
    }
    fn capping_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_capping_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_cone_source_capping_off(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cone_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cone_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cone_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cone_source_get_output_points_precision(self.0) }
    }
}
impl VtkCubeSource for vtkCubeSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cube_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cube_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cube_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cube_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cube_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cube_source_new_instance(self.0) }
    }
    fn set_x_length(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_cube_source_set_x_length(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_cube_source_set_x_length(self.0, _arg) }
    }
    fn get_x_length_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_x_length_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_x_length_min_value(self.0) }
    }
    fn get_x_length_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_x_length_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_x_length_max_value(self.0) }
    }
    fn get_x_length(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_x_length(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_x_length(self.0) }
    }
    fn set_y_length(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_cube_source_set_y_length(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_cube_source_set_y_length(self.0, _arg) }
    }
    fn get_y_length_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_y_length_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_y_length_min_value(self.0) }
    }
    fn get_y_length_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_y_length_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_y_length_max_value(self.0) }
    }
    fn get_y_length(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_y_length(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_y_length(self.0) }
    }
    fn set_z_length(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_cube_source_set_z_length(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_cube_source_set_z_length(self.0, _arg) }
    }
    fn get_z_length_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_z_length_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_z_length_min_value(self.0) }
    }
    fn get_z_length_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_z_length_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_z_length_max_value(self.0) }
    }
    fn get_z_length(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cube_source_get_z_length(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cube_source_get_z_length(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_cube_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_cube_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_bounds(
        &mut self,
        xMin: core::ffi::c_double,
        xMax: core::ffi::c_double,
        yMin: core::ffi::c_double,
        yMax: core::ffi::c_double,
        zMin: core::ffi::c_double,
        zMax: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_cube_source_set_bounds(
                sself: *mut core::ffi::c_void,
                xMin: core::ffi::c_double,
                xMax: core::ffi::c_double,
                yMin: core::ffi::c_double,
                yMax: core::ffi::c_double,
                zMin: core::ffi::c_double,
                zMax: core::ffi::c_double,
            );
        }
        unsafe { vtk_cube_source_set_bounds(self.0, xMin, xMax, yMin, yMax, zMin, zMax) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cube_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cube_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cube_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cube_source_get_output_points_precision(self.0) }
    }
}
impl VtkCylinderSource for vtkCylinderSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cylinder_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cylinder_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cylinder_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cylinder_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cylinder_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cylinder_source_new_instance(self.0) }
    }
    fn set_height(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_cylinder_source_set_height(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_cylinder_source_set_height(self.0, _arg) }
    }
    fn get_height_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_height_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cylinder_source_get_height_min_value(self.0) }
    }
    fn get_height_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_height_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cylinder_source_get_height_max_value(self.0) }
    }
    fn get_height(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_height(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cylinder_source_get_height(self.0) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_cylinder_source_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_cylinder_source_set_radius(self.0, _arg) }
    }
    fn get_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cylinder_source_get_radius_min_value(self.0) }
    }
    fn get_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cylinder_source_get_radius_max_value(self.0) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_cylinder_source_get_radius(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_cylinder_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_cylinder_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cylinder_source_set_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cylinder_source_set_resolution(self.0, _arg) }
    }
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cylinder_source_get_resolution_min_value(self.0) }
    }
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cylinder_source_get_resolution_max_value(self.0) }
    }
    fn get_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cylinder_source_get_resolution(self.0) }
    }
    fn set_capping(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cylinder_source_set_capping(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cylinder_source_set_capping(self.0, _arg) }
    }
    fn get_capping(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_capping(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cylinder_source_get_capping(self.0) }
    }
    fn capping_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_cylinder_source_capping_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_cylinder_source_capping_on(self.0) }
    }
    fn capping_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_cylinder_source_capping_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_cylinder_source_capping_off(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_cylinder_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_cylinder_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_cylinder_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_cylinder_source_get_output_points_precision(self.0) }
    }
}
impl VtkDiagonalMatrixSource for vtkDiagonalMatrixSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_diagonal_matrix_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_diagonal_matrix_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_diagonal_matrix_source_new_instance(self.0) }
    }
    fn get_array_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_get_array_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_diagonal_matrix_source_get_array_type(self.0) }
    }
    fn set_array_type(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_set_array_type(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_diagonal_matrix_source_set_array_type(self.0, _arg) }
    }
    fn get_extents(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_get_extents(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_diagonal_matrix_source_get_extents(self.0) }
    }
    fn set_extents(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_set_extents(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_diagonal_matrix_source_set_extents(self.0, _arg) }
    }
    fn get_diagonal(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_get_diagonal(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_diagonal_matrix_source_get_diagonal(self.0) }
    }
    fn set_diagonal(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_set_diagonal(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_diagonal_matrix_source_set_diagonal(self.0, _arg) }
    }
    fn get_super_diagonal(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_get_super_diagonal(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_diagonal_matrix_source_get_super_diagonal(self.0) }
    }
    fn set_super_diagonal(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_set_super_diagonal(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_diagonal_matrix_source_set_super_diagonal(self.0, _arg) }
    }
    fn get_sub_diagonal(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_get_sub_diagonal(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_diagonal_matrix_source_get_sub_diagonal(self.0) }
    }
    fn set_sub_diagonal(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_set_sub_diagonal(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_diagonal_matrix_source_set_sub_diagonal(self.0, _arg) }
    }
    fn set_row_label(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_set_row_label(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_diagonal_matrix_source_set_row_label(self.0, c__arg.as_ptr()) }
    }
    fn set_column_label(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_diagonal_matrix_source_set_column_label(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_diagonal_matrix_source_set_column_label(self.0, c__arg.as_ptr()) }
    }
}
impl VtkDiskSource for vtkDiskSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_disk_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_disk_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_disk_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_disk_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_disk_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_disk_source_new_instance(self.0) }
    }
    fn set_inner_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_disk_source_set_inner_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_disk_source_set_inner_radius(self.0, _arg) }
    }
    fn get_inner_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_disk_source_get_inner_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_disk_source_get_inner_radius_min_value(self.0) }
    }
    fn get_inner_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_disk_source_get_inner_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_disk_source_get_inner_radius_max_value(self.0) }
    }
    fn get_inner_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_disk_source_get_inner_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_disk_source_get_inner_radius(self.0) }
    }
    fn set_outer_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_disk_source_set_outer_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_disk_source_set_outer_radius(self.0, _arg) }
    }
    fn get_outer_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_disk_source_get_outer_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_disk_source_get_outer_radius_min_value(self.0) }
    }
    fn get_outer_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_disk_source_get_outer_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_disk_source_get_outer_radius_max_value(self.0) }
    }
    fn get_outer_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_disk_source_get_outer_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_disk_source_get_outer_radius(self.0) }
    }
    fn set_radial_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_disk_source_set_radial_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_disk_source_set_radial_resolution(self.0, _arg) }
    }
    fn get_radial_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_disk_source_get_radial_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_disk_source_get_radial_resolution_min_value(self.0) }
    }
    fn get_radial_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_disk_source_get_radial_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_disk_source_get_radial_resolution_max_value(self.0) }
    }
    fn get_radial_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_disk_source_get_radial_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_disk_source_get_radial_resolution(self.0) }
    }
    fn set_circumferential_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_disk_source_set_circumferential_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_disk_source_set_circumferential_resolution(self.0, _arg) }
    }
    fn get_circumferential_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_disk_source_get_circumferential_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_disk_source_get_circumferential_resolution_min_value(self.0) }
    }
    fn get_circumferential_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_disk_source_get_circumferential_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_disk_source_get_circumferential_resolution_max_value(self.0) }
    }
    fn get_circumferential_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_disk_source_get_circumferential_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_disk_source_get_circumferential_resolution(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_disk_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_disk_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_disk_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_disk_source_get_output_points_precision(self.0) }
    }
}
impl VtkEllipseArcSource for vtkEllipseArcSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_ellipse_arc_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_ellipse_arc_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_ellipse_arc_source_new_instance(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_ellipse_arc_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_normal(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_normal(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_ellipse_arc_source_set_normal(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_major_radius_vector(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_major_radius_vector(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_ellipse_arc_source_set_major_radius_vector(self.0, _arg1, _arg2, _arg3)
        }
    }
    fn set_start_angle(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_start_angle(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_ellipse_arc_source_set_start_angle(self.0, _arg) }
    }
    fn get_start_angle_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_start_angle_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_start_angle_min_value(self.0) }
    }
    fn get_start_angle_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_start_angle_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_start_angle_max_value(self.0) }
    }
    fn get_start_angle(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_start_angle(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_start_angle(self.0) }
    }
    fn set_segment_angle(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_segment_angle(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_ellipse_arc_source_set_segment_angle(self.0, _arg) }
    }
    fn get_segment_angle_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_segment_angle_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_segment_angle_min_value(self.0) }
    }
    fn get_segment_angle_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_segment_angle_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_segment_angle_max_value(self.0) }
    }
    fn get_segment_angle(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_segment_angle(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_segment_angle(self.0) }
    }
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_ellipse_arc_source_set_resolution(self.0, _arg) }
    }
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_ellipse_arc_source_get_resolution_min_value(self.0) }
    }
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_ellipse_arc_source_get_resolution_max_value(self.0) }
    }
    fn get_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_ellipse_arc_source_get_resolution(self.0) }
    }
    fn set_close(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_close(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_ellipse_arc_source_set_close(self.0, _arg) }
    }
    fn get_close(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_close(sself: *mut core::ffi::c_void) -> bool;
        }
        unsafe { vtk_ellipse_arc_source_get_close(self.0) }
    }
    fn close_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_close_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_ellipse_arc_source_close_on(self.0) }
    }
    fn close_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_close_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_ellipse_arc_source_close_off(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_ellipse_arc_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_ellipse_arc_source_get_output_points_precision(self.0) }
    }
    fn set_ratio(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_set_ratio(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_ellipse_arc_source_set_ratio(self.0, _arg) }
    }
    fn get_ratio_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_ratio_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_ratio_min_value(self.0) }
    }
    fn get_ratio_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_ratio_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_ratio_max_value(self.0) }
    }
    fn get_ratio(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_ellipse_arc_source_get_ratio(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_ellipse_arc_source_get_ratio(self.0) }
    }
}
impl VtkEllipticalButtonSource for vtkEllipticalButtonSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_elliptical_button_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_elliptical_button_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_elliptical_button_source_new(self.0) }
    }
    fn set_width(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_set_width(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_elliptical_button_source_set_width(self.0, _arg) }
    }
    fn get_width_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_width_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_width_min_value(self.0) }
    }
    fn get_width_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_width_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_width_max_value(self.0) }
    }
    fn get_width(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_width(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_width(self.0) }
    }
    fn set_height(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_set_height(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_elliptical_button_source_set_height(self.0, _arg) }
    }
    fn get_height_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_height_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_height_min_value(self.0) }
    }
    fn get_height_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_height_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_height_max_value(self.0) }
    }
    fn get_height(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_height(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_height(self.0) }
    }
    fn set_depth(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_set_depth(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_elliptical_button_source_set_depth(self.0, _arg) }
    }
    fn get_depth_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_depth_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_depth_min_value(self.0) }
    }
    fn get_depth_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_depth_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_depth_max_value(self.0) }
    }
    fn get_depth(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_depth(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_depth(self.0) }
    }
    fn set_circumferential_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_set_circumferential_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_elliptical_button_source_set_circumferential_resolution(self.0, _arg)
        }
    }
    fn get_circumferential_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_circumferential_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_elliptical_button_source_get_circumferential_resolution_min_value(self.0)
        }
    }
    fn get_circumferential_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_circumferential_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_elliptical_button_source_get_circumferential_resolution_max_value(self.0)
        }
    }
    fn get_circumferential_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_circumferential_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_elliptical_button_source_get_circumferential_resolution(self.0) }
    }
    fn set_texture_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_set_texture_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_elliptical_button_source_set_texture_resolution(self.0, _arg) }
    }
    fn get_texture_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_texture_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_elliptical_button_source_get_texture_resolution_min_value(self.0) }
    }
    fn get_texture_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_texture_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_elliptical_button_source_get_texture_resolution_max_value(self.0) }
    }
    fn get_texture_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_texture_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_elliptical_button_source_get_texture_resolution(self.0) }
    }
    fn set_shoulder_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_set_shoulder_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_elliptical_button_source_set_shoulder_resolution(self.0, _arg) }
    }
    fn get_shoulder_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_shoulder_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_elliptical_button_source_get_shoulder_resolution_min_value(self.0) }
    }
    fn get_shoulder_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_shoulder_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_elliptical_button_source_get_shoulder_resolution_max_value(self.0) }
    }
    fn get_shoulder_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_shoulder_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_elliptical_button_source_get_shoulder_resolution(self.0) }
    }
    fn set_radial_ratio(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_set_radial_ratio(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_elliptical_button_source_set_radial_ratio(self.0, _arg) }
    }
    fn get_radial_ratio_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_radial_ratio_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_radial_ratio_min_value(self.0) }
    }
    fn get_radial_ratio_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_radial_ratio_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_radial_ratio_max_value(self.0) }
    }
    fn get_radial_ratio(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_radial_ratio(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_elliptical_button_source_get_radial_ratio(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_elliptical_button_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_elliptical_button_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_elliptical_button_source_get_output_points_precision(self.0) }
    }
}
impl VtkFrustumSource for vtkFrustumSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_frustum_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_frustum_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_frustum_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_frustum_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_frustum_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_frustum_source_new_instance(self.0) }
    }
    fn get_planes(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_frustum_source_get_planes(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_frustum_source_get_planes(self.0) }
    }
    fn set_planes(&mut self, planes: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_frustum_source_set_planes(
                sself: *mut core::ffi::c_void,
                planes: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_frustum_source_set_planes(self.0, planes) }
    }
    fn get_show_lines(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_frustum_source_get_show_lines(sself: *mut core::ffi::c_void) -> bool;
        }
        unsafe { vtk_frustum_source_get_show_lines(self.0) }
    }
    fn set_show_lines(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_frustum_source_set_show_lines(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_frustum_source_set_show_lines(self.0, _arg) }
    }
    fn show_lines_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_frustum_source_show_lines_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_frustum_source_show_lines_on(self.0) }
    }
    fn show_lines_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_frustum_source_show_lines_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_frustum_source_show_lines_off(self.0) }
    }
    fn get_lines_length(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_frustum_source_get_lines_length(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_frustum_source_get_lines_length(self.0) }
    }
    fn set_lines_length(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_frustum_source_set_lines_length(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_frustum_source_set_lines_length(self.0, _arg) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_frustum_source_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_frustum_source_get_m_time(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_frustum_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_frustum_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_frustum_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_frustum_source_get_output_points_precision(self.0) }
    }
}
impl VtkGlyphSource2D for vtkGlyphSource2D {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_glyph_source_2_d_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_glyph_source_2_d_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_glyph_source_2_d_new(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_scale(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_scale(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_scale(self.0, _arg) }
    }
    fn get_scale_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_scale_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_glyph_source_2_d_get_scale_min_value(self.0) }
    }
    fn get_scale_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_scale_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_glyph_source_2_d_get_scale_max_value(self.0) }
    }
    fn get_scale(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_scale(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_glyph_source_2_d_get_scale(self.0) }
    }
    fn set_scale_2(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_scale_2(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_scale_2(self.0, _arg) }
    }
    fn get_scale_2_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_scale_2_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_glyph_source_2_d_get_scale_2_min_value(self.0) }
    }
    fn get_scale_2_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_scale_2_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_glyph_source_2_d_get_scale_2_max_value(self.0) }
    }
    fn get_scale_2(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_scale_2(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_glyph_source_2_d_get_scale_2(self.0) }
    }
    fn set_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_color(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_color(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_filled(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_filled(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_filled(self.0, _arg) }
    }
    fn get_filled(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_filled(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_filled(self.0) }
    }
    fn filled_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_filled_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_glyph_source_2_d_filled_on(self.0) }
    }
    fn filled_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_filled_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_glyph_source_2_d_filled_off(self.0) }
    }
    fn set_dash(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_dash(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_dash(self.0, _arg) }
    }
    fn get_dash(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_dash(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_dash(self.0) }
    }
    fn dash_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_dash_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_glyph_source_2_d_dash_on(self.0) }
    }
    fn dash_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_dash_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_glyph_source_2_d_dash_off(self.0) }
    }
    fn set_cross(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_cross(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_cross(self.0, _arg) }
    }
    fn get_cross(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_cross(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_cross(self.0) }
    }
    fn cross_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_cross_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_glyph_source_2_d_cross_on(self.0) }
    }
    fn cross_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_cross_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_glyph_source_2_d_cross_off(self.0) }
    }
    fn set_rotation_angle(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_rotation_angle(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_rotation_angle(self.0, _arg) }
    }
    fn get_rotation_angle(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_rotation_angle(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_glyph_source_2_d_get_rotation_angle(self.0) }
    }
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_resolution(self.0, _arg) }
    }
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_resolution_min_value(self.0) }
    }
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_resolution_max_value(self.0) }
    }
    fn get_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_resolution(self.0) }
    }
    fn set_glyph_type(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type(self.0, _arg) }
    }
    fn get_glyph_type_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_glyph_type_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_glyph_type_min_value(self.0) }
    }
    fn get_glyph_type_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_glyph_type_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_glyph_type_max_value(self.0) }
    }
    fn get_glyph_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_glyph_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_glyph_type(self.0) }
    }
    fn set_glyph_type_to_none(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_none(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_none(self.0) }
    }
    fn set_glyph_type_to_vertex(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_vertex(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_vertex(self.0) }
    }
    fn set_glyph_type_to_dash(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_dash(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_dash(self.0) }
    }
    fn set_glyph_type_to_cross(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_cross(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_cross(self.0) }
    }
    fn set_glyph_type_to_thick_cross(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_thick_cross(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_thick_cross(self.0) }
    }
    fn set_glyph_type_to_triangle(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_triangle(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_triangle(self.0) }
    }
    fn set_glyph_type_to_square(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_square(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_square(self.0) }
    }
    fn set_glyph_type_to_circle(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_circle(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_circle(self.0) }
    }
    fn set_glyph_type_to_diamond(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_diamond(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_diamond(self.0) }
    }
    fn set_glyph_type_to_arrow(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_arrow(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_arrow(self.0) }
    }
    fn set_glyph_type_to_thick_arrow(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_thick_arrow(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_thick_arrow(self.0) }
    }
    fn set_glyph_type_to_hooked_arrow(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_hooked_arrow(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_hooked_arrow(self.0) }
    }
    fn set_glyph_type_to_edge_arrow(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_glyph_type_to_edge_arrow(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_glyph_type_to_edge_arrow(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_glyph_source_2_d_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_glyph_source_2_d_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_glyph_source_2_d_get_output_points_precision(self.0) }
    }
}
impl VtkGraphToPolyData for vtkGraphToPolyData {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_graph_to_poly_data_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_graph_to_poly_data_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_graph_to_poly_data_new_instance(self.0) }
    }
    fn set_edge_glyph_output(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_set_edge_glyph_output(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_graph_to_poly_data_set_edge_glyph_output(self.0, _arg) }
    }
    fn get_edge_glyph_output(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_get_edge_glyph_output(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_graph_to_poly_data_get_edge_glyph_output(self.0) }
    }
    fn edge_glyph_output_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_edge_glyph_output_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_graph_to_poly_data_edge_glyph_output_on(self.0) }
    }
    fn edge_glyph_output_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_edge_glyph_output_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_graph_to_poly_data_edge_glyph_output_off(self.0) }
    }
    fn set_edge_glyph_position(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_set_edge_glyph_position(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_graph_to_poly_data_set_edge_glyph_position(self.0, _arg) }
    }
    fn get_edge_glyph_position(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_graph_to_poly_data_get_edge_glyph_position(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_graph_to_poly_data_get_edge_glyph_position(self.0) }
    }
}
impl VtkHyperTreeGridSource for vtkHyperTreeGridSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hyper_tree_grid_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hyper_tree_grid_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hyper_tree_grid_source_new(self.0) }
    }
    fn get_maximum_level(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_maximum_level(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_hyper_tree_grid_source_get_maximum_level(self.0) }
    }
    fn set_maximum_level(&mut self, levels: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_maximum_level(
                sself: *mut core::ffi::c_void,
                levels: core::ffi::c_uint,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_maximum_level(self.0, levels) }
    }
    fn get_max_depth(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_max_depth(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_hyper_tree_grid_source_get_max_depth(self.0) }
    }
    fn set_max_depth(&mut self, levels: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_max_depth(
                sself: *mut core::ffi::c_void,
                levels: core::ffi::c_uint,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_max_depth(self.0, levels) }
    }
    fn set_origin(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_origin(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_origin(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_grid_scale(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_grid_scale(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_grid_scale(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_transposed_root_indexing(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_transposed_root_indexing(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_transposed_root_indexing(self.0, _arg) }
    }
    fn get_transposed_root_indexing(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_transposed_root_indexing(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_hyper_tree_grid_source_get_transposed_root_indexing(self.0) }
    }
    fn set_indexing_mode_to_kji(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_indexing_mode_to_kji(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_indexing_mode_to_kji(self.0) }
    }
    fn set_indexing_mode_to_ijk(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_indexing_mode_to_ijk(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_indexing_mode_to_ijk(self.0) }
    }
    fn get_orientation(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_orientation(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_hyper_tree_grid_source_get_orientation(self.0) }
    }
    fn set_branch_factor(&mut self, _arg: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_branch_factor(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_uint,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_branch_factor(self.0, _arg) }
    }
    fn get_branch_factor_min_value(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_branch_factor_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_hyper_tree_grid_source_get_branch_factor_min_value(self.0) }
    }
    fn get_branch_factor_max_value(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_branch_factor_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_hyper_tree_grid_source_get_branch_factor_max_value(self.0) }
    }
    fn get_branch_factor(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_branch_factor(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_hyper_tree_grid_source_get_branch_factor(self.0) }
    }
    fn set_use_descriptor(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_use_descriptor(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_use_descriptor(self.0, _arg) }
    }
    fn get_use_descriptor(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_use_descriptor(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_hyper_tree_grid_source_get_use_descriptor(self.0) }
    }
    fn use_descriptor_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_use_descriptor_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_use_descriptor_on(self.0) }
    }
    fn use_descriptor_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_use_descriptor_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_use_descriptor_off(self.0) }
    }
    fn set_use_mask(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_use_mask(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_use_mask(self.0, _arg) }
    }
    fn get_use_mask(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_use_mask(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_hyper_tree_grid_source_get_use_mask(self.0) }
    }
    fn use_mask_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_use_mask_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_hyper_tree_grid_source_use_mask_on(self.0) }
    }
    fn use_mask_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_use_mask_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_hyper_tree_grid_source_use_mask_off(self.0) }
    }
    fn set_generate_interface_fields(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_generate_interface_fields(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_generate_interface_fields(self.0, _arg) }
    }
    fn get_generate_interface_fields(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_generate_interface_fields(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_hyper_tree_grid_source_get_generate_interface_fields(self.0) }
    }
    fn generate_interface_fields_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_generate_interface_fields_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_generate_interface_fields_on(self.0) }
    }
    fn generate_interface_fields_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_generate_interface_fields_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_generate_interface_fields_off(self.0) }
    }
    fn set_descriptor(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_descriptor(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_descriptor(self.0, c__arg.as_ptr()) }
    }
    fn set_mask(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_mask(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_mask(self.0, c__arg.as_ptr()) }
    }
    fn set_descriptor_bits(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_descriptor_bits(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_descriptor_bits(self.0, p0) }
    }
    fn get_descriptor_bits(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_descriptor_bits(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hyper_tree_grid_source_get_descriptor_bits(self.0) }
    }
    fn set_level_zero_material_index(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_level_zero_material_index(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_level_zero_material_index(self.0, p0) }
    }
    fn set_mask_bits(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_mask_bits(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_mask_bits(self.0, p0) }
    }
    fn get_mask_bits(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_mask_bits(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hyper_tree_grid_source_get_mask_bits(self.0) }
    }
    fn set_quadric(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_set_quadric(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_hyper_tree_grid_source_set_quadric(self.0, p0) }
    }
    fn get_quadric(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_quadric(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_hyper_tree_grid_source_get_quadric(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_hyper_tree_grid_source_get_m_time(self.0) }
    }
    fn convert_descriptor_string_to_bit_array(
        &mut self,
        p0: &str,
    ) -> *mut core::ffi::c_void {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_convert_descriptor_string_to_bit_array(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_hyper_tree_grid_source_convert_descriptor_string_to_bit_array(
                self.0,
                c_p0.as_ptr(),
            )
        }
    }
    fn convert_mask_string_to_bit_array(&mut self, p0: &str) -> *mut core::ffi::c_void {
        let c_p0 = std::ffi::CString::new(p0).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_hyper_tree_grid_source_convert_mask_string_to_bit_array(
                sself: *mut core::ffi::c_void,
                p0: *const core::ffi::c_char,
            ) -> *mut core::ffi::c_void;
        }
        unsafe {
            vtk_hyper_tree_grid_source_convert_mask_string_to_bit_array(
                self.0,
                c_p0.as_ptr(),
            )
        }
    }
}
impl VtkLineSource for vtkLineSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_line_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_line_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_line_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_line_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_line_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_line_source_new_instance(self.0) }
    }
    fn set_point_1(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_line_source_set_point_1(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_line_source_set_point_1(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_point_2(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_line_source_set_point_2(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_line_source_set_point_2(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_use_regular_refinement(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_line_source_set_use_regular_refinement(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe { vtk_line_source_set_use_regular_refinement(self.0, _arg) }
    }
    fn get_use_regular_refinement(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_line_source_get_use_regular_refinement(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_line_source_get_use_regular_refinement(self.0) }
    }
    fn use_regular_refinement_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_line_source_use_regular_refinement_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_line_source_use_regular_refinement_on(self.0) }
    }
    fn use_regular_refinement_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_line_source_use_regular_refinement_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_line_source_use_regular_refinement_off(self.0) }
    }
    fn set_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_line_source_set_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_line_source_set_resolution(self.0, _arg) }
    }
    fn get_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_line_source_get_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_line_source_get_resolution_min_value(self.0) }
    }
    fn get_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_line_source_get_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_line_source_get_resolution_max_value(self.0) }
    }
    fn get_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_line_source_get_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_line_source_get_resolution(self.0) }
    }
    fn set_number_of_refinement_ratios(&mut self, p0: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_line_source_set_number_of_refinement_ratios(
                sself: *mut core::ffi::c_void,
                p0: core::ffi::c_int,
            );
        }
        unsafe { vtk_line_source_set_number_of_refinement_ratios(self.0, p0) }
    }
    fn set_refinement_ratio(
        &mut self,
        index: core::ffi::c_int,
        value: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_line_source_set_refinement_ratio(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
                value: core::ffi::c_double,
            );
        }
        unsafe { vtk_line_source_set_refinement_ratio(self.0, index, value) }
    }
    fn get_number_of_refinement_ratios(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_line_source_get_number_of_refinement_ratios(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_line_source_get_number_of_refinement_ratios(self.0) }
    }
    fn get_refinement_ratio(&mut self, index: core::ffi::c_int) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_line_source_get_refinement_ratio(
                sself: *mut core::ffi::c_void,
                index: core::ffi::c_int,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_line_source_get_refinement_ratio(self.0, index) }
    }
    fn set_points(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_line_source_set_points(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_line_source_set_points(self.0, p0) }
    }
    fn get_points(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_line_source_get_points(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_line_source_get_points(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_line_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_line_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_line_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_line_source_get_output_points_precision(self.0) }
    }
}
impl VtkOutlineCornerFilter for vtkOutlineCornerFilter {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_corner_filter_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_corner_filter_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_corner_filter_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_corner_filter_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_corner_filter_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_corner_filter_new(self.0) }
    }
    fn set_corner_factor(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_outline_corner_filter_set_corner_factor(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_outline_corner_filter_set_corner_factor(self.0, _arg) }
    }
    fn get_corner_factor_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_outline_corner_filter_get_corner_factor_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_outline_corner_filter_get_corner_factor_min_value(self.0) }
    }
    fn get_corner_factor_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_outline_corner_filter_get_corner_factor_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_outline_corner_filter_get_corner_factor_max_value(self.0) }
    }
    fn get_corner_factor(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_outline_corner_filter_get_corner_factor(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_outline_corner_filter_get_corner_factor(self.0) }
    }
}
impl VtkOutlineCornerSource for vtkOutlineCornerSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_corner_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_corner_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_corner_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_corner_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_corner_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_corner_source_new(self.0) }
    }
    fn set_corner_factor(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_outline_corner_source_set_corner_factor(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_outline_corner_source_set_corner_factor(self.0, _arg) }
    }
    fn get_corner_factor_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_outline_corner_source_get_corner_factor_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_outline_corner_source_get_corner_factor_min_value(self.0) }
    }
    fn get_corner_factor_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_outline_corner_source_get_corner_factor_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_outline_corner_source_get_corner_factor_max_value(self.0) }
    }
    fn get_corner_factor(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_outline_corner_source_get_corner_factor(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_outline_corner_source_get_corner_factor(self.0) }
    }
}
impl VtkOutlineSource for vtkOutlineSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_outline_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_outline_source_new_instance(self.0) }
    }
    fn set_box_type(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_outline_source_set_box_type(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_outline_source_set_box_type(self.0, _arg) }
    }
    fn get_box_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_outline_source_get_box_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_outline_source_get_box_type(self.0) }
    }
    fn set_box_type_to_axis_aligned(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_outline_source_set_box_type_to_axis_aligned(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_outline_source_set_box_type_to_axis_aligned(self.0) }
    }
    fn set_box_type_to_oriented(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_outline_source_set_box_type_to_oriented(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_outline_source_set_box_type_to_oriented(self.0) }
    }
    fn set_bounds(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
        _arg5: core::ffi::c_double,
        _arg6: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_outline_source_set_bounds(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
                _arg4: core::ffi::c_double,
                _arg5: core::ffi::c_double,
                _arg6: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_outline_source_set_bounds(
                self.0,
                _arg1,
                _arg2,
                _arg3,
                _arg4,
                _arg5,
                _arg6,
            )
        }
    }
    fn set_generate_faces(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_outline_source_set_generate_faces(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_outline_source_set_generate_faces(self.0, _arg) }
    }
    fn generate_faces_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_outline_source_generate_faces_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_outline_source_generate_faces_on(self.0) }
    }
    fn generate_faces_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_outline_source_generate_faces_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_outline_source_generate_faces_off(self.0) }
    }
    fn get_generate_faces(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_outline_source_get_generate_faces(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_outline_source_get_generate_faces(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_outline_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_outline_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_outline_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_outline_source_get_output_points_precision(self.0) }
    }
}
impl VtkParametricFunctionSource for vtkParametricFunctionSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_function_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_function_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_function_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_function_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_function_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_function_source_new(self.0) }
    }
    fn set_parametric_function(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_parametric_function(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_parametric_function(self.0, p0) }
    }
    fn get_parametric_function(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_parametric_function(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_parametric_function_source_get_parametric_function(self.0) }
    }
    fn set_u_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_u_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_function_source_set_u_resolution(self.0, _arg) }
    }
    fn get_u_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_u_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_u_resolution_min_value(self.0) }
    }
    fn get_u_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_u_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_u_resolution_max_value(self.0) }
    }
    fn get_u_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_u_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_u_resolution(self.0) }
    }
    fn set_v_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_v_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_function_source_set_v_resolution(self.0, _arg) }
    }
    fn get_v_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_v_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_v_resolution_min_value(self.0) }
    }
    fn get_v_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_v_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_v_resolution_max_value(self.0) }
    }
    fn get_v_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_v_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_v_resolution(self.0) }
    }
    fn set_w_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_w_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_function_source_set_w_resolution(self.0, _arg) }
    }
    fn get_w_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_w_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_w_resolution_min_value(self.0) }
    }
    fn get_w_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_w_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_w_resolution_max_value(self.0) }
    }
    fn get_w_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_w_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_w_resolution(self.0) }
    }
    fn generate_texture_coordinates_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_generate_texture_coordinates_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_generate_texture_coordinates_on(self.0) }
    }
    fn generate_texture_coordinates_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_generate_texture_coordinates_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtk_parametric_function_source_generate_texture_coordinates_off(self.0)
        }
    }
    fn set_generate_texture_coordinates(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_generate_texture_coordinates(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_parametric_function_source_set_generate_texture_coordinates(self.0, _arg)
        }
    }
    fn get_generate_texture_coordinates_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_generate_texture_coordinates_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_parametric_function_source_get_generate_texture_coordinates_min_value(
                self.0,
            )
        }
    }
    fn get_generate_texture_coordinates_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_generate_texture_coordinates_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_parametric_function_source_get_generate_texture_coordinates_max_value(
                self.0,
            )
        }
    }
    fn get_generate_texture_coordinates(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_generate_texture_coordinates(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_parametric_function_source_get_generate_texture_coordinates(self.0)
        }
    }
    fn generate_normals_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_generate_normals_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_generate_normals_on(self.0) }
    }
    fn generate_normals_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_generate_normals_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_generate_normals_off(self.0) }
    }
    fn set_generate_normals(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_generate_normals(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_function_source_set_generate_normals(self.0, _arg) }
    }
    fn get_generate_normals_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_generate_normals_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_generate_normals_min_value(self.0) }
    }
    fn get_generate_normals_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_generate_normals_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_generate_normals_max_value(self.0) }
    }
    fn get_generate_normals(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_generate_normals(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_generate_normals(self.0) }
    }
    fn set_scalar_mode(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode(self.0, _arg) }
    }
    fn get_scalar_mode_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_scalar_mode_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_scalar_mode_min_value(self.0) }
    }
    fn get_scalar_mode_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_scalar_mode_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_scalar_mode_max_value(self.0) }
    }
    fn get_scalar_mode(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_scalar_mode(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_scalar_mode(self.0) }
    }
    fn set_scalar_mode_to_none(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_none(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_none(self.0) }
    }
    fn set_scalar_mode_to_u(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_u(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_u(self.0) }
    }
    fn set_scalar_mode_to_v(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_v(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_v(self.0) }
    }
    fn set_scalar_mode_to_u_0(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_u_0(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_u_0(self.0) }
    }
    fn set_scalar_mode_to_v_0(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_v_0(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_v_0(self.0) }
    }
    fn set_scalar_mode_to_u_0_v_0(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_u_0_v_0(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_u_0_v_0(self.0) }
    }
    fn set_scalar_mode_to_modulus(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_modulus(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_modulus(self.0) }
    }
    fn set_scalar_mode_to_phase(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_phase(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_phase(self.0) }
    }
    fn set_scalar_mode_to_quadrant(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_quadrant(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_quadrant(self.0) }
    }
    fn set_scalar_mode_to_x(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_x(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_x(self.0) }
    }
    fn set_scalar_mode_to_y(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_y(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_y(self.0) }
    }
    fn set_scalar_mode_to_z(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_z(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_z(self.0) }
    }
    fn set_scalar_mode_to_distance(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_distance(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_parametric_function_source_set_scalar_mode_to_distance(self.0) }
    }
    fn set_scalar_mode_to_function_defined(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_scalar_mode_to_function_defined(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtk_parametric_function_source_set_scalar_mode_to_function_defined(self.0)
        }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_parametric_function_source_get_m_time(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_parametric_function_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_parametric_function_source_set_output_points_precision(self.0, _arg)
        }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_parametric_function_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_parametric_function_source_get_output_points_precision(self.0) }
    }
}
impl VtkPartitionedDataSetCollectionSource for vtkPartitionedDataSetCollectionSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_collection_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_partitioned_data_set_collection_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_collection_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_partitioned_data_set_collection_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_collection_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_partitioned_data_set_collection_source_new_instance(self.0) }
    }
    fn set_number_of_shapes(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_collection_source_set_number_of_shapes(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_partitioned_data_set_collection_source_set_number_of_shapes(self.0, _arg)
        }
    }
    fn get_number_of_shapes_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_collection_source_get_number_of_shapes_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_partitioned_data_set_collection_source_get_number_of_shapes_min_value(
                self.0,
            )
        }
    }
    fn get_number_of_shapes_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_collection_source_get_number_of_shapes_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_partitioned_data_set_collection_source_get_number_of_shapes_max_value(
                self.0,
            )
        }
    }
    fn get_number_of_shapes(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_collection_source_get_number_of_shapes(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_partitioned_data_set_collection_source_get_number_of_shapes(self.0)
        }
    }
}
impl VtkPartitionedDataSetSource for vtkPartitionedDataSetSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_partitioned_data_set_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_partitioned_data_set_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_partitioned_data_set_source_new_instance(self.0) }
    }
    fn enable_rank(&mut self, rank: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_enable_rank(
                sself: *mut core::ffi::c_void,
                rank: core::ffi::c_int,
            );
        }
        unsafe { vtk_partitioned_data_set_source_enable_rank(self.0, rank) }
    }
    fn enable_all_ranks(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_enable_all_ranks(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_partitioned_data_set_source_enable_all_ranks(self.0) }
    }
    fn disable_rank(&mut self, rank: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_disable_rank(
                sself: *mut core::ffi::c_void,
                rank: core::ffi::c_int,
            );
        }
        unsafe { vtk_partitioned_data_set_source_disable_rank(self.0, rank) }
    }
    fn disable_all_ranks(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_disable_all_ranks(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_partitioned_data_set_source_disable_all_ranks(self.0) }
    }
    fn is_enabled_rank(&mut self, rank: core::ffi::c_int) -> bool {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_is_enabled_rank(
                sself: *mut core::ffi::c_void,
                rank: core::ffi::c_int,
            ) -> bool;
        }
        unsafe { vtk_partitioned_data_set_source_is_enabled_rank(self.0, rank) }
    }
    fn set_number_of_partitions(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_set_number_of_partitions(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_partitioned_data_set_source_set_number_of_partitions(self.0, _arg) }
    }
    fn get_number_of_partitions_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_get_number_of_partitions_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_partitioned_data_set_source_get_number_of_partitions_min_value(self.0)
        }
    }
    fn get_number_of_partitions_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_get_number_of_partitions_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_partitioned_data_set_source_get_number_of_partitions_max_value(self.0)
        }
    }
    fn get_number_of_partitions(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_get_number_of_partitions(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_partitioned_data_set_source_get_number_of_partitions(self.0) }
    }
    fn set_parametric_function(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_set_parametric_function(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_partitioned_data_set_source_set_parametric_function(self.0, p0) }
    }
    fn get_parametric_function(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_partitioned_data_set_source_get_parametric_function(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_partitioned_data_set_source_get_parametric_function(self.0) }
    }
}
impl VtkPlaneSource for vtkPlaneSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_plane_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_plane_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_plane_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_plane_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_plane_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_plane_source_new(self.0) }
    }
    fn set_x_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_x_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_plane_source_set_x_resolution(self.0, _arg) }
    }
    fn get_x_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_plane_source_get_x_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_plane_source_get_x_resolution(self.0) }
    }
    fn set_y_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_y_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_plane_source_set_y_resolution(self.0, _arg) }
    }
    fn get_y_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_plane_source_get_y_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_plane_source_get_y_resolution(self.0) }
    }
    fn set_resolution(&mut self, xR: core::ffi::c_int, yR: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_resolution(
                sself: *mut core::ffi::c_void,
                xR: core::ffi::c_int,
                yR: core::ffi::c_int,
            );
        }
        unsafe { vtk_plane_source_set_resolution(self.0, xR, yR) }
    }
    fn get_resolution(
        &mut self,
        xR: &mut core::ffi::c_int,
        yR: &mut core::ffi::c_int,
    ) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_get_resolution(
                sself: *mut core::ffi::c_void,
                xR: &mut core::ffi::c_int,
                yR: &mut core::ffi::c_int,
            );
        }
        unsafe { vtk_plane_source_get_resolution(self.0, xR, yR) }
    }
    fn set_origin(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_origin(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_plane_source_set_origin(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_point_1(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_point_1(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_plane_source_set_point_1(self.0, x, y, z) }
    }
    fn set_point_2(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_point_2(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_plane_source_set_point_2(self.0, x, y, z) }
    }
    fn set_center(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_center(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_plane_source_set_center(self.0, x, y, z) }
    }
    fn set_normal(
        &mut self,
        nx: core::ffi::c_double,
        ny: core::ffi::c_double,
        nz: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_normal(
                sself: *mut core::ffi::c_void,
                nx: core::ffi::c_double,
                ny: core::ffi::c_double,
                nz: core::ffi::c_double,
            );
        }
        unsafe { vtk_plane_source_set_normal(self.0, nx, ny, nz) }
    }
    fn push(&mut self, distance: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_push(
                sself: *mut core::ffi::c_void,
                distance: core::ffi::c_double,
            );
        }
        unsafe { vtk_plane_source_push(self.0, distance) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_plane_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_plane_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_plane_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_plane_source_get_output_points_precision(self.0) }
    }
}
impl VtkPlatonicSolidSource for vtkPlatonicSolidSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_platonic_solid_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_platonic_solid_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_platonic_solid_source_new_instance(self.0) }
    }
    fn set_solid_type(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_set_solid_type(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_platonic_solid_source_set_solid_type(self.0, _arg) }
    }
    fn get_solid_type_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_get_solid_type_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_platonic_solid_source_get_solid_type_min_value(self.0) }
    }
    fn get_solid_type_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_get_solid_type_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_platonic_solid_source_get_solid_type_max_value(self.0) }
    }
    fn get_solid_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_get_solid_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_platonic_solid_source_get_solid_type(self.0) }
    }
    fn set_solid_type_to_tetrahedron(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_set_solid_type_to_tetrahedron(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_platonic_solid_source_set_solid_type_to_tetrahedron(self.0) }
    }
    fn set_solid_type_to_cube(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_set_solid_type_to_cube(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_platonic_solid_source_set_solid_type_to_cube(self.0) }
    }
    fn set_solid_type_to_octahedron(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_set_solid_type_to_octahedron(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_platonic_solid_source_set_solid_type_to_octahedron(self.0) }
    }
    fn set_solid_type_to_icosahedron(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_set_solid_type_to_icosahedron(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_platonic_solid_source_set_solid_type_to_icosahedron(self.0) }
    }
    fn set_solid_type_to_dodecahedron(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_set_solid_type_to_dodecahedron(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_platonic_solid_source_set_solid_type_to_dodecahedron(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_platonic_solid_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_platonic_solid_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_platonic_solid_source_get_output_points_precision(self.0) }
    }
}
impl VtkPointHandleSource for vtkPointHandleSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_handle_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_handle_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_handle_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_handle_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_handle_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_handle_source_new_instance(self.0) }
    }
    fn set_position(
        &mut self,
        xPos: core::ffi::c_double,
        yPos: core::ffi::c_double,
        zPos: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_point_handle_source_set_position(
                sself: *mut core::ffi::c_void,
                xPos: core::ffi::c_double,
                yPos: core::ffi::c_double,
                zPos: core::ffi::c_double,
            );
        }
        unsafe { vtk_point_handle_source_set_position(self.0, xPos, yPos, zPos) }
    }
    fn set_direction(
        &mut self,
        xDir: core::ffi::c_double,
        yDir: core::ffi::c_double,
        zDir: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_point_handle_source_set_direction(
                sself: *mut core::ffi::c_void,
                xDir: core::ffi::c_double,
                yDir: core::ffi::c_double,
                zDir: core::ffi::c_double,
            );
        }
        unsafe { vtk_point_handle_source_set_direction(self.0, xDir, yDir, zDir) }
    }
}
impl VtkPointSource for vtkPointSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_source_new_instance(self.0) }
    }
    fn set_number_of_points(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_point_source_set_number_of_points(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_point_source_set_number_of_points(self.0, _arg) }
    }
    fn get_number_of_points_min_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_point_source_get_number_of_points_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_point_source_get_number_of_points_min_value(self.0) }
    }
    fn get_number_of_points_max_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_point_source_get_number_of_points_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_point_source_get_number_of_points_max_value(self.0) }
    }
    fn get_number_of_points(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_point_source_get_number_of_points(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_point_source_get_number_of_points(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_point_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_point_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_point_source_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_point_source_set_radius(self.0, _arg) }
    }
    fn get_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_point_source_get_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_point_source_get_radius_min_value(self.0) }
    }
    fn get_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_point_source_get_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_point_source_get_radius_max_value(self.0) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_point_source_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_point_source_get_radius(self.0) }
    }
    fn set_distribution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_point_source_set_distribution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_point_source_set_distribution(self.0, _arg) }
    }
    fn set_distribution_to_uniform(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_point_source_set_distribution_to_uniform(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_point_source_set_distribution_to_uniform(self.0) }
    }
    fn set_distribution_to_shell(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_point_source_set_distribution_to_shell(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_point_source_set_distribution_to_shell(self.0) }
    }
    fn get_distribution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_point_source_get_distribution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_point_source_get_distribution(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_point_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_point_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_point_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_point_source_get_output_points_precision(self.0) }
    }
    fn set_random_sequence(&mut self, randomSequence: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_point_source_set_random_sequence(
                sself: *mut core::ffi::c_void,
                randomSequence: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_point_source_set_random_sequence(self.0, randomSequence) }
    }
    fn get_random_sequence(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_point_source_get_random_sequence(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_point_source_get_random_sequence(self.0) }
    }
}
impl VtkPolyLineSource for vtkPolyLineSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_line_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_line_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_line_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_line_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_line_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_line_source_new_instance(self.0) }
    }
    fn set_closed(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_poly_line_source_set_closed(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_poly_line_source_set_closed(self.0, _arg) }
    }
    fn get_closed(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_poly_line_source_get_closed(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_poly_line_source_get_closed(self.0) }
    }
    fn closed_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_poly_line_source_closed_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_poly_line_source_closed_on(self.0) }
    }
    fn closed_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_poly_line_source_closed_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_poly_line_source_closed_off(self.0) }
    }
}
impl VtkPolyPointSource for vtkPolyPointSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_point_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_point_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_point_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_point_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_point_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_point_source_new_instance(self.0) }
    }
    fn set_number_of_points(&mut self, numPoints: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_poly_point_source_set_number_of_points(
                sself: *mut core::ffi::c_void,
                numPoints: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_poly_point_source_set_number_of_points(self.0, numPoints) }
    }
    fn get_number_of_points(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_poly_point_source_get_number_of_points(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_poly_point_source_get_number_of_points(self.0) }
    }
    fn resize(&mut self, numPoints: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_poly_point_source_resize(
                sself: *mut core::ffi::c_void,
                numPoints: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_poly_point_source_resize(self.0, numPoints) }
    }
    fn set_point(
        &mut self,
        id: core::ffi::c_longlong,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_poly_point_source_set_point(
                sself: *mut core::ffi::c_void,
                id: core::ffi::c_longlong,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_poly_point_source_set_point(self.0, id, x, y, z) }
    }
    fn set_points(&mut self, points: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_poly_point_source_set_points(
                sself: *mut core::ffi::c_void,
                points: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_poly_point_source_set_points(self.0, points) }
    }
    fn get_points(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_poly_point_source_get_points(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_poly_point_source_get_points(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_poly_point_source_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_poly_point_source_get_m_time(self.0) }
    }
}
impl VtkProgrammableDataObjectSource for vtkProgrammableDataObjectSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_data_object_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_data_object_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_data_object_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_data_object_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_data_object_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_data_object_source_new_instance(self.0) }
    }
    fn set_execute_method_arg_delete(&mut self, f: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_programmable_data_object_source_set_execute_method_arg_delete(
                sself: *mut core::ffi::c_void,
                f: *mut core::ffi::c_void,
            );
        }
        unsafe {
            vtk_programmable_data_object_source_set_execute_method_arg_delete(self.0, f)
        }
    }
}
impl VtkProgrammableSource for vtkProgrammableSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_new_instance(self.0) }
    }
    fn set_execute_method_arg_delete(&mut self, f: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_programmable_source_set_execute_method_arg_delete(
                sself: *mut core::ffi::c_void,
                f: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_programmable_source_set_execute_method_arg_delete(self.0, f) }
    }
    fn set_request_information_method(&mut self, f: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_programmable_source_set_request_information_method(
                sself: *mut core::ffi::c_void,
                f: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_programmable_source_set_request_information_method(self.0, f) }
    }
    fn get_poly_data_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_get_poly_data_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_get_poly_data_output(self.0) }
    }
    fn get_structured_points_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_get_structured_points_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_get_structured_points_output(self.0) }
    }
    fn get_structured_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_get_structured_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_get_structured_grid_output(self.0) }
    }
    fn get_unstructured_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_get_unstructured_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_get_unstructured_grid_output(self.0) }
    }
    fn get_rectilinear_grid_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_get_rectilinear_grid_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_get_rectilinear_grid_output(self.0) }
    }
    fn get_graph_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_get_graph_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_get_graph_output(self.0) }
    }
    fn get_molecule_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_get_molecule_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_get_molecule_output(self.0) }
    }
    fn get_table_output(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_programmable_source_get_table_output(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_programmable_source_get_table_output(self.0) }
    }
}
impl VtkRandomHyperTreeGridSource for vtkRandomHyperTreeGridSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_random_hyper_tree_grid_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_random_hyper_tree_grid_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_random_hyper_tree_grid_source_new_instance(self.0) }
    }
    fn set_dimensions(
        &mut self,
        _arg1: core::ffi::c_uint,
        _arg2: core::ffi::c_uint,
        _arg3: core::ffi::c_uint,
    ) -> () {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_set_dimensions(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_uint,
                _arg2: core::ffi::c_uint,
                _arg3: core::ffi::c_uint,
            );
        }
        unsafe {
            vtk_random_hyper_tree_grid_source_set_dimensions(self.0, _arg1, _arg2, _arg3)
        }
    }
    fn set_output_bounds(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
        _arg5: core::ffi::c_double,
        _arg6: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_set_output_bounds(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
                _arg4: core::ffi::c_double,
                _arg5: core::ffi::c_double,
                _arg6: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_random_hyper_tree_grid_source_set_output_bounds(
                self.0,
                _arg1,
                _arg2,
                _arg3,
                _arg4,
                _arg5,
                _arg6,
            )
        }
    }
    fn get_seed(&mut self) -> core::ffi::c_uint {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_get_seed(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_uint;
        }
        unsafe { vtk_random_hyper_tree_grid_source_get_seed(self.0) }
    }
    fn set_seed(&mut self, _arg: core::ffi::c_uint) -> () {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_set_seed(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_uint,
            );
        }
        unsafe { vtk_random_hyper_tree_grid_source_set_seed(self.0, _arg) }
    }
    fn get_max_depth(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_get_max_depth(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_hyper_tree_grid_source_get_max_depth(self.0) }
    }
    fn set_max_depth(&mut self, _arg: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_set_max_depth(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_random_hyper_tree_grid_source_set_max_depth(self.0, _arg) }
    }
    fn get_max_depth_min_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_get_max_depth_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_hyper_tree_grid_source_get_max_depth_min_value(self.0) }
    }
    fn get_max_depth_max_value(&mut self) -> core::ffi::c_longlong {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_get_max_depth_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_longlong;
        }
        unsafe { vtk_random_hyper_tree_grid_source_get_max_depth_max_value(self.0) }
    }
    fn get_split_fraction(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_get_split_fraction(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_random_hyper_tree_grid_source_get_split_fraction(self.0) }
    }
    fn set_split_fraction(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_set_split_fraction(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_random_hyper_tree_grid_source_set_split_fraction(self.0, _arg) }
    }
    fn get_split_fraction_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_get_split_fraction_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_random_hyper_tree_grid_source_get_split_fraction_min_value(self.0) }
    }
    fn get_split_fraction_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_random_hyper_tree_grid_source_get_split_fraction_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_random_hyper_tree_grid_source_get_split_fraction_max_value(self.0) }
    }
}
impl VtkRectangularButtonSource for vtkRectangularButtonSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_rectangular_button_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_rectangular_button_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_rectangular_button_source_new(self.0) }
    }
    fn set_width(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_set_width(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_rectangular_button_source_set_width(self.0, _arg) }
    }
    fn get_width_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_width_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_width_min_value(self.0) }
    }
    fn get_width_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_width_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_width_max_value(self.0) }
    }
    fn get_width(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_width(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_width(self.0) }
    }
    fn set_height(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_set_height(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_rectangular_button_source_set_height(self.0, _arg) }
    }
    fn get_height_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_height_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_height_min_value(self.0) }
    }
    fn get_height_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_height_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_height_max_value(self.0) }
    }
    fn get_height(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_height(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_height(self.0) }
    }
    fn set_depth(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_set_depth(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_rectangular_button_source_set_depth(self.0, _arg) }
    }
    fn get_depth_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_depth_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_depth_min_value(self.0) }
    }
    fn get_depth_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_depth_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_depth_max_value(self.0) }
    }
    fn get_depth(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_depth(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_depth(self.0) }
    }
    fn set_box_ratio(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_set_box_ratio(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_rectangular_button_source_set_box_ratio(self.0, _arg) }
    }
    fn get_box_ratio_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_box_ratio_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_box_ratio_min_value(self.0) }
    }
    fn get_box_ratio_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_box_ratio_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_box_ratio_max_value(self.0) }
    }
    fn get_box_ratio(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_box_ratio(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_box_ratio(self.0) }
    }
    fn set_texture_ratio(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_set_texture_ratio(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_rectangular_button_source_set_texture_ratio(self.0, _arg) }
    }
    fn get_texture_ratio_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_texture_ratio_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_texture_ratio_min_value(self.0) }
    }
    fn get_texture_ratio_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_texture_ratio_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_texture_ratio_max_value(self.0) }
    }
    fn get_texture_ratio(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_texture_ratio(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_texture_ratio(self.0) }
    }
    fn set_texture_height_ratio(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_set_texture_height_ratio(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_rectangular_button_source_set_texture_height_ratio(self.0, _arg) }
    }
    fn get_texture_height_ratio_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_texture_height_ratio_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe {
            vtk_rectangular_button_source_get_texture_height_ratio_min_value(self.0)
        }
    }
    fn get_texture_height_ratio_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_texture_height_ratio_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe {
            vtk_rectangular_button_source_get_texture_height_ratio_max_value(self.0)
        }
    }
    fn get_texture_height_ratio(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_texture_height_ratio(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_rectangular_button_source_get_texture_height_ratio(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe {
            vtk_rectangular_button_source_set_output_points_precision(self.0, _arg)
        }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_rectangular_button_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_rectangular_button_source_get_output_points_precision(self.0) }
    }
}
impl VtkRegularPolygonSource for vtkRegularPolygonSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_regular_polygon_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_regular_polygon_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_regular_polygon_source_new_instance(self.0) }
    }
    fn set_number_of_sides(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_set_number_of_sides(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_regular_polygon_source_set_number_of_sides(self.0, _arg) }
    }
    fn get_number_of_sides_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_get_number_of_sides_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_regular_polygon_source_get_number_of_sides_min_value(self.0) }
    }
    fn get_number_of_sides_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_get_number_of_sides_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_regular_polygon_source_get_number_of_sides_max_value(self.0) }
    }
    fn get_number_of_sides(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_get_number_of_sides(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_regular_polygon_source_get_number_of_sides(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_regular_polygon_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_normal(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_set_normal(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_regular_polygon_source_set_normal(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_regular_polygon_source_set_radius(self.0, _arg) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_regular_polygon_source_get_radius(self.0) }
    }
    fn set_generate_polygon(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_set_generate_polygon(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_regular_polygon_source_set_generate_polygon(self.0, _arg) }
    }
    fn get_generate_polygon(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_get_generate_polygon(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_regular_polygon_source_get_generate_polygon(self.0) }
    }
    fn generate_polygon_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_generate_polygon_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_regular_polygon_source_generate_polygon_on(self.0) }
    }
    fn generate_polygon_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_generate_polygon_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_regular_polygon_source_generate_polygon_off(self.0) }
    }
    fn set_generate_polyline(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_set_generate_polyline(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_regular_polygon_source_set_generate_polyline(self.0, _arg) }
    }
    fn get_generate_polyline(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_get_generate_polyline(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_regular_polygon_source_get_generate_polyline(self.0) }
    }
    fn generate_polyline_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_generate_polyline_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_regular_polygon_source_generate_polyline_on(self.0) }
    }
    fn generate_polyline_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_generate_polyline_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_regular_polygon_source_generate_polyline_off(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_regular_polygon_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_regular_polygon_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_regular_polygon_source_get_output_points_precision(self.0) }
    }
}
impl VtkSelectionSource for vtkSelectionSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_selection_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_selection_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_selection_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_selection_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_selection_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_selection_source_new_instance(self.0) }
    }
    fn add_id(&mut self, piece: core::ffi::c_longlong, id: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_add_id(
                sself: *mut core::ffi::c_void,
                piece: core::ffi::c_longlong,
                id: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_selection_source_add_id(self.0, piece, id) }
    }
    fn add_string_id(&mut self, piece: core::ffi::c_longlong, id: &str) -> () {
        let c_id = std::ffi::CString::new(id).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_selection_source_add_string_id(
                sself: *mut core::ffi::c_void,
                piece: core::ffi::c_longlong,
                id: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_selection_source_add_string_id(self.0, piece, c_id.as_ptr()) }
    }
    fn add_location(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_add_location(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_selection_source_add_location(self.0, x, y, z) }
    }
    fn add_threshold(
        &mut self,
        min: core::ffi::c_double,
        max: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_add_threshold(
                sself: *mut core::ffi::c_void,
                min: core::ffi::c_double,
                max: core::ffi::c_double,
            );
        }
        unsafe { vtk_selection_source_add_threshold(self.0, min, max) }
    }
    fn add_block(&mut self, blockno: core::ffi::c_longlong) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_add_block(
                sself: *mut core::ffi::c_void,
                blockno: core::ffi::c_longlong,
            );
        }
        unsafe { vtk_selection_source_add_block(self.0, blockno) }
    }
    fn add_block_selector(&mut self, selector: &str) -> () {
        let c_selector = std::ffi::CString::new(selector).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_selection_source_add_block_selector(
                sself: *mut core::ffi::c_void,
                selector: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_selection_source_add_block_selector(self.0, c_selector.as_ptr()) }
    }
    fn remove_all_block_selectors(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_remove_all_block_selectors(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_selection_source_remove_all_block_selectors(self.0) }
    }
    fn remove_all_i_ds(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_remove_all_i_ds(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_selection_source_remove_all_i_ds(self.0) }
    }
    fn remove_all_string_i_ds(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_remove_all_string_i_ds(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_selection_source_remove_all_string_i_ds(self.0) }
    }
    fn remove_all_thresholds(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_remove_all_thresholds(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_selection_source_remove_all_thresholds(self.0) }
    }
    fn remove_all_locations(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_remove_all_locations(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_selection_source_remove_all_locations(self.0) }
    }
    fn remove_all_blocks(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_remove_all_blocks(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_selection_source_remove_all_blocks(self.0) }
    }
    fn set_content_type(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_content_type(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_content_type(self.0, _arg) }
    }
    fn get_content_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_content_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_content_type(self.0) }
    }
    fn set_field_type(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_field_type(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_field_type(self.0, _arg) }
    }
    fn get_field_type(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_field_type(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_field_type(self.0) }
    }
    fn set_containing_cells(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_containing_cells(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_containing_cells(self.0, _arg) }
    }
    fn get_containing_cells(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_containing_cells(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_containing_cells(self.0) }
    }
    fn set_number_of_layers(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_number_of_layers(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_number_of_layers(self.0, _arg) }
    }
    fn get_number_of_layers_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_number_of_layers_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_number_of_layers_min_value(self.0) }
    }
    fn get_number_of_layers_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_number_of_layers_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_number_of_layers_max_value(self.0) }
    }
    fn get_number_of_layers(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_number_of_layers(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_number_of_layers(self.0) }
    }
    fn set_inverse(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_inverse(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_inverse(self.0, _arg) }
    }
    fn get_inverse(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_inverse(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_inverse(self.0) }
    }
    fn set_array_name(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_selection_source_set_array_name(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_selection_source_set_array_name(self.0, c__arg.as_ptr()) }
    }
    fn set_array_component(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_array_component(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_array_component(self.0, _arg) }
    }
    fn get_array_component(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_array_component(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_array_component(self.0) }
    }
    fn set_composite_index(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_composite_index(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_composite_index(self.0, _arg) }
    }
    fn get_composite_index(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_composite_index(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_composite_index(self.0) }
    }
    fn set_hierarchical_level(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_hierarchical_level(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_hierarchical_level(self.0, _arg) }
    }
    fn get_hierarchical_level(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_hierarchical_level(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_hierarchical_level(self.0) }
    }
    fn set_hierarchical_index(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_set_hierarchical_index(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_selection_source_set_hierarchical_index(self.0, _arg) }
    }
    fn get_hierarchical_index(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_selection_source_get_hierarchical_index(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_selection_source_get_hierarchical_index(self.0) }
    }
    fn set_assembly_name(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_selection_source_set_assembly_name(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_selection_source_set_assembly_name(self.0, c__arg.as_ptr()) }
    }
    fn add_selector(&mut self, selector: &str) -> () {
        let c_selector = std::ffi::CString::new(selector).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_selection_source_add_selector(
                sself: *mut core::ffi::c_void,
                selector: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_selection_source_add_selector(self.0, c_selector.as_ptr()) }
    }
    fn remove_all_selectors(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_selection_source_remove_all_selectors(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_selection_source_remove_all_selectors(self.0) }
    }
    fn set_query_string(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_selection_source_set_query_string(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_selection_source_set_query_string(self.0, c__arg.as_ptr()) }
    }
}
impl VtkSphereSource for vtkSphereSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sphere_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sphere_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sphere_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sphere_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_sphere_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_sphere_source_new(self.0) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_sphere_source_set_radius(self.0, _arg) }
    }
    fn get_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_radius_min_value(self.0) }
    }
    fn get_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_radius_max_value(self.0) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_radius(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_sphere_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_theta_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_theta_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_sphere_source_set_theta_resolution(self.0, _arg) }
    }
    fn get_theta_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_theta_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_theta_resolution_min_value(self.0) }
    }
    fn get_theta_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_theta_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_theta_resolution_max_value(self.0) }
    }
    fn get_theta_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_theta_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_theta_resolution(self.0) }
    }
    fn set_phi_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_phi_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_sphere_source_set_phi_resolution(self.0, _arg) }
    }
    fn get_phi_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_phi_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_phi_resolution_min_value(self.0) }
    }
    fn get_phi_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_phi_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_phi_resolution_max_value(self.0) }
    }
    fn get_phi_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_phi_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_phi_resolution(self.0) }
    }
    fn set_start_theta(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_start_theta(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_sphere_source_set_start_theta(self.0, _arg) }
    }
    fn get_start_theta_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_start_theta_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_start_theta_min_value(self.0) }
    }
    fn get_start_theta_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_start_theta_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_start_theta_max_value(self.0) }
    }
    fn get_start_theta(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_start_theta(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_start_theta(self.0) }
    }
    fn set_end_theta(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_end_theta(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_sphere_source_set_end_theta(self.0, _arg) }
    }
    fn get_end_theta_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_end_theta_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_end_theta_min_value(self.0) }
    }
    fn get_end_theta_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_end_theta_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_end_theta_max_value(self.0) }
    }
    fn get_end_theta(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_end_theta(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_end_theta(self.0) }
    }
    fn set_start_phi(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_start_phi(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_sphere_source_set_start_phi(self.0, _arg) }
    }
    fn get_start_phi_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_start_phi_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_start_phi_min_value(self.0) }
    }
    fn get_start_phi_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_start_phi_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_start_phi_max_value(self.0) }
    }
    fn get_start_phi(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_start_phi(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_start_phi(self.0) }
    }
    fn set_end_phi(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_end_phi(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_sphere_source_set_end_phi(self.0, _arg) }
    }
    fn get_end_phi_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_end_phi_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_end_phi_min_value(self.0) }
    }
    fn get_end_phi_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_end_phi_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_end_phi_max_value(self.0) }
    }
    fn get_end_phi(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_sphere_source_get_end_phi(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_sphere_source_get_end_phi(self.0) }
    }
    fn set_lat_long_tessellation(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_lat_long_tessellation(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_sphere_source_set_lat_long_tessellation(self.0, _arg) }
    }
    fn get_lat_long_tessellation(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_lat_long_tessellation(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_lat_long_tessellation(self.0) }
    }
    fn lat_long_tessellation_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_lat_long_tessellation_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_sphere_source_lat_long_tessellation_on(self.0) }
    }
    fn lat_long_tessellation_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_lat_long_tessellation_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_sphere_source_lat_long_tessellation_off(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_sphere_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_output_points_precision(self.0) }
    }
    fn set_generate_normals(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_set_generate_normals(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_sphere_source_set_generate_normals(self.0, _arg) }
    }
    fn get_generate_normals(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_sphere_source_get_generate_normals(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_sphere_source_get_generate_normals(self.0) }
    }
    fn generate_normals_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_generate_normals_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_sphere_source_generate_normals_on(self.0) }
    }
    fn generate_normals_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_sphere_source_generate_normals_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_sphere_source_generate_normals_off(self.0) }
    }
}
impl VtkSuperquadricSource for vtkSuperquadricSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_superquadric_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_superquadric_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_superquadric_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_superquadric_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_superquadric_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_superquadric_source_new_instance(self.0) }
    }
    fn set_center(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_center(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_superquadric_source_set_center(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_scale(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_scale(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_superquadric_source_set_scale(self.0, _arg1, _arg2, _arg3) }
    }
    fn get_theta_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_theta_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_superquadric_source_get_theta_resolution(self.0) }
    }
    fn set_theta_resolution(&mut self, i: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_theta_resolution(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            );
        }
        unsafe { vtk_superquadric_source_set_theta_resolution(self.0, i) }
    }
    fn get_phi_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_phi_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_superquadric_source_get_phi_resolution(self.0) }
    }
    fn set_phi_resolution(&mut self, i: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_phi_resolution(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            );
        }
        unsafe { vtk_superquadric_source_set_phi_resolution(self.0, i) }
    }
    fn get_thickness(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_thickness(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_superquadric_source_get_thickness(self.0) }
    }
    fn set_thickness(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_thickness(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_superquadric_source_set_thickness(self.0, _arg) }
    }
    fn get_thickness_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_thickness_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_superquadric_source_get_thickness_min_value(self.0) }
    }
    fn get_thickness_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_thickness_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_superquadric_source_get_thickness_max_value(self.0) }
    }
    fn get_phi_roundness(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_phi_roundness(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_superquadric_source_get_phi_roundness(self.0) }
    }
    fn set_phi_roundness(&mut self, e: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_phi_roundness(
                sself: *mut core::ffi::c_void,
                e: core::ffi::c_double,
            );
        }
        unsafe { vtk_superquadric_source_set_phi_roundness(self.0, e) }
    }
    fn get_theta_roundness(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_theta_roundness(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_superquadric_source_get_theta_roundness(self.0) }
    }
    fn set_theta_roundness(&mut self, e: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_theta_roundness(
                sself: *mut core::ffi::c_void,
                e: core::ffi::c_double,
            );
        }
        unsafe { vtk_superquadric_source_set_theta_roundness(self.0, e) }
    }
    fn set_size(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_size(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_superquadric_source_set_size(self.0, _arg) }
    }
    fn get_size(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_size(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_superquadric_source_get_size(self.0) }
    }
    fn set_axis_of_symmetry(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_axis_of_symmetry(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_superquadric_source_set_axis_of_symmetry(self.0, _arg) }
    }
    fn get_axis_of_symmetry(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_axis_of_symmetry(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_superquadric_source_get_axis_of_symmetry(self.0) }
    }
    fn set_x_axis_of_symmetry(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_x_axis_of_symmetry(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_superquadric_source_set_x_axis_of_symmetry(self.0) }
    }
    fn set_y_axis_of_symmetry(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_y_axis_of_symmetry(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_superquadric_source_set_y_axis_of_symmetry(self.0) }
    }
    fn set_z_axis_of_symmetry(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_z_axis_of_symmetry(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_superquadric_source_set_z_axis_of_symmetry(self.0) }
    }
    fn toroidal_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_toroidal_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_superquadric_source_toroidal_on(self.0) }
    }
    fn toroidal_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_toroidal_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_superquadric_source_toroidal_off(self.0) }
    }
    fn get_toroidal(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_toroidal(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_superquadric_source_get_toroidal(self.0) }
    }
    fn set_toroidal(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_toroidal(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_superquadric_source_set_toroidal(self.0, _arg) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_superquadric_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_superquadric_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_superquadric_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_superquadric_source_get_output_points_precision(self.0) }
    }
}
impl VtkTessellatedBoxSource for vtkTessellatedBoxSource {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_tessellated_box_source_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_tessellated_box_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_tessellated_box_source_new_instance(self.0) }
    }
    fn set_bounds(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
        _arg4: core::ffi::c_double,
        _arg5: core::ffi::c_double,
        _arg6: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_set_bounds(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
                _arg4: core::ffi::c_double,
                _arg5: core::ffi::c_double,
                _arg6: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_tessellated_box_source_set_bounds(
                self.0,
                _arg1,
                _arg2,
                _arg3,
                _arg4,
                _arg5,
                _arg6,
            )
        }
    }
    fn set_level(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_set_level(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_tessellated_box_source_set_level(self.0, _arg) }
    }
    fn get_level(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_get_level(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_tessellated_box_source_get_level(self.0) }
    }
    fn set_duplicate_shared_points(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_set_duplicate_shared_points(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_tessellated_box_source_set_duplicate_shared_points(self.0, _arg) }
    }
    fn get_duplicate_shared_points(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_get_duplicate_shared_points(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_tessellated_box_source_get_duplicate_shared_points(self.0) }
    }
    fn duplicate_shared_points_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_duplicate_shared_points_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_tessellated_box_source_duplicate_shared_points_on(self.0) }
    }
    fn duplicate_shared_points_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_duplicate_shared_points_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_tessellated_box_source_duplicate_shared_points_off(self.0) }
    }
    fn set_quads(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_set_quads(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_tessellated_box_source_set_quads(self.0, _arg) }
    }
    fn get_quads(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_get_quads(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_tessellated_box_source_get_quads(self.0) }
    }
    fn quads_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_quads_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_tessellated_box_source_quads_on(self.0) }
    }
    fn quads_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_quads_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_tessellated_box_source_quads_off(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_tessellated_box_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_tessellated_box_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_tessellated_box_source_get_output_points_precision(self.0) }
    }
}
impl VtkTextSource for vtkTextSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_text_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_text_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_text_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_text_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_text_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_text_source_new(self.0) }
    }
    fn set_text(&mut self, _arg: &str) -> () {
        let c__arg = std::ffi::CString::new(_arg).expect("CString::new failed");
        unsafe extern "C" {
            fn vtk_text_source_set_text(
                sself: *mut core::ffi::c_void,
                _arg: *const core::ffi::c_char,
            );
        }
        unsafe { vtk_text_source_set_text(self.0, c__arg.as_ptr()) }
    }
    fn set_backing(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_text_source_set_backing(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_text_source_set_backing(self.0, _arg) }
    }
    fn get_backing(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_text_source_get_backing(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_text_source_get_backing(self.0) }
    }
    fn backing_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_text_source_backing_on(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_text_source_backing_on(self.0) }
    }
    fn backing_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_text_source_backing_off(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_text_source_backing_off(self.0) }
    }
    fn set_foreground_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_text_source_set_foreground_color(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_text_source_set_foreground_color(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_background_color(
        &mut self,
        _arg1: core::ffi::c_double,
        _arg2: core::ffi::c_double,
        _arg3: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_text_source_set_background_color(
                sself: *mut core::ffi::c_void,
                _arg1: core::ffi::c_double,
                _arg2: core::ffi::c_double,
                _arg3: core::ffi::c_double,
            );
        }
        unsafe { vtk_text_source_set_background_color(self.0, _arg1, _arg2, _arg3) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_text_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_text_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_text_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_text_source_get_output_points_precision(self.0) }
    }
}
impl VtkTexturedSphereSource for vtkTexturedSphereSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_textured_sphere_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_textured_sphere_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_textured_sphere_source_new(self.0) }
    }
    fn set_radius(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_set_radius(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_textured_sphere_source_set_radius(self.0, _arg) }
    }
    fn get_radius_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_radius_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_radius_min_value(self.0) }
    }
    fn get_radius_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_radius_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_radius_max_value(self.0) }
    }
    fn get_radius(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_radius(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_radius(self.0) }
    }
    fn set_theta_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_set_theta_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_textured_sphere_source_set_theta_resolution(self.0, _arg) }
    }
    fn get_theta_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_theta_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_textured_sphere_source_get_theta_resolution_min_value(self.0) }
    }
    fn get_theta_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_theta_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_textured_sphere_source_get_theta_resolution_max_value(self.0) }
    }
    fn get_theta_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_theta_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_textured_sphere_source_get_theta_resolution(self.0) }
    }
    fn set_phi_resolution(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_set_phi_resolution(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_textured_sphere_source_set_phi_resolution(self.0, _arg) }
    }
    fn get_phi_resolution_min_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_phi_resolution_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_textured_sphere_source_get_phi_resolution_min_value(self.0) }
    }
    fn get_phi_resolution_max_value(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_phi_resolution_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_textured_sphere_source_get_phi_resolution_max_value(self.0) }
    }
    fn get_phi_resolution(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_phi_resolution(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_textured_sphere_source_get_phi_resolution(self.0) }
    }
    fn set_theta(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_set_theta(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_textured_sphere_source_set_theta(self.0, _arg) }
    }
    fn get_theta_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_theta_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_theta_min_value(self.0) }
    }
    fn get_theta_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_theta_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_theta_max_value(self.0) }
    }
    fn get_theta(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_theta(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_theta(self.0) }
    }
    fn set_phi(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_set_phi(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_textured_sphere_source_set_phi(self.0, _arg) }
    }
    fn get_phi_min_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_phi_min_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_phi_min_value(self.0) }
    }
    fn get_phi_max_value(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_phi_max_value(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_phi_max_value(self.0) }
    }
    fn get_phi(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_phi(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_textured_sphere_source_get_phi(self.0) }
    }
    fn set_output_points_precision(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_set_output_points_precision(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_textured_sphere_source_set_output_points_precision(self.0, _arg) }
    }
    fn get_output_points_precision(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_textured_sphere_source_get_output_points_precision(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_textured_sphere_source_get_output_points_precision(self.0) }
    }
}
impl VtkUniformHyperTreeGridSource for vtkUniformHyperTreeGridSource {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_hyper_tree_grid_source_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_hyper_tree_grid_source_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_hyper_tree_grid_source_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_hyper_tree_grid_source_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_uniform_hyper_tree_grid_source_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_uniform_hyper_tree_grid_source_new(self.0) }
    }
}
/// create a circular arc
///
///
///
/// vtkArcSource is a source object that creates an arc defined by two
/// endpoints and a center. The number of segments composing the polyline
/// is controlled by setting the object resolution.
/// Alternatively, one can use a better API (that does not allow for
/// inconsistent nor ambiguous inputs), using a starting point (polar vector,
/// measured from the arc's center), a normal to the plane of the arc,
/// and an angle defining the arc length.
/// Since the default API remains the original one, in order to use
/// the improved API, one must switch the UseNormalAndAngle flag to TRUE.
///
/// The development of an improved, consistent API (based on point, normal,
/// and angle) was supported by CEA/DIF - Commissariat a l'Energie Atomique,
/// Centre DAM Ile-De-France, BP12, F-91297 Arpajon, France, and implemented
/// by Philippe Pebay, Kitware SAS 2012.
///
/// @sa
/// vtkEllipseArcSource
#[allow(non_camel_case_types)]
pub struct vtkArcSource(*mut core::ffi::c_void);
impl vtkArcSource {
    /// Creates a new [vtkArcSource] via `vtkArcSource::New()`
    #[doc(alias = "vtkArcSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkArcSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkArcSource_new() })
    }
}
impl std::default::Default for vtkArcSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkArcSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkArcSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkArcSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkArcSource_create_drop() {
    let obj = vtkArcSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Appends a cylinder to a cone to form an arrow.
///
///
/// vtkArrowSource was intended to be used as the source for a glyph.
/// The shaft base is always at (0,0,0). The arrow tip is always at (1,0,0). If
/// "Invert" is true, then the ends are flipped i.e. tip is at (0,0,0) while
/// base is at (1, 0, 0).
/// The resolution of the cone and shaft can be set and default to 6.
/// The radius of the cone and shaft can be set and default to 0.03 and 0.1.
/// The length of the tip can also be set, and defaults to 0.35.
#[allow(non_camel_case_types)]
pub struct vtkArrowSource(*mut core::ffi::c_void);
impl vtkArrowSource {
    /// Creates a new [vtkArrowSource] via `vtkArrowSource::New()`
    #[doc(alias = "vtkArrowSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkArrowSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkArrowSource_new() })
    }
}
impl std::default::Default for vtkArrowSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkArrowSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkArrowSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkArrowSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkArrowSource_create_drop() {
    let obj = vtkArrowSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate a capsule centered at the origin
///
///
/// vtkCapsuleSource creates a capsule (represented by polygons) of specified
/// radius centered at the origin. The resolution (polygonal discretization) in
/// both the latitude (phi) and longitude (theta) directions can be specified as
/// well as the length of the capsule cylinder (CylinderLength). By default, the
/// surface tessellation of the sphere uses triangles; however you can set
/// LatLongTessellation to produce a tessellation using quadrilaterals (except
/// at the poles of the capsule).
#[allow(non_camel_case_types)]
pub struct vtkCapsuleSource(*mut core::ffi::c_void);
impl vtkCapsuleSource {
    /// Creates a new [vtkCapsuleSource] via `vtkCapsuleSource::New()`
    #[doc(alias = "vtkCapsuleSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCapsuleSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCapsuleSource_new() })
    }
}
impl std::default::Default for vtkCapsuleSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCapsuleSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCapsuleSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCapsuleSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCapsuleSource_create_drop() {
    let obj = vtkCapsuleSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Create cells of a given type
///
///
/// vtkCellTypeSource is a source object that creates cells of the given
/// input type. BlocksDimensions specifies the number of cell "blocks" in each
/// direction. A cell block may be divided into multiple cells based on
/// the chosen cell type (e.g. 6 pyramid cells make up a single cell block).
/// If a 1D cell is selected then only the first dimension is
/// used to specify how many cells are generated. If a 2D cell is
/// selected then only the first and second dimensions are used to
/// determine how many cells are created. The source respects pieces.
#[allow(non_camel_case_types)]
pub struct vtkCellTypeSource(*mut core::ffi::c_void);
impl vtkCellTypeSource {
    /// Creates a new [vtkCellTypeSource] via `vtkCellTypeSource::New()`
    #[doc(alias = "vtkCellTypeSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCellTypeSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCellTypeSource_new() })
    }
}
impl std::default::Default for vtkCellTypeSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCellTypeSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCellTypeSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCellTypeSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCellTypeSource_create_drop() {
    let obj = vtkCellTypeSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// generate polygonal cone
///
///
/// vtkConeSource creates a cone centered at a specified point and pointing in
/// a specified direction. (By default, the center is the origin and the
/// direction is the x-axis.) Depending upon the resolution of this object,
/// different representations are created. If resolution=0 a line is created;
/// if resolution=1, a single triangle is created; if resolution=2, two
/// crossed triangles are created. For resolution > 2, a 3D cone (with
/// resolution number of sides) is created. It also is possible to control
/// whether the bottom of the cone is capped with a (resolution-sided)
/// polygon, and to specify the height and radius of the cone.
#[allow(non_camel_case_types)]
pub struct vtkConeSource(*mut core::ffi::c_void);
impl vtkConeSource {
    /// Creates a new [vtkConeSource] via `vtkConeSource::New()`
    #[doc(alias = "vtkConeSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkConeSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkConeSource_new() })
    }
}
impl std::default::Default for vtkConeSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkConeSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkConeSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkConeSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkConeSource_create_drop() {
    let obj = vtkConeSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a polygonal representation of a cube
///
///
/// vtkCubeSource creates a cube centered at origin. The cube is represented
/// with four-sided polygons. It is possible to specify the length, width,
/// and height of the cube independently.
#[allow(non_camel_case_types)]
pub struct vtkCubeSource(*mut core::ffi::c_void);
impl vtkCubeSource {
    /// Creates a new [vtkCubeSource] via `vtkCubeSource::New()`
    #[doc(alias = "vtkCubeSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCubeSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCubeSource_new() })
    }
}
impl std::default::Default for vtkCubeSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCubeSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCubeSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCubeSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCubeSource_create_drop() {
    let obj = vtkCubeSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// generate a cylinder centered at origin
///
///
/// vtkCylinderSource creates a polygonal cylinder centered at Center;
/// The axis of the cylinder is aligned along the global y-axis.
/// The height and radius of the cylinder can be specified, as well as the
/// number of sides. It is also possible to control whether the cylinder is
/// open-ended or capped. If you have the end points of the cylinder, you
/// should use a vtkLineSource followed by a vtkTubeFilter instead of the
/// vtkCylinderSource.
#[allow(non_camel_case_types)]
pub struct vtkCylinderSource(*mut core::ffi::c_void);
impl vtkCylinderSource {
    /// Creates a new [vtkCylinderSource] via `vtkCylinderSource::New()`
    #[doc(alias = "vtkCylinderSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCylinderSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCylinderSource_new() })
    }
}
impl std::default::Default for vtkCylinderSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCylinderSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCylinderSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCylinderSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCylinderSource_create_drop() {
    let obj = vtkCylinderSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// generates a sparse or dense square matrix
///
/// with user-specified values for the diagonal, superdiagonal, and subdiagonal.
///
/// @par Thanks:
/// Developed by Timothy M. Shead (tshead@sandia.gov) at Sandia National Laboratories.
#[allow(non_camel_case_types)]
pub struct vtkDiagonalMatrixSource(*mut core::ffi::c_void);
impl vtkDiagonalMatrixSource {
    /// Creates a new [vtkDiagonalMatrixSource] via `vtkDiagonalMatrixSource::New()`
    #[doc(alias = "vtkDiagonalMatrixSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDiagonalMatrixSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDiagonalMatrixSource_new() })
    }
}
impl std::default::Default for vtkDiagonalMatrixSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDiagonalMatrixSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDiagonalMatrixSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDiagonalMatrixSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDiagonalMatrixSource_create_drop() {
    let obj = vtkDiagonalMatrixSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a disk with hole in center
///
///
/// vtkDiskSource creates a polygonal disk with a hole in the center. The
/// disk has zero height. The user can specify the inner and outer radius
/// of the disk, and the radial and circumferential resolution of the
/// polygonal representation.
/// @sa
/// vtkLinearExtrusionFilter
#[allow(non_camel_case_types)]
pub struct vtkDiskSource(*mut core::ffi::c_void);
impl vtkDiskSource {
    /// Creates a new [vtkDiskSource] via `vtkDiskSource::New()`
    #[doc(alias = "vtkDiskSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkDiskSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkDiskSource_new() })
    }
}
impl std::default::Default for vtkDiskSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkDiskSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkDiskSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkDiskSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkDiskSource_create_drop() {
    let obj = vtkDiskSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create an elliptical arc
///
///
///
/// vtkEllipseArcSource is a source object that creates an elliptical arc
/// defined by a normal, a center and the major radius vector.
/// You can define an angle to draw only a section of the ellipse. The number of
/// segments composing the polyline is controlled by setting the object
/// resolution.
///
/// @sa
/// vtkArcSource
#[allow(non_camel_case_types)]
pub struct vtkEllipseArcSource(*mut core::ffi::c_void);
impl vtkEllipseArcSource {
    /// Creates a new [vtkEllipseArcSource] via `vtkEllipseArcSource::New()`
    #[doc(alias = "vtkEllipseArcSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEllipseArcSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkEllipseArcSource_new() })
    }
}
impl std::default::Default for vtkEllipseArcSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEllipseArcSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEllipseArcSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEllipseArcSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEllipseArcSource_create_drop() {
    let obj = vtkEllipseArcSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a ellipsoidal-shaped button
///
///
/// vtkEllipticalButtonSource creates a ellipsoidal shaped button with
/// texture coordinates suitable for application of a texture map. This
/// provides a way to make nice looking 3D buttons. The buttons are
/// represented as vtkPolyData that includes texture coordinates and
/// normals. The button lies in the x-y plane.
///
/// To use this class you must define the major and minor axes lengths of an
/// ellipsoid (expressed as width (x), height (y) and depth (z)). The button
/// has a rectangular mesh region in the center with texture coordinates that
/// range smoothly from (0,1). (This flat region is called the texture
/// region.) The outer, curved portion of the button (called the shoulder) has
/// texture coordinates set to a user specified value (by default (0,0).
/// (This results in coloring the button curve the same color as the (s,t)
/// location of the texture map.) The resolution in the radial direction, the
/// texture region, and the shoulder region must also be set. The button can
/// be moved by specifying an origin.
///
/// @sa
/// vtkButtonSource vtkRectangularButtonSource
#[allow(non_camel_case_types)]
pub struct vtkEllipticalButtonSource(*mut core::ffi::c_void);
impl vtkEllipticalButtonSource {
    /// Creates a new [vtkEllipticalButtonSource] via `vtkEllipticalButtonSource::New()`
    #[doc(alias = "vtkEllipticalButtonSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkEllipticalButtonSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkEllipticalButtonSource_new() })
    }
}
impl std::default::Default for vtkEllipticalButtonSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkEllipticalButtonSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkEllipticalButtonSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkEllipticalButtonSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkEllipticalButtonSource_create_drop() {
    let obj = vtkEllipticalButtonSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a polygonal representation of a frustum
///
///
/// vtkFrustumSource creates a frustum defines by a set of planes. The frustum
/// is represented with four-sided polygons. It is possible to specify extra
/// lines to better visualize the field of view.
///
/// @par Usage:
/// Typical use consists of 3 steps:
/// 1. get the planes coefficients from a vtkCamera with
/// vtkCamera::GetFrustumPlanes()
/// 2. initialize the planes with vtkPlanes::SetFrustumPlanes() with the planes
/// coefficients
/// 3. pass the vtkPlanes to a vtkFrustumSource.
#[allow(non_camel_case_types)]
pub struct vtkFrustumSource(*mut core::ffi::c_void);
impl vtkFrustumSource {
    /// Creates a new [vtkFrustumSource] via `vtkFrustumSource::New()`
    #[doc(alias = "vtkFrustumSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkFrustumSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkFrustumSource_new() })
    }
}
impl std::default::Default for vtkFrustumSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkFrustumSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkFrustumSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkFrustumSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkFrustumSource_create_drop() {
    let obj = vtkFrustumSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create 2D glyphs represented by vtkPolyData
///
///
/// vtkGlyphSource2D can generate a family of 2D glyphs each of which lies
/// in the x-y plane (i.e., the z-coordinate is zero). The class is a helper
/// class to be used with vtkGlyph2D and vtkXYPlotActor.
///
/// To use this class, specify the glyph type to use and its
/// attributes. Attributes include its position (i.e., center point), scale,
/// color, and whether the symbol is filled or not (a polygon or closed line
/// sequence). You can also put a short line through the glyph running from -x
/// to +x (the glyph looks like it's on a line), or a cross.
#[allow(non_camel_case_types)]
pub struct vtkGlyphSource2D(*mut core::ffi::c_void);
impl vtkGlyphSource2D {
    /// Creates a new [vtkGlyphSource2D] via `vtkGlyphSource2D::New()`
    #[doc(alias = "vtkGlyphSource2D")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGlyphSource2D_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkGlyphSource2D_new() })
    }
}
impl std::default::Default for vtkGlyphSource2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGlyphSource2D {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGlyphSource2D_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGlyphSource2D_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGlyphSource2D_create_drop() {
    let obj = vtkGlyphSource2D::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// convert a vtkGraph to vtkPolyData
///
///
///
/// Converts a vtkGraph to a vtkPolyData.  This assumes that the points
/// of the graph have already been filled (perhaps by vtkGraphLayout),
/// and coverts all the edge of the graph into lines in the polydata.
/// The vertex data is passed along to the point data, and the edge data
/// is passed along to the cell data.
///
/// Only the owned graph edges (i.e. edges with ghost level 0) are copied
/// into the vtkPolyData.
#[allow(non_camel_case_types)]
pub struct vtkGraphToPolyData(*mut core::ffi::c_void);
impl vtkGraphToPolyData {
    /// Creates a new [vtkGraphToPolyData] via `vtkGraphToPolyData::New()`
    #[doc(alias = "vtkGraphToPolyData")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGraphToPolyData_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkGraphToPolyData_new() })
    }
}
impl std::default::Default for vtkGraphToPolyData {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGraphToPolyData {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGraphToPolyData_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGraphToPolyData_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGraphToPolyData_create_drop() {
    let obj = vtkGraphToPolyData::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Create a synthetic grid of hypertrees.
///
///
///
/// This class uses input parameters, most notably a string descriptor,
/// to generate a vtkHyperTreeGrid instance representing the corresponding
/// tree-based AMR grid. This descriptor uses the following conventions,
/// e.g., to describe a 1-D ternary subdivision with 2 root cells
/// L0    L1        L2
/// RR  | .R. ... | ...
/// For this tree:
/// HTG:       .
/// /   \
/// L0:     .     .
/// /|\   /|\
/// L1:   c . c c c c
/// /|\
/// L2:   c c c
/// The top level of the tree is not considered a grid level
/// NB: For ease of legibility, white spaces are allowed and ignored.
///
/// @par Thanks:
/// This class was written by Philippe Pebay, Joachim Pouderoux, and Charles Law, Kitware 2013
/// This class was modified by Guenole Harel and Jacques-Bernard Lekien 2014
/// This class was modified by Philippe Pebay, 2016
/// This work was supported by Commissariat a l'Energie Atomique (CEA/DIF)
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkHyperTreeGridSource(*mut core::ffi::c_void);
impl vtkHyperTreeGridSource {
    /// Creates a new [vtkHyperTreeGridSource] via `vtkHyperTreeGridSource::New()`
    #[doc(alias = "vtkHyperTreeGridSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkHyperTreeGridSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkHyperTreeGridSource_new() })
    }
}
impl std::default::Default for vtkHyperTreeGridSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkHyperTreeGridSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkHyperTreeGridSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkHyperTreeGridSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkHyperTreeGridSource_create_drop() {
    let obj = vtkHyperTreeGridSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a line defined by two end points
///
///
/// vtkLineSource is a source object that creates a polyline defined by
/// two endpoints or a collection of connected line segments. To define the line
/// by end points, use `SetPoint1` and `SetPoint2` methods. To define a broken
/// line comprising of multiple line segments, use `SetPoints` to provide the
/// corner points that for the line.
///
/// Intermediate points within line segment (when specifying end points alone) or
/// each of the individual line segments (when specifying broken line) can be
/// specified in two ways. First, when `UseRegularRefinement` is true (default),
/// the `Resolution` is used to determine how many intermediate points to add
/// using regular refinement. Thus, if `Resolution` is set to 1, a mid point will
/// be added for each of the line segments resulting in a line with 3 points: the
/// two end points and the mid point. Second, when `UseRegularRefinement` is
/// false, refinement ratios for points per segment are specified using
/// `SetRefinementRatio` and `SetNumberOfRefinementRatios`. To generate same
/// points as `Resolution` set to 1, the refinement ratios will be `[0, 0.5,
/// 1.0]`. To add the end points of the line segment `0.0` and `1.0` must be
/// included in the collection of refinement ratios.
///
/// @section ChangesVTK9 Changes in VTK 9.0
///
/// Prior to VTK 9.0, when broken line was being generated, the texture
/// coordinates for each of the individual breaks in the line ranged from [0.0,
/// 1.0]. This has been changed to generate texture coordinates in the range
/// [0.0, 1.0] over the entire output line irrespective of whether the line was
/// generated by simply specifying the end points or multiple line segments.
///
/// @par Thanks:
/// This class was extended by Philippe Pebay, Kitware SAS 2011, to support
/// broken lines as well as simple lines.
#[allow(non_camel_case_types)]
pub struct vtkLineSource(*mut core::ffi::c_void);
impl vtkLineSource {
    /// Creates a new [vtkLineSource] via `vtkLineSource::New()`
    #[doc(alias = "vtkLineSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLineSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkLineSource_new() })
    }
}
impl std::default::Default for vtkLineSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLineSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLineSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLineSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLineSource_create_drop() {
    let obj = vtkLineSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create wireframe outline corners for arbitrary data set
///
///
/// vtkOutlineCornerFilter is a filter that generates wireframe outline corners of any
/// data set. The outline consists of the eight corners of the dataset
/// bounding box.
#[allow(non_camel_case_types)]
pub struct vtkOutlineCornerFilter(*mut core::ffi::c_void);
impl vtkOutlineCornerFilter {
    /// Creates a new [vtkOutlineCornerFilter] via `vtkOutlineCornerFilter::New()`
    #[doc(alias = "vtkOutlineCornerFilter")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOutlineCornerFilter_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkOutlineCornerFilter_new() })
    }
}
impl std::default::Default for vtkOutlineCornerFilter {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOutlineCornerFilter {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOutlineCornerFilter_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOutlineCornerFilter_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOutlineCornerFilter_create_drop() {
    let obj = vtkOutlineCornerFilter::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create wireframe outline corners around bounding box
///
///
/// vtkOutlineCornerSource creates wireframe outline corners around a user-specified
/// bounding box.
#[allow(non_camel_case_types)]
pub struct vtkOutlineCornerSource(*mut core::ffi::c_void);
impl vtkOutlineCornerSource {
    /// Creates a new [vtkOutlineCornerSource] via `vtkOutlineCornerSource::New()`
    #[doc(alias = "vtkOutlineCornerSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOutlineCornerSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkOutlineCornerSource_new() })
    }
}
impl std::default::Default for vtkOutlineCornerSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOutlineCornerSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOutlineCornerSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOutlineCornerSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOutlineCornerSource_create_drop() {
    let obj = vtkOutlineCornerSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create wireframe outline around bounding box
///
///
/// vtkOutlineSource creates a wireframe outline around a
/// user-specified bounding box.  The outline may be created aligned
/// with the {x,y,z} axis - in which case it is defined by the 6 bounds
/// {xmin,xmax,ymin,ymax,zmin,zmax} via SetBounds(). Alternatively, the
/// box may be arbitrarily aligned, in which case it should be set via
/// the SetCorners() member.
#[allow(non_camel_case_types)]
pub struct vtkOutlineSource(*mut core::ffi::c_void);
impl vtkOutlineSource {
    /// Creates a new [vtkOutlineSource] via `vtkOutlineSource::New()`
    #[doc(alias = "vtkOutlineSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkOutlineSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkOutlineSource_new() })
    }
}
impl std::default::Default for vtkOutlineSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkOutlineSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkOutlineSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkOutlineSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkOutlineSource_create_drop() {
    let obj = vtkOutlineSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// tessellate parametric functions
///
///
/// This class tessellates parametric functions. The user must specify how
/// many points in the parametric coordinate directions are required (i.e.,
/// the resolution), and the mode to use to generate scalars.
///
/// @par Thanks:
/// Andrew Maclean andrew.amaclean@gmail.com for creating and contributing
/// the class.
///
/// @sa
/// vtkParametricFunction
///
/// @sa
/// Implementation of parametrics for 1D lines:
/// vtkParametricSpline
///
/// @sa
/// Subclasses of vtkParametricFunction implementing non-orentable surfaces:
/// vtkParametricBoy vtkParametricCrossCap vtkParametricFigure8Klein
/// vtkParametricKlein vtkParametricMobius vtkParametricRoman
///
/// @sa
/// Subclasses of vtkParametricFunction implementing orientable surfaces:
/// vtkParametricConicSpiral vtkParametricDini vtkParametricEllipsoid
/// vtkParametricEnneper vtkParametricRandomHills vtkParametricSuperEllipsoid
/// vtkParametricSuperToroid vtkParametricTorus
#[allow(non_camel_case_types)]
pub struct vtkParametricFunctionSource(*mut core::ffi::c_void);
impl vtkParametricFunctionSource {
    /// Creates a new [vtkParametricFunctionSource] via `vtkParametricFunctionSource::New()`
    #[doc(alias = "vtkParametricFunctionSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkParametricFunctionSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkParametricFunctionSource_new() })
    }
}
impl std::default::Default for vtkParametricFunctionSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkParametricFunctionSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkParametricFunctionSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkParametricFunctionSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkParametricFunctionSource_create_drop() {
    let obj = vtkParametricFunctionSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// a source that produces a vtkPartitionedDataSetCollection.
///
///
/// vtkPartitionedDataSetCollection generates a vtkPartitionedDataSetCollection
/// for testing purposes. It uses vtkParametricFunctionSource internally to
/// generate different types of surfaces for each partitioned dataset in the
/// collection. Each partitioned dataset is split among ranks in an even fashion.
/// Thus the number of partitions per rank for a partitioned dataset are always
/// different.
#[allow(non_camel_case_types)]
pub struct vtkPartitionedDataSetCollectionSource(*mut core::ffi::c_void);
impl vtkPartitionedDataSetCollectionSource {
    /// Creates a new [vtkPartitionedDataSetCollectionSource] via `vtkPartitionedDataSetCollectionSource::New()`
    #[doc(alias = "vtkPartitionedDataSetCollectionSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPartitionedDataSetCollectionSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPartitionedDataSetCollectionSource_new() })
    }
}
impl std::default::Default for vtkPartitionedDataSetCollectionSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPartitionedDataSetCollectionSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPartitionedDataSetCollectionSource_destructor(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtkPartitionedDataSetCollectionSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPartitionedDataSetCollectionSource_create_drop() {
    let obj = vtkPartitionedDataSetCollectionSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// a source that produces a vtkPartitionedDataSet.
///
///
/// vtkPartitionedDataSetSource generates a vtkPartitionedDataSet which is
/// composed of partitions of a given vtkParametricFunction.
/// The resulting partitioned dataset is split among ranks in an even fashion
/// by default.
///
/// The user can pass the parametric function to be used using SetParametricFunction.
/// Otherwise it will default to vtkParametricKlein as its Parametric function.
///
/// The partitioning scheme for the produced vtkPartitionedDataSet can be controlled
/// with the methods: SetNumberOfPartitiones, EnableRank, DisableRank, EnableAllRanks,
/// DisableAllRanks.
///
/// @see vtkParametricFunction
/// @see vtkPartitionedDataSet
#[allow(non_camel_case_types)]
pub struct vtkPartitionedDataSetSource(*mut core::ffi::c_void);
impl vtkPartitionedDataSetSource {
    /// Creates a new [vtkPartitionedDataSetSource] via `vtkPartitionedDataSetSource::New()`
    #[doc(alias = "vtkPartitionedDataSetSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPartitionedDataSetSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPartitionedDataSetSource_new() })
    }
}
impl std::default::Default for vtkPartitionedDataSetSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPartitionedDataSetSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPartitionedDataSetSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPartitionedDataSetSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPartitionedDataSetSource_create_drop() {
    let obj = vtkPartitionedDataSetSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create an array of quadrilaterals located in a plane
///
///
/// vtkPlaneSource creates an m x n array of quadrilaterals arranged as
/// a regular tiling in a plane. The plane is defined by specifying an
/// origin point, and then two other points that, together with the
/// origin, define two axes for the plane. These axes do not have to be
/// orthogonal - so you can create a parallelogram. (The axes must not
/// be parallel.) The resolution of the plane (i.e., number of subdivisions) is
/// controlled by the ivars XResolution and YResolution.
///
/// By default, the plane is centered at the origin and perpendicular to the
/// z-axis, with width and height of length 1 and resolutions set to 1.
///
/// There are three convenience methods that allow you to easily move the
/// plane.  The first, SetNormal(), allows you to specify the plane
/// normal. The effect of this method is to rotate the plane around the center
/// of the plane, aligning the plane normal with the specified normal. The
/// rotation is about the axis defined by the cross product of the current
/// normal with the new normal. The second, SetCenter(), translates the center
/// of the plane to the specified center point. The third method, Push(),
/// allows you to translate the plane along the plane normal by the distance
/// specified. (Negative Push values translate the plane in the negative
/// normal direction.)  Note that the SetNormal(), SetCenter() and Push()
/// methods modify the Origin, Point1, and/or Point2 instance variables.
///
/// @warning
/// The normal to the plane will point in the direction of the cross product
/// of the first axis (Origin->Point1) with the second (Origin->Point2). This
/// also affects the normals to the generated polygons.
#[allow(non_camel_case_types)]
pub struct vtkPlaneSource(*mut core::ffi::c_void);
impl vtkPlaneSource {
    /// Creates a new [vtkPlaneSource] via `vtkPlaneSource::New()`
    #[doc(alias = "vtkPlaneSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPlaneSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPlaneSource_new() })
    }
}
impl std::default::Default for vtkPlaneSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPlaneSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPlaneSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPlaneSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPlaneSource_create_drop() {
    let obj = vtkPlaneSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// produce polygonal Platonic solids
///
///
/// vtkPlatonicSolidSource can generate each of the five Platonic solids:
/// tetrahedron, cube, octahedron, icosahedron, and dodecahedron. Each of the
/// solids is placed inside a sphere centered at the origin with radius 1.0.
/// To use this class, simply specify the solid to create. Note that this
/// source object creates cell scalars that are (integral value) face numbers.
#[allow(non_camel_case_types)]
pub struct vtkPlatonicSolidSource(*mut core::ffi::c_void);
impl vtkPlatonicSolidSource {
    /// Creates a new [vtkPlatonicSolidSource] via `vtkPlatonicSolidSource::New()`
    #[doc(alias = "vtkPlatonicSolidSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPlatonicSolidSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPlatonicSolidSource_new() })
    }
}
impl std::default::Default for vtkPlatonicSolidSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPlatonicSolidSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPlatonicSolidSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPlatonicSolidSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPlatonicSolidSource_create_drop() {
    let obj = vtkPlatonicSolidSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// handle source used to represent points.
///
///
/// vtkPointHandleSource is deriving vtkHandleSource interface.
/// This handle represents a point with its shape being a sphere.
/// Its center and radius can be modified.
/// If the point is also parametered by any direction, it is then
/// represented as a cone pointing in this direction.
#[allow(non_camel_case_types)]
pub struct vtkPointHandleSource(*mut core::ffi::c_void);
impl vtkPointHandleSource {
    /// Creates a new [vtkPointHandleSource] via `vtkPointHandleSource::New()`
    #[doc(alias = "vtkPointHandleSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointHandleSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPointHandleSource_new() })
    }
}
impl std::default::Default for vtkPointHandleSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointHandleSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointHandleSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointHandleSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointHandleSource_create_drop() {
    let obj = vtkPointHandleSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a random cloud of points
///
///
/// vtkPointSource is a source object that creates a user-specified number
/// of points within a specified radius about a specified center point.
/// By default location of the points is random within the sphere. It is
/// also possible to generate random points only on the surface of the
/// sphere. The output PolyData has the specified number of points and
/// 1 cell - a vtkPolyVertex containing all of the points.
#[allow(non_camel_case_types)]
pub struct vtkPointSource(*mut core::ffi::c_void);
impl vtkPointSource {
    /// Creates a new [vtkPointSource] via `vtkPointSource::New()`
    #[doc(alias = "vtkPointSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPointSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPointSource_new() })
    }
}
impl std::default::Default for vtkPointSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPointSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPointSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPointSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPointSource_create_drop() {
    let obj = vtkPointSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a poly line from a list of input points
///
///
/// vtkPolyLineSource is a source object that creates a poly line from
/// user-specified points. The output is a vtkPolyLine.
#[allow(non_camel_case_types)]
pub struct vtkPolyLineSource(*mut core::ffi::c_void);
impl vtkPolyLineSource {
    /// Creates a new [vtkPolyLineSource] via `vtkPolyLineSource::New()`
    #[doc(alias = "vtkPolyLineSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyLineSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPolyLineSource_new() })
    }
}
impl std::default::Default for vtkPolyLineSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyLineSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyLineSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyLineSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyLineSource_create_drop() {
    let obj = vtkPolyLineSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create points from a list of input points
///
///
/// vtkPolyPointSource is a source object that creates a vert from
/// user-specified points. The output is a vtkPolyData.
#[allow(non_camel_case_types)]
pub struct vtkPolyPointSource(*mut core::ffi::c_void);
impl vtkPolyPointSource {
    /// Creates a new [vtkPolyPointSource] via `vtkPolyPointSource::New()`
    #[doc(alias = "vtkPolyPointSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPolyPointSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPolyPointSource_new() })
    }
}
impl std::default::Default for vtkPolyPointSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPolyPointSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPolyPointSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPolyPointSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPolyPointSource_create_drop() {
    let obj = vtkPolyPointSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// generate source data object via a user-specified function
///
///
/// vtkProgrammableDataObjectSource is a source object that is programmable by
/// the user. The output of the filter is a data object (vtkDataObject) which
/// represents data via an instance of field data. To use this object, you
/// must specify a function that creates the output.
///
/// Example use of this filter includes reading tabular data and encoding it
/// as vtkFieldData. You can then use filters like vtkDataObjectToDataSetFilter
/// to convert the data object to a dataset and then visualize it.  Another
/// important use of this class is that it allows users of interpreters (e.g.,
/// Java) the ability to write source objects without having to
/// recompile C++ code or generate new libraries.
///
/// @sa
/// vtkProgrammableFilter vtkProgrammableAttributeDataFilter
/// vtkProgrammableSource vtkDataObjectToDataSetFilter
#[allow(non_camel_case_types)]
pub struct vtkProgrammableDataObjectSource(*mut core::ffi::c_void);
impl vtkProgrammableDataObjectSource {
    /// Creates a new [vtkProgrammableDataObjectSource] via `vtkProgrammableDataObjectSource::New()`
    #[doc(alias = "vtkProgrammableDataObjectSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkProgrammableDataObjectSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkProgrammableDataObjectSource_new() })
    }
}
impl std::default::Default for vtkProgrammableDataObjectSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkProgrammableDataObjectSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkProgrammableDataObjectSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkProgrammableDataObjectSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkProgrammableDataObjectSource_create_drop() {
    let obj = vtkProgrammableDataObjectSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// generate source dataset via a user-specified function
///
///
/// vtkProgrammableSource is a source object that is programmable by the
/// user. To use this object, you must specify a function that creates the
/// output.  It is possible to generate an output dataset of any (concrete)
/// type; it is up to the function to properly initialize and define the
/// output. Typically, you use one of the methods to get a concrete output
/// type (e.g., GetPolyDataOutput() or GetStructuredPointsOutput()), and
/// then manipulate the output in the user-specified function.
///
/// Example use of this include writing a function to read a data file or
/// interface to another system. (You might want to do this in favor of
/// deriving a new class.) Another important use of this class is that it
/// allows users of interpreters (e.g., Java) the ability to write
/// source objects without having to recompile C++ code or generate new
/// libraries.
/// @sa
/// vtkProgrammableFilter vtkProgrammableAttributeDataFilter
/// vtkProgrammableDataObjectSource
#[allow(non_camel_case_types)]
pub struct vtkProgrammableSource(*mut core::ffi::c_void);
impl vtkProgrammableSource {
    /// Creates a new [vtkProgrammableSource] via `vtkProgrammableSource::New()`
    #[doc(alias = "vtkProgrammableSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkProgrammableSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkProgrammableSource_new() })
    }
}
impl std::default::Default for vtkProgrammableSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkProgrammableSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkProgrammableSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkProgrammableSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkProgrammableSource_create_drop() {
    let obj = vtkProgrammableSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Builds a randomized but reproducible vtkHyperTreeGrid.
///
#[allow(non_camel_case_types)]
pub struct vtkRandomHyperTreeGridSource(*mut core::ffi::c_void);
impl vtkRandomHyperTreeGridSource {
    /// Creates a new [vtkRandomHyperTreeGridSource] via `vtkRandomHyperTreeGridSource::New()`
    #[doc(alias = "vtkRandomHyperTreeGridSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRandomHyperTreeGridSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkRandomHyperTreeGridSource_new() })
    }
}
impl std::default::Default for vtkRandomHyperTreeGridSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRandomHyperTreeGridSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRandomHyperTreeGridSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRandomHyperTreeGridSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRandomHyperTreeGridSource_create_drop() {
    let obj = vtkRandomHyperTreeGridSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a rectangular button
///
///
/// vtkRectangularButtonSource creates a rectangular shaped button with
/// texture coordinates suitable for application of a texture map. This
/// provides a way to make nice looking 3D buttons. The buttons are
/// represented as vtkPolyData that includes texture coordinates and
/// normals. The button lies in the x-y plane.
///
/// To use this class you must define its width, height and length. These
/// measurements are all taken with respect to the shoulder of the button.
/// The shoulder is defined as follows. Imagine a box sitting on the floor.
/// The distance from the floor to the top of the box is the depth; the other
/// directions are the length (x-direction) and height (y-direction). In
/// this particular widget the box can have a smaller bottom than top. The
/// ratio in size between bottom and top is called the box ratio (by
/// default=1.0). The ratio of the texture region to the shoulder region
/// is the texture ratio. And finally the texture region may be out of plane
/// compared to the shoulder. The texture height ratio controls this.
///
/// @sa
/// vtkButtonSource vtkEllipticalButtonSource
///
/// @warning
/// The button is defined in the x-y plane. Use vtkTransformPolyDataFilter
/// or vtkGlyph3D to orient the button in a different direction.
#[allow(non_camel_case_types)]
pub struct vtkRectangularButtonSource(*mut core::ffi::c_void);
impl vtkRectangularButtonSource {
    /// Creates a new [vtkRectangularButtonSource] via `vtkRectangularButtonSource::New()`
    #[doc(alias = "vtkRectangularButtonSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRectangularButtonSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkRectangularButtonSource_new() })
    }
}
impl std::default::Default for vtkRectangularButtonSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRectangularButtonSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRectangularButtonSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRectangularButtonSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRectangularButtonSource_create_drop() {
    let obj = vtkRectangularButtonSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a regular, n-sided polygon and/or polyline
///
///
/// vtkRegularPolygonSource is a source object that creates a single n-sided polygon and/or
/// polyline. The polygon is centered at a specified point, orthogonal to
/// a specified normal, and with a circumscribing radius set by the user. The user can
/// also specify the number of sides of the polygon ranging from [3,N].
///
/// This object can be used for seeding streamlines or defining regions for clipping/cutting.
#[allow(non_camel_case_types)]
pub struct vtkRegularPolygonSource(*mut core::ffi::c_void);
impl vtkRegularPolygonSource {
    /// Creates a new [vtkRegularPolygonSource] via `vtkRegularPolygonSource::New()`
    #[doc(alias = "vtkRegularPolygonSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkRegularPolygonSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkRegularPolygonSource_new() })
    }
}
impl std::default::Default for vtkRegularPolygonSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkRegularPolygonSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkRegularPolygonSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkRegularPolygonSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkRegularPolygonSource_create_drop() {
    let obj = vtkRegularPolygonSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Generate selection from given set of ids
///
/// vtkSelectionSource generates a vtkSelection from a set of
/// (piece id, cell id) pairs. It will only generate the selection values
/// that match UPDATE_PIECE_NUMBER (i.e. piece == UPDATE_PIECE_NUMBER).
///
/// User-supplied, application-specific selections (with a ContentType of
/// vtkSelectionNode::USER) are not supported.
#[allow(non_camel_case_types)]
pub struct vtkSelectionSource(*mut core::ffi::c_void);
impl vtkSelectionSource {
    /// Creates a new [vtkSelectionSource] via `vtkSelectionSource::New()`
    #[doc(alias = "vtkSelectionSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSelectionSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSelectionSource_new() })
    }
}
impl std::default::Default for vtkSelectionSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSelectionSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSelectionSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSelectionSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSelectionSource_create_drop() {
    let obj = vtkSelectionSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a polygonal sphere centered at the origin
///
///
/// vtkSphereSource creates a sphere (represented by polygons) of specified
/// radius centered at the origin. The resolution (polygonal discretization)
/// in both the latitude (phi) and longitude (theta) directions can be
/// specified. It also is possible to create partial spheres by specifying
/// maximum phi and theta angles. By default, the surface tessellation of
/// the sphere uses triangles; however you can set LatLongTessellation to
/// produce a tessellation using quadrilaterals.
///
/// @warning
/// Resolution means the number of latitude or longitude lines for a complete
/// sphere. If you create partial spheres the number of latitude/longitude
/// lines may be off by one.
#[allow(non_camel_case_types)]
pub struct vtkSphereSource(*mut core::ffi::c_void);
impl vtkSphereSource {
    /// Creates a new [vtkSphereSource] via `vtkSphereSource::New()`
    #[doc(alias = "vtkSphereSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSphereSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSphereSource_new() })
    }
}
impl std::default::Default for vtkSphereSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSphereSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSphereSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSphereSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSphereSource_create_drop() {
    let obj = vtkSphereSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a polygonal superquadric centered
///
/// at the origin
///
/// vtkSuperquadricSource creates a superquadric (represented by polygons) of
/// specified size centered at the origin. The alignment of the axis of the
/// superquadric along one of the global axes can be specified. The resolution
/// (polygonal discretization)
/// in both the latitude (phi) and longitude (theta) directions can be
/// specified. Roundness parameters (PhiRoundness and ThetaRoundness) control
/// the shape of the superquadric.  The Toroidal boolean controls whether
/// a toroidal superquadric is produced.  If so, the Thickness parameter
/// controls the thickness of the toroid:  0 is the thinnest allowable
/// toroid, and 1 has a minimum sized hole.  The Scale parameters allow
/// the superquadric to be scaled in x, y, and z (normal vectors are correctly
/// generated in any case).  The Size parameter controls size of the
/// superquadric.
///
/// This code is based on "Rigid physically based superquadrics", A. H. Barr,
/// in "Graphics Gems III", David Kirk, ed., Academic Press, 1992.
///
/// @warning
/// Resolution means the number of latitude or longitude lines for a complete
/// superquadric. The resolution parameters are rounded to the nearest 4
/// in phi and 8 in theta.
///
/// @warning
/// Texture coordinates are not equally distributed around all superquadrics.
///
/// @warning
/// The Size and Thickness parameters control coefficients of superquadric
/// generation, and may do not exactly describe the size of the superquadric.
#[allow(non_camel_case_types)]
pub struct vtkSuperquadricSource(*mut core::ffi::c_void);
impl vtkSuperquadricSource {
    /// Creates a new [vtkSuperquadricSource] via `vtkSuperquadricSource::New()`
    #[doc(alias = "vtkSuperquadricSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSuperquadricSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSuperquadricSource_new() })
    }
}
impl std::default::Default for vtkSuperquadricSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSuperquadricSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSuperquadricSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSuperquadricSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSuperquadricSource_create_drop() {
    let obj = vtkSuperquadricSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Create a polygonal representation of a box
///
/// with a given level of subdivision.
///
/// vtkTessellatedBoxSource creates a axis-aligned box defined by its bounds
/// and a level of subdivision. Connectivity is strong: points of the vertices
/// and inside the edges are shared between faces. In other words, faces are
/// connected. Each face looks like a grid of quads, each quad is composed of
/// 2 triangles.
/// Given a level of subdivision `l', each edge has `l'+2 points, `l' of them
/// are internal edge points, the 2 other ones are the vertices.
/// Each face has a total of (`l'+2)*(`l'+2) points, 4 of them are vertices,
/// 4*`l' are internal edge points, it remains `l'^2 internal face points.
///
/// This source only generate geometry, no DataArrays like normals or texture
/// coordinates.
#[allow(non_camel_case_types)]
pub struct vtkTessellatedBoxSource(*mut core::ffi::c_void);
impl vtkTessellatedBoxSource {
    /// Creates a new [vtkTessellatedBoxSource] via `vtkTessellatedBoxSource::New()`
    #[doc(alias = "vtkTessellatedBoxSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTessellatedBoxSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTessellatedBoxSource_new() })
    }
}
impl std::default::Default for vtkTessellatedBoxSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTessellatedBoxSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTessellatedBoxSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTessellatedBoxSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTessellatedBoxSource_create_drop() {
    let obj = vtkTessellatedBoxSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create polygonal text
///
///
/// vtkTextSource converts a text string into polygons.  This way you can
/// insert text into your renderings. It uses the 9x15 font from X Windows.
/// You can specify if you want the background to be drawn or not. The
/// characters are formed by scan converting the raster font into
/// quadrilaterals. Colors are assigned to the letters using scalar data.
/// To set the color of the characters with the source's actor property, set
/// BackingOff on the text source and ScalarVisibilityOff on the associated
/// vtkPolyDataMapper. Then, the color can be set using the associated actor's
/// property.
///
/// vtkVectorText generates higher quality polygonal representations of
/// characters.
///
/// @sa
/// vtkVectorText
#[allow(non_camel_case_types)]
pub struct vtkTextSource(*mut core::ffi::c_void);
impl vtkTextSource {
    /// Creates a new [vtkTextSource] via `vtkTextSource::New()`
    #[doc(alias = "vtkTextSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTextSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTextSource_new() })
    }
}
impl std::default::Default for vtkTextSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTextSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTextSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTextSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTextSource_create_drop() {
    let obj = vtkTextSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// create a sphere centered at the origin
///
///
/// vtkTexturedSphereSource creates a polygonal sphere of specified radius
/// centered at the origin. The resolution (polygonal discretization) in both
/// the latitude (phi) and longitude (theta) directions can be specified.
/// It also is possible to create partial sphere by specifying maximum phi and
/// theta angles.
#[allow(non_camel_case_types)]
pub struct vtkTexturedSphereSource(*mut core::ffi::c_void);
impl vtkTexturedSphereSource {
    /// Creates a new [vtkTexturedSphereSource] via `vtkTexturedSphereSource::New()`
    #[doc(alias = "vtkTexturedSphereSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTexturedSphereSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTexturedSphereSource_new() })
    }
}
impl std::default::Default for vtkTexturedSphereSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTexturedSphereSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTexturedSphereSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTexturedSphereSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTexturedSphereSource_create_drop() {
    let obj = vtkTexturedSphereSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// Create a synthetic grid of uniform hypertrees.
///
///
/// This class uses input parameters, most notably a string descriptor,
/// to generate a vtkHyperTreeGrid instance representing the corresponding
/// tree-based AMR grid with uniform root cell sizes along each axis.
///
/// @sa
/// vtkHyperTreeGridSource vtkUniformHyperTreeGrid
///
/// @par Thanks:
/// This class was written by Philippe Pebay, NexGen Analytics 2017
/// This work was supported by Commissariat a l'Energie Atomique (CEA/DIF)
/// CEA, DAM, DIF, F-91297 Arpajon, France.
#[allow(non_camel_case_types)]
pub struct vtkUniformHyperTreeGridSource(*mut core::ffi::c_void);
impl vtkUniformHyperTreeGridSource {
    /// Creates a new [vtkUniformHyperTreeGridSource] via `vtkUniformHyperTreeGridSource::New()`
    #[doc(alias = "vtkUniformHyperTreeGridSource")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkUniformHyperTreeGridSource_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkUniformHyperTreeGridSource_new() })
    }
}
impl std::default::Default for vtkUniformHyperTreeGridSource {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkUniformHyperTreeGridSource {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkUniformHyperTreeGridSource_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkUniformHyperTreeGridSource_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkUniformHyperTreeGridSource_create_drop() {
    let obj = vtkUniformHyperTreeGridSource::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
