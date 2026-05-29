pub trait VtkAbstractTransform {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn transform_points(
        &mut self,
        inPts: *mut core::ffi::c_void,
        outPts: *mut core::ffi::c_void,
    ) -> ();
    fn get_inverse(&mut self) -> *mut core::ffi::c_void;
    fn set_inverse(&mut self, transform: *mut core::ffi::c_void) -> ();
    fn inverse(&mut self) -> ();
    fn deep_copy(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn update(&mut self) -> ();
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
    fn circuit_check(&mut self, transform: *mut core::ffi::c_void) -> core::ffi::c_int;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn un_register(&mut self, O: *mut core::ffi::c_void) -> ();
}
pub trait VtkCylindricalTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkGeneralTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn identity(&mut self) -> ();
    fn inverse(&mut self) -> ();
    fn translate(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn rotate_wxyz(
        &mut self,
        angle: core::ffi::c_double,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn rotate_x(&mut self, angle: core::ffi::c_double) -> ();
    fn rotate_y(&mut self, angle: core::ffi::c_double) -> ();
    fn rotate_z(&mut self, angle: core::ffi::c_double) -> ();
    fn scale(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn concatenate(&mut self, matrix: *mut core::ffi::c_void) -> ();
    fn pre_multiply(&mut self) -> ();
    fn post_multiply(&mut self) -> ();
    fn get_number_of_concatenated_transforms(&mut self) -> core::ffi::c_int;
    fn get_concatenated_transform(
        &mut self,
        i: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_input(&mut self, input: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_inverse_flag(&mut self) -> core::ffi::c_int;
    fn push(&mut self) -> ();
    fn pop(&mut self) -> ();
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkHomogeneousTransform {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn get_matrix(&mut self, m: *mut core::ffi::c_void) -> ();
    fn get_homogeneous_inverse(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkIdentityTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn inverse(&mut self) -> ();
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkLandmarkTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_source_landmarks(&mut self, source: *mut core::ffi::c_void) -> ();
    fn set_target_landmarks(&mut self, target: *mut core::ffi::c_void) -> ();
    fn get_source_landmarks(&mut self) -> *mut core::ffi::c_void;
    fn get_target_landmarks(&mut self) -> *mut core::ffi::c_void;
    fn set_mode(&mut self, _arg: core::ffi::c_int) -> ();
    fn set_mode_to_rigid_body(&mut self) -> ();
    fn set_mode_to_similarity(&mut self) -> ();
    fn set_mode_to_affine(&mut self) -> ();
    fn get_mode(&mut self) -> core::ffi::c_int;
    fn get_mode_as_string(&mut self) -> &str;
    fn inverse(&mut self) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkLinearTransform {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn transform_normals(
        &mut self,
        inNms: *mut core::ffi::c_void,
        outNms: *mut core::ffi::c_void,
    ) -> ();
    fn transform_vectors(
        &mut self,
        inVrs: *mut core::ffi::c_void,
        outVrs: *mut core::ffi::c_void,
    ) -> ();
    fn get_linear_inverse(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkMatrixToHomogeneousTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_input(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn inverse(&mut self) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkMatrixToLinearTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn set_input(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn inverse(&mut self) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkPerspectiveTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn identity(&mut self) -> ();
    fn inverse(&mut self) -> ();
    fn adjust_viewport(
        &mut self,
        oldXMin: core::ffi::c_double,
        oldXMax: core::ffi::c_double,
        oldYMin: core::ffi::c_double,
        oldYMax: core::ffi::c_double,
        newXMin: core::ffi::c_double,
        newXMax: core::ffi::c_double,
        newYMin: core::ffi::c_double,
        newYMax: core::ffi::c_double,
    ) -> ();
    fn adjust_z_buffer(
        &mut self,
        oldNearZ: core::ffi::c_double,
        oldFarZ: core::ffi::c_double,
        newNearZ: core::ffi::c_double,
        newFarZ: core::ffi::c_double,
    ) -> ();
    fn ortho(
        &mut self,
        xmin: core::ffi::c_double,
        xmax: core::ffi::c_double,
        ymin: core::ffi::c_double,
        ymax: core::ffi::c_double,
        znear: core::ffi::c_double,
        zfar: core::ffi::c_double,
    ) -> ();
    fn frustum(
        &mut self,
        xmin: core::ffi::c_double,
        xmax: core::ffi::c_double,
        ymin: core::ffi::c_double,
        ymax: core::ffi::c_double,
        znear: core::ffi::c_double,
        zfar: core::ffi::c_double,
    ) -> ();
    fn perspective(
        &mut self,
        angle: core::ffi::c_double,
        aspect: core::ffi::c_double,
        znear: core::ffi::c_double,
        zfar: core::ffi::c_double,
    ) -> ();
    fn shear(
        &mut self,
        dxdz: core::ffi::c_double,
        dydz: core::ffi::c_double,
        zplane: core::ffi::c_double,
    ) -> ();
    fn stereo(
        &mut self,
        angle: core::ffi::c_double,
        focaldistance: core::ffi::c_double,
    ) -> ();
    fn setup_camera(
        &mut self,
        p0: core::ffi::c_double,
        p1: core::ffi::c_double,
        p2: core::ffi::c_double,
        fp0: core::ffi::c_double,
        fp1: core::ffi::c_double,
        fp2: core::ffi::c_double,
        vup0: core::ffi::c_double,
        vup1: core::ffi::c_double,
        vup2: core::ffi::c_double,
    ) -> ();
    fn translate(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn rotate_wxyz(
        &mut self,
        angle: core::ffi::c_double,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn rotate_x(&mut self, angle: core::ffi::c_double) -> ();
    fn rotate_y(&mut self, angle: core::ffi::c_double) -> ();
    fn rotate_z(&mut self, angle: core::ffi::c_double) -> ();
    fn scale(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn set_matrix(&mut self, matrix: *mut core::ffi::c_void) -> ();
    fn concatenate(&mut self, matrix: *mut core::ffi::c_void) -> ();
    fn pre_multiply(&mut self) -> ();
    fn post_multiply(&mut self) -> ();
    fn get_number_of_concatenated_transforms(&mut self) -> core::ffi::c_int;
    fn get_concatenated_transform(
        &mut self,
        i: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn set_input(&mut self, input: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_inverse_flag(&mut self) -> core::ffi::c_int;
    fn push(&mut self) -> ();
    fn pop(&mut self) -> ();
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkSphericalTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkThinPlateSplineTransform {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn get_sigma(&mut self) -> core::ffi::c_double;
    fn set_sigma(&mut self, _arg: core::ffi::c_double) -> ();
    fn set_basis(&mut self, basis: core::ffi::c_int) -> ();
    fn get_basis(&mut self) -> core::ffi::c_int;
    fn set_basis_to_r(&mut self) -> ();
    fn set_basis_to_r_2_log_r(&mut self) -> ();
    fn get_basis_as_string(&mut self) -> &str;
    fn set_basis_function(&mut self, U: *mut core::ffi::c_void) -> ();
    fn set_basis_derivative(&mut self, dUdr: *mut core::ffi::c_void) -> ();
    fn set_source_landmarks(&mut self, source: *mut core::ffi::c_void) -> ();
    fn get_source_landmarks(&mut self) -> *mut core::ffi::c_void;
    fn set_target_landmarks(&mut self, target: *mut core::ffi::c_void) -> ();
    fn get_target_landmarks(&mut self) -> *mut core::ffi::c_void;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
    fn get_regularize_bulk_transform(&mut self) -> bool;
    fn set_regularize_bulk_transform(&mut self, _arg: bool) -> ();
    fn regularize_bulk_transform_on(&mut self) -> ();
    fn regularize_bulk_transform_off(&mut self) -> ();
}
pub trait VtkTransform {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn identity(&mut self) -> ();
    fn inverse(&mut self) -> ();
    fn translate(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn rotate_wxyz(
        &mut self,
        angle: core::ffi::c_double,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn rotate_x(&mut self, angle: core::ffi::c_double) -> ();
    fn rotate_y(&mut self, angle: core::ffi::c_double) -> ();
    fn rotate_z(&mut self, angle: core::ffi::c_double) -> ();
    fn scale(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn set_matrix(&mut self, matrix: *mut core::ffi::c_void) -> ();
    fn concatenate(&mut self, matrix: *mut core::ffi::c_void) -> ();
    fn pre_multiply(&mut self) -> ();
    fn post_multiply(&mut self) -> ();
    fn get_number_of_concatenated_transforms(&mut self) -> core::ffi::c_int;
    fn get_concatenated_transform(
        &mut self,
        i: core::ffi::c_int,
    ) -> *mut core::ffi::c_void;
    fn get_inverse(&mut self, inverse: *mut core::ffi::c_void) -> ();
    fn get_transpose(&mut self, transpose: *mut core::ffi::c_void) -> ();
    fn set_input(&mut self, input: *mut core::ffi::c_void) -> ();
    fn get_input(&mut self) -> *mut core::ffi::c_void;
    fn get_inverse_flag(&mut self) -> core::ffi::c_int;
    fn push(&mut self) -> ();
    fn pop(&mut self) -> ();
    fn make_transform(&mut self) -> *mut core::ffi::c_void;
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkTransform2D {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn identity(&mut self) -> ();
    fn inverse(&mut self) -> ();
    fn translate(&mut self, x: core::ffi::c_double, y: core::ffi::c_double) -> ();
    fn rotate(&mut self, angle: core::ffi::c_double) -> ();
    fn scale(&mut self, x: core::ffi::c_double, y: core::ffi::c_double) -> ();
    fn set_matrix(&mut self, matrix: *mut core::ffi::c_void) -> ();
    fn get_matrix(&mut self) -> *mut core::ffi::c_void;
    fn get_inverse(&mut self, inverse: *mut core::ffi::c_void) -> ();
    fn get_transpose(&mut self, transpose: *mut core::ffi::c_void) -> ();
    fn get_m_time(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkTransformCollection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn add_item(&mut self, p0: *mut core::ffi::c_void) -> ();
    fn get_next_item(&mut self) -> *mut core::ffi::c_void;
}
pub trait VtkTransformConcatenation {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn delete(&mut self) -> ();
    fn concatenate(&mut self, transform: *mut core::ffi::c_void) -> ();
    fn set_pre_multiply_flag(&mut self, flag: core::ffi::c_int) -> ();
    fn get_pre_multiply_flag(&mut self) -> core::ffi::c_int;
    fn translate(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn rotate(
        &mut self,
        angle: core::ffi::c_double,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn scale(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> ();
    fn inverse(&mut self) -> ();
    fn get_inverse_flag(&mut self) -> core::ffi::c_int;
    fn identity(&mut self) -> ();
    fn deep_copy(&mut self, transform: *mut core::ffi::c_void) -> ();
    fn get_number_of_transforms(&mut self) -> core::ffi::c_int;
    fn get_number_of_pre_transforms(&mut self) -> core::ffi::c_int;
    fn get_number_of_post_transforms(&mut self) -> core::ffi::c_int;
    fn get_transform(&mut self, i: core::ffi::c_int) -> *mut core::ffi::c_void;
    fn get_max_m_time(&mut self) -> core::ffi::c_ulong;
}
pub trait VtkTransformConcatenationStack {
    fn new(&mut self) -> *mut core::ffi::c_void;
    fn delete(&mut self) -> ();
    fn deep_copy(&mut self, stack: *mut core::ffi::c_void) -> ();
}
pub trait VtkTransformPair {
    fn swap_forward_inverse(&mut self) -> ();
}
pub trait VtkWarpTransform {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn new_instance(&mut self) -> *mut core::ffi::c_void;
    fn inverse(&mut self) -> ();
    fn get_inverse_flag(&mut self) -> core::ffi::c_int;
    fn set_inverse_tolerance(&mut self, _arg: core::ffi::c_double) -> ();
    fn get_inverse_tolerance(&mut self) -> core::ffi::c_double;
    fn set_inverse_iterations(&mut self, _arg: core::ffi::c_int) -> ();
    fn get_inverse_iterations(&mut self) -> core::ffi::c_int;
}
impl VtkCylindricalTransform for vtkCylindricalTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cylindrical_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cylindrical_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cylindrical_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cylindrical_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cylindrical_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cylindrical_transform_new_instance(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_cylindrical_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_cylindrical_transform_make_transform(self.0) }
    }
}
impl VtkGeneralTransform for vtkGeneralTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_general_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_general_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_general_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_general_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_general_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_general_transform_new_instance(self.0) }
    }
    fn identity(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_identity(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_general_transform_identity(self.0) }
    }
    fn inverse(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_inverse(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_general_transform_inverse(self.0) }
    }
    fn translate(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_translate(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_general_transform_translate(self.0, x, y, z) }
    }
    fn rotate_wxyz(
        &mut self,
        angle: core::ffi::c_double,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_rotate_wxyz(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_general_transform_rotate_wxyz(self.0, angle, x, y, z) }
    }
    fn rotate_x(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_rotate_x(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_general_transform_rotate_x(self.0, angle) }
    }
    fn rotate_y(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_rotate_y(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_general_transform_rotate_y(self.0, angle) }
    }
    fn rotate_z(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_rotate_z(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_general_transform_rotate_z(self.0, angle) }
    }
    fn scale(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_scale(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_general_transform_scale(self.0, x, y, z) }
    }
    fn concatenate(&mut self, matrix: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_concatenate(
                sself: *mut core::ffi::c_void,
                matrix: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_general_transform_concatenate(self.0, matrix) }
    }
    fn pre_multiply(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_pre_multiply(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_general_transform_pre_multiply(self.0) }
    }
    fn post_multiply(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_post_multiply(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_general_transform_post_multiply(self.0) }
    }
    fn get_number_of_concatenated_transforms(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_general_transform_get_number_of_concatenated_transforms(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_general_transform_get_number_of_concatenated_transforms(self.0) }
    }
    fn get_concatenated_transform(
        &mut self,
        i: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_general_transform_get_concatenated_transform(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_general_transform_get_concatenated_transform(self.0, i) }
    }
    fn set_input(&mut self, input: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_set_input(
                sself: *mut core::ffi::c_void,
                input: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_general_transform_set_input(self.0, input) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_general_transform_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_general_transform_get_input(self.0) }
    }
    fn get_inverse_flag(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_general_transform_get_inverse_flag(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_general_transform_get_inverse_flag(self.0) }
    }
    fn push(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_push(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_general_transform_push(self.0) }
    }
    fn pop(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_general_transform_pop(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_general_transform_pop(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_general_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_general_transform_make_transform(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_general_transform_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_general_transform_get_m_time(self.0) }
    }
}
impl VtkIdentityTransform for vtkIdentityTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_identity_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_identity_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_identity_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_identity_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_identity_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_identity_transform_new_instance(self.0) }
    }
    fn inverse(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_identity_transform_inverse(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_identity_transform_inverse(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_identity_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_identity_transform_make_transform(self.0) }
    }
}
impl VtkLandmarkTransform for vtkLandmarkTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_landmark_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_landmark_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_landmark_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_landmark_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_landmark_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_landmark_transform_new_instance(self.0) }
    }
    fn set_source_landmarks(&mut self, source: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_landmark_transform_set_source_landmarks(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_landmark_transform_set_source_landmarks(self.0, source) }
    }
    fn set_target_landmarks(&mut self, target: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_landmark_transform_set_target_landmarks(
                sself: *mut core::ffi::c_void,
                target: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_landmark_transform_set_target_landmarks(self.0, target) }
    }
    fn get_source_landmarks(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_landmark_transform_get_source_landmarks(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_landmark_transform_get_source_landmarks(self.0) }
    }
    fn get_target_landmarks(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_landmark_transform_get_target_landmarks(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_landmark_transform_get_target_landmarks(self.0) }
    }
    fn set_mode(&mut self, _arg: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_landmark_transform_set_mode(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_int,
            );
        }
        unsafe { vtk_landmark_transform_set_mode(self.0, _arg) }
    }
    fn set_mode_to_rigid_body(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_landmark_transform_set_mode_to_rigid_body(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_landmark_transform_set_mode_to_rigid_body(self.0) }
    }
    fn set_mode_to_similarity(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_landmark_transform_set_mode_to_similarity(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_landmark_transform_set_mode_to_similarity(self.0) }
    }
    fn set_mode_to_affine(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_landmark_transform_set_mode_to_affine(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_landmark_transform_set_mode_to_affine(self.0) }
    }
    fn get_mode(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_landmark_transform_get_mode(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_landmark_transform_get_mode(self.0) }
    }
    fn get_mode_as_string(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_landmark_transform_get_mode_as_string(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_landmark_transform_get_mode_as_string(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn inverse(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_landmark_transform_inverse(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_landmark_transform_inverse(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_landmark_transform_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_landmark_transform_get_m_time(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_landmark_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_landmark_transform_make_transform(self.0) }
    }
}
impl VtkMatrixToHomogeneousTransform for vtkMatrixToHomogeneousTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_homogeneous_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_homogeneous_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_homogeneous_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_homogeneous_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_homogeneous_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_homogeneous_transform_new_instance(self.0) }
    }
    fn set_input(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_matrix_to_homogeneous_transform_set_input(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_matrix_to_homogeneous_transform_set_input(self.0, p0) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_homogeneous_transform_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_homogeneous_transform_get_input(self.0) }
    }
    fn inverse(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_matrix_to_homogeneous_transform_inverse(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_matrix_to_homogeneous_transform_inverse(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_matrix_to_homogeneous_transform_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_matrix_to_homogeneous_transform_get_m_time(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_homogeneous_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_homogeneous_transform_make_transform(self.0) }
    }
}
impl VtkMatrixToLinearTransform for vtkMatrixToLinearTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_linear_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_linear_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_linear_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_linear_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_linear_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_linear_transform_new_instance(self.0) }
    }
    fn set_input(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_matrix_to_linear_transform_set_input(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_matrix_to_linear_transform_set_input(self.0, p0) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_linear_transform_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_linear_transform_get_input(self.0) }
    }
    fn inverse(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_matrix_to_linear_transform_inverse(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_matrix_to_linear_transform_inverse(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_matrix_to_linear_transform_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_matrix_to_linear_transform_get_m_time(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_matrix_to_linear_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_matrix_to_linear_transform_make_transform(self.0) }
    }
}
impl VtkPerspectiveTransform for vtkPerspectiveTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_perspective_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_perspective_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_perspective_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_perspective_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_perspective_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_perspective_transform_new_instance(self.0) }
    }
    fn identity(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_identity(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_perspective_transform_identity(self.0) }
    }
    fn inverse(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_inverse(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_perspective_transform_inverse(self.0) }
    }
    fn adjust_viewport(
        &mut self,
        oldXMin: core::ffi::c_double,
        oldXMax: core::ffi::c_double,
        oldYMin: core::ffi::c_double,
        oldYMax: core::ffi::c_double,
        newXMin: core::ffi::c_double,
        newXMax: core::ffi::c_double,
        newYMin: core::ffi::c_double,
        newYMax: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_adjust_viewport(
                sself: *mut core::ffi::c_void,
                oldXMin: core::ffi::c_double,
                oldXMax: core::ffi::c_double,
                oldYMin: core::ffi::c_double,
                oldYMax: core::ffi::c_double,
                newXMin: core::ffi::c_double,
                newXMax: core::ffi::c_double,
                newYMin: core::ffi::c_double,
                newYMax: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_perspective_transform_adjust_viewport(
                self.0,
                oldXMin,
                oldXMax,
                oldYMin,
                oldYMax,
                newXMin,
                newXMax,
                newYMin,
                newYMax,
            )
        }
    }
    fn adjust_z_buffer(
        &mut self,
        oldNearZ: core::ffi::c_double,
        oldFarZ: core::ffi::c_double,
        newNearZ: core::ffi::c_double,
        newFarZ: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_adjust_z_buffer(
                sself: *mut core::ffi::c_void,
                oldNearZ: core::ffi::c_double,
                oldFarZ: core::ffi::c_double,
                newNearZ: core::ffi::c_double,
                newFarZ: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_perspective_transform_adjust_z_buffer(
                self.0,
                oldNearZ,
                oldFarZ,
                newNearZ,
                newFarZ,
            )
        }
    }
    fn ortho(
        &mut self,
        xmin: core::ffi::c_double,
        xmax: core::ffi::c_double,
        ymin: core::ffi::c_double,
        ymax: core::ffi::c_double,
        znear: core::ffi::c_double,
        zfar: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_ortho(
                sself: *mut core::ffi::c_void,
                xmin: core::ffi::c_double,
                xmax: core::ffi::c_double,
                ymin: core::ffi::c_double,
                ymax: core::ffi::c_double,
                znear: core::ffi::c_double,
                zfar: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_perspective_transform_ortho(self.0, xmin, xmax, ymin, ymax, znear, zfar)
        }
    }
    fn frustum(
        &mut self,
        xmin: core::ffi::c_double,
        xmax: core::ffi::c_double,
        ymin: core::ffi::c_double,
        ymax: core::ffi::c_double,
        znear: core::ffi::c_double,
        zfar: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_frustum(
                sself: *mut core::ffi::c_void,
                xmin: core::ffi::c_double,
                xmax: core::ffi::c_double,
                ymin: core::ffi::c_double,
                ymax: core::ffi::c_double,
                znear: core::ffi::c_double,
                zfar: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_perspective_transform_frustum(
                self.0,
                xmin,
                xmax,
                ymin,
                ymax,
                znear,
                zfar,
            )
        }
    }
    fn perspective(
        &mut self,
        angle: core::ffi::c_double,
        aspect: core::ffi::c_double,
        znear: core::ffi::c_double,
        zfar: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_perspective(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
                aspect: core::ffi::c_double,
                znear: core::ffi::c_double,
                zfar: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_perspective_transform_perspective(self.0, angle, aspect, znear, zfar)
        }
    }
    fn shear(
        &mut self,
        dxdz: core::ffi::c_double,
        dydz: core::ffi::c_double,
        zplane: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_shear(
                sself: *mut core::ffi::c_void,
                dxdz: core::ffi::c_double,
                dydz: core::ffi::c_double,
                zplane: core::ffi::c_double,
            );
        }
        unsafe { vtk_perspective_transform_shear(self.0, dxdz, dydz, zplane) }
    }
    fn stereo(
        &mut self,
        angle: core::ffi::c_double,
        focaldistance: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_stereo(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
                focaldistance: core::ffi::c_double,
            );
        }
        unsafe { vtk_perspective_transform_stereo(self.0, angle, focaldistance) }
    }
    fn setup_camera(
        &mut self,
        p0: core::ffi::c_double,
        p1: core::ffi::c_double,
        p2: core::ffi::c_double,
        fp0: core::ffi::c_double,
        fp1: core::ffi::c_double,
        fp2: core::ffi::c_double,
        vup0: core::ffi::c_double,
        vup1: core::ffi::c_double,
        vup2: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_setup_camera(
                sself: *mut core::ffi::c_void,
                p0: core::ffi::c_double,
                p1: core::ffi::c_double,
                p2: core::ffi::c_double,
                fp0: core::ffi::c_double,
                fp1: core::ffi::c_double,
                fp2: core::ffi::c_double,
                vup0: core::ffi::c_double,
                vup1: core::ffi::c_double,
                vup2: core::ffi::c_double,
            );
        }
        unsafe {
            vtk_perspective_transform_setup_camera(
                self.0,
                p0,
                p1,
                p2,
                fp0,
                fp1,
                fp2,
                vup0,
                vup1,
                vup2,
            )
        }
    }
    fn translate(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_translate(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_perspective_transform_translate(self.0, x, y, z) }
    }
    fn rotate_wxyz(
        &mut self,
        angle: core::ffi::c_double,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_rotate_wxyz(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_perspective_transform_rotate_wxyz(self.0, angle, x, y, z) }
    }
    fn rotate_x(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_rotate_x(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_perspective_transform_rotate_x(self.0, angle) }
    }
    fn rotate_y(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_rotate_y(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_perspective_transform_rotate_y(self.0, angle) }
    }
    fn rotate_z(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_rotate_z(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_perspective_transform_rotate_z(self.0, angle) }
    }
    fn scale(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_scale(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_perspective_transform_scale(self.0, x, y, z) }
    }
    fn set_matrix(&mut self, matrix: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_set_matrix(
                sself: *mut core::ffi::c_void,
                matrix: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_perspective_transform_set_matrix(self.0, matrix) }
    }
    fn concatenate(&mut self, matrix: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_concatenate(
                sself: *mut core::ffi::c_void,
                matrix: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_perspective_transform_concatenate(self.0, matrix) }
    }
    fn pre_multiply(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_pre_multiply(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_perspective_transform_pre_multiply(self.0) }
    }
    fn post_multiply(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_post_multiply(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_perspective_transform_post_multiply(self.0) }
    }
    fn get_number_of_concatenated_transforms(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_perspective_transform_get_number_of_concatenated_transforms(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe {
            vtk_perspective_transform_get_number_of_concatenated_transforms(self.0)
        }
    }
    fn get_concatenated_transform(
        &mut self,
        i: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_perspective_transform_get_concatenated_transform(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_perspective_transform_get_concatenated_transform(self.0, i) }
    }
    fn set_input(&mut self, input: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_set_input(
                sself: *mut core::ffi::c_void,
                input: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_perspective_transform_set_input(self.0, input) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_perspective_transform_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_perspective_transform_get_input(self.0) }
    }
    fn get_inverse_flag(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_perspective_transform_get_inverse_flag(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_perspective_transform_get_inverse_flag(self.0) }
    }
    fn push(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_push(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_perspective_transform_push(self.0) }
    }
    fn pop(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_perspective_transform_pop(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_perspective_transform_pop(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_perspective_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_perspective_transform_make_transform(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_perspective_transform_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_perspective_transform_get_m_time(self.0) }
    }
}
impl VtkSphericalTransform for vtkSphericalTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_spherical_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_spherical_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_spherical_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_spherical_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_spherical_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_spherical_transform_new_instance(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_spherical_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_spherical_transform_make_transform(self.0) }
    }
}
impl VtkThinPlateSplineTransform for vtkThinPlateSplineTransform {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_thin_plate_spline_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_thin_plate_spline_transform_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_thin_plate_spline_transform_new(self.0) }
    }
    fn get_sigma(&mut self) -> core::ffi::c_double {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_get_sigma(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_double;
        }
        unsafe { vtk_thin_plate_spline_transform_get_sigma(self.0) }
    }
    fn set_sigma(&mut self, _arg: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_sigma(
                sself: *mut core::ffi::c_void,
                _arg: core::ffi::c_double,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_set_sigma(self.0, _arg) }
    }
    fn set_basis(&mut self, basis: core::ffi::c_int) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_basis(
                sself: *mut core::ffi::c_void,
                basis: core::ffi::c_int,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_set_basis(self.0, basis) }
    }
    fn get_basis(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_get_basis(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_thin_plate_spline_transform_get_basis(self.0) }
    }
    fn set_basis_to_r(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_basis_to_r(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_set_basis_to_r(self.0) }
    }
    fn set_basis_to_r_2_log_r(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_basis_to_r_2_log_r(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_set_basis_to_r_2_log_r(self.0) }
    }
    fn get_basis_as_string(&mut self) -> &str {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_get_basis_as_string(
                sself: *mut core::ffi::c_void,
            ) -> *const core::ffi::c_char;
        }
        let ptr = unsafe { vtk_thin_plate_spline_transform_get_basis_as_string(self.0) };
        if ptr.is_null() {
            return "";
        }
        unsafe { std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("") }
    }
    fn set_basis_function(&mut self, U: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_basis_function(
                sself: *mut core::ffi::c_void,
                U: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_set_basis_function(self.0, U) }
    }
    fn set_basis_derivative(&mut self, dUdr: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_basis_derivative(
                sself: *mut core::ffi::c_void,
                dUdr: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_set_basis_derivative(self.0, dUdr) }
    }
    fn set_source_landmarks(&mut self, source: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_source_landmarks(
                sself: *mut core::ffi::c_void,
                source: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_set_source_landmarks(self.0, source) }
    }
    fn get_source_landmarks(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_get_source_landmarks(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_thin_plate_spline_transform_get_source_landmarks(self.0) }
    }
    fn set_target_landmarks(&mut self, target: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_target_landmarks(
                sself: *mut core::ffi::c_void,
                target: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_set_target_landmarks(self.0, target) }
    }
    fn get_target_landmarks(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_get_target_landmarks(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_thin_plate_spline_transform_get_target_landmarks(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_thin_plate_spline_transform_get_m_time(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_thin_plate_spline_transform_make_transform(self.0) }
    }
    fn get_regularize_bulk_transform(&mut self) -> bool {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_get_regularize_bulk_transform(
                sself: *mut core::ffi::c_void,
            ) -> bool;
        }
        unsafe { vtk_thin_plate_spline_transform_get_regularize_bulk_transform(self.0) }
    }
    fn set_regularize_bulk_transform(&mut self, _arg: bool) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_set_regularize_bulk_transform(
                sself: *mut core::ffi::c_void,
                _arg: bool,
            );
        }
        unsafe {
            vtk_thin_plate_spline_transform_set_regularize_bulk_transform(self.0, _arg)
        }
    }
    fn regularize_bulk_transform_on(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_regularize_bulk_transform_on(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_regularize_bulk_transform_on(self.0) }
    }
    fn regularize_bulk_transform_off(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_thin_plate_spline_transform_regularize_bulk_transform_off(
                sself: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_thin_plate_spline_transform_regularize_bulk_transform_off(self.0) }
    }
}
impl VtkTransform for vtkTransform {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_new_instance(self.0) }
    }
    fn identity(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_transform_identity(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_transform_identity(self.0) }
    }
    fn inverse(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_transform_inverse(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_transform_inverse(self.0) }
    }
    fn translate(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_transform_translate(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_translate(self.0, x, y, z) }
    }
    fn rotate_wxyz(
        &mut self,
        angle: core::ffi::c_double,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_transform_rotate_wxyz(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_rotate_wxyz(self.0, angle, x, y, z) }
    }
    fn rotate_x(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_transform_rotate_x(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_rotate_x(self.0, angle) }
    }
    fn rotate_y(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_transform_rotate_y(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_rotate_y(self.0, angle) }
    }
    fn rotate_z(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_transform_rotate_z(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_rotate_z(self.0, angle) }
    }
    fn scale(
        &mut self,
        x: core::ffi::c_double,
        y: core::ffi::c_double,
        z: core::ffi::c_double,
    ) -> () {
        unsafe extern "C" {
            fn vtk_transform_scale(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
                z: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_scale(self.0, x, y, z) }
    }
    fn set_matrix(&mut self, matrix: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_set_matrix(
                sself: *mut core::ffi::c_void,
                matrix: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_set_matrix(self.0, matrix) }
    }
    fn concatenate(&mut self, matrix: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_concatenate(
                sself: *mut core::ffi::c_void,
                matrix: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_concatenate(self.0, matrix) }
    }
    fn pre_multiply(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_transform_pre_multiply(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_transform_pre_multiply(self.0) }
    }
    fn post_multiply(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_transform_post_multiply(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_transform_post_multiply(self.0) }
    }
    fn get_number_of_concatenated_transforms(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_transform_get_number_of_concatenated_transforms(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_transform_get_number_of_concatenated_transforms(self.0) }
    }
    fn get_concatenated_transform(
        &mut self,
        i: core::ffi::c_int,
    ) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_get_concatenated_transform(
                sself: *mut core::ffi::c_void,
                i: core::ffi::c_int,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_get_concatenated_transform(self.0, i) }
    }
    fn get_inverse(&mut self, inverse: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_get_inverse(
                sself: *mut core::ffi::c_void,
                inverse: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_get_inverse(self.0, inverse) }
    }
    fn get_transpose(&mut self, transpose: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_get_transpose(
                sself: *mut core::ffi::c_void,
                transpose: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_get_transpose(self.0, transpose) }
    }
    fn set_input(&mut self, input: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_set_input(
                sself: *mut core::ffi::c_void,
                input: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_set_input(self.0, input) }
    }
    fn get_input(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_get_input(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_get_input(self.0) }
    }
    fn get_inverse_flag(&mut self) -> core::ffi::c_int {
        unsafe extern "C" {
            fn vtk_transform_get_inverse_flag(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_int;
        }
        unsafe { vtk_transform_get_inverse_flag(self.0) }
    }
    fn push(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_transform_push(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_transform_push(self.0) }
    }
    fn pop(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_transform_pop(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_transform_pop(self.0) }
    }
    fn make_transform(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_make_transform(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_make_transform(self.0) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_transform_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_transform_get_m_time(self.0) }
    }
}
impl VtkTransform2D for vtkTransform2D {
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_2_d_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_2_d_new(self.0) }
    }
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_2_d_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_2_d_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_2_d_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_2_d_new_instance(self.0) }
    }
    fn identity(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_transform_2_d_identity(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_transform_2_d_identity(self.0) }
    }
    fn inverse(&mut self) -> () {
        unsafe extern "C" {
            fn vtk_transform_2_d_inverse(sself: *mut core::ffi::c_void);
        }
        unsafe { vtk_transform_2_d_inverse(self.0) }
    }
    fn translate(&mut self, x: core::ffi::c_double, y: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_transform_2_d_translate(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_2_d_translate(self.0, x, y) }
    }
    fn rotate(&mut self, angle: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_transform_2_d_rotate(
                sself: *mut core::ffi::c_void,
                angle: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_2_d_rotate(self.0, angle) }
    }
    fn scale(&mut self, x: core::ffi::c_double, y: core::ffi::c_double) -> () {
        unsafe extern "C" {
            fn vtk_transform_2_d_scale(
                sself: *mut core::ffi::c_void,
                x: core::ffi::c_double,
                y: core::ffi::c_double,
            );
        }
        unsafe { vtk_transform_2_d_scale(self.0, x, y) }
    }
    fn set_matrix(&mut self, matrix: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_2_d_set_matrix(
                sself: *mut core::ffi::c_void,
                matrix: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_2_d_set_matrix(self.0, matrix) }
    }
    fn get_matrix(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_2_d_get_matrix(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_2_d_get_matrix(self.0) }
    }
    fn get_inverse(&mut self, inverse: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_2_d_get_inverse(
                sself: *mut core::ffi::c_void,
                inverse: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_2_d_get_inverse(self.0, inverse) }
    }
    fn get_transpose(&mut self, transpose: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_2_d_get_transpose(
                sself: *mut core::ffi::c_void,
                transpose: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_2_d_get_transpose(self.0, transpose) }
    }
    fn get_m_time(&mut self) -> core::ffi::c_ulong {
        unsafe extern "C" {
            fn vtk_transform_2_d_get_m_time(
                sself: *mut core::ffi::c_void,
            ) -> core::ffi::c_ulong;
        }
        unsafe { vtk_transform_2_d_get_m_time(self.0) }
    }
}
impl VtkTransformCollection for vtkTransformCollection {
    fn safe_down_cast(&mut self, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_collection_safe_down_cast(
                sself: *mut core::ffi::c_void,
                o: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_collection_safe_down_cast(self.0, o) }
    }
    fn new_instance(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_collection_new_instance(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_collection_new_instance(self.0) }
    }
    fn new(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_collection_new(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_collection_new(self.0) }
    }
    fn add_item(&mut self, p0: *mut core::ffi::c_void) -> () {
        unsafe extern "C" {
            fn vtk_transform_collection_add_item(
                sself: *mut core::ffi::c_void,
                p0: *mut core::ffi::c_void,
            );
        }
        unsafe { vtk_transform_collection_add_item(self.0, p0) }
    }
    fn get_next_item(&mut self) -> *mut core::ffi::c_void {
        unsafe extern "C" {
            fn vtk_transform_collection_get_next_item(
                sself: *mut core::ffi::c_void,
            ) -> *mut core::ffi::c_void;
        }
        unsafe { vtk_transform_collection_get_next_item(self.0) }
    }
}
/// cylindrical to rectangular coords and back
///
///
/// vtkCylindricalTransform will convert (r,theta,z) coordinates to
/// (x,y,z) coordinates and back again.  The angles are given in radians.
/// By default, it converts cylindrical coordinates to rectangular, but
/// GetInverse() returns a transform that will do the opposite.  The
/// equation that is used is x = r*cos(theta), y = r*sin(theta), z = z.
/// @warning
/// This transform is not well behaved along the line x=y=0 (i.e. along
/// the z-axis)
/// @sa
/// vtkSphericalTransform vtkGeneralTransform
#[allow(non_camel_case_types)]
pub struct vtkCylindricalTransform(*mut core::ffi::c_void);
impl vtkCylindricalTransform {
    /// Creates a new [vtkCylindricalTransform] via `vtkCylindricalTransform::New()`
    #[doc(alias = "vtkCylindricalTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkCylindricalTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkCylindricalTransform_new() })
    }
}
impl std::default::Default for vtkCylindricalTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkCylindricalTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkCylindricalTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkCylindricalTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkCylindricalTransform_create_drop() {
    let obj = vtkCylindricalTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// allows operations on any transforms
///
///
/// vtkGeneralTransform is like vtkTransform and vtkPerspectiveTransform,
/// but it will work with any vtkAbstractTransform as input.  It is
/// not as efficient as the other two, however, because arbitrary
/// transformations cannot be concatenated by matrix multiplication.
/// Transform concatenation is simulated by passing each input point
/// through each transform in turn.
/// @sa
/// vtkTransform vtkPerspectiveTransform
#[allow(non_camel_case_types)]
pub struct vtkGeneralTransform(*mut core::ffi::c_void);
impl vtkGeneralTransform {
    /// Creates a new [vtkGeneralTransform] via `vtkGeneralTransform::New()`
    #[doc(alias = "vtkGeneralTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkGeneralTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkGeneralTransform_new() })
    }
}
impl std::default::Default for vtkGeneralTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkGeneralTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkGeneralTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkGeneralTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkGeneralTransform_create_drop() {
    let obj = vtkGeneralTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// a transform that doesn't do anything
///
///
/// vtkIdentityTransform is a transformation which will simply pass coordinate
/// data unchanged.  All other transform types can also do this, however,
/// the vtkIdentityTransform does so with much greater efficiency.
/// @sa
/// vtkLinearTransform
#[allow(non_camel_case_types)]
pub struct vtkIdentityTransform(*mut core::ffi::c_void);
impl vtkIdentityTransform {
    /// Creates a new [vtkIdentityTransform] via `vtkIdentityTransform::New()`
    #[doc(alias = "vtkIdentityTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkIdentityTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkIdentityTransform_new() })
    }
}
impl std::default::Default for vtkIdentityTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkIdentityTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkIdentityTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkIdentityTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkIdentityTransform_create_drop() {
    let obj = vtkIdentityTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// a linear transform specified by two corresponding point sets
///
///
/// A vtkLandmarkTransform is defined by two sets of landmarks, the
/// transform computed gives the best fit mapping one onto the other, in a
/// least squares sense. The indices are taken to correspond, so point 1
/// in the first set will get mapped close to point 1 in the second set,
/// etc. Call SetSourceLandmarks and SetTargetLandmarks to specify the two
/// sets of landmarks, ensure they have the same number of points.
/// @warning
/// Whenever you add, subtract, or set points you must call Modified()
/// on the vtkPoints object, or the transformation might not update.
/// @sa
/// vtkLinearTransform
#[allow(non_camel_case_types)]
pub struct vtkLandmarkTransform(*mut core::ffi::c_void);
impl vtkLandmarkTransform {
    /// Creates a new [vtkLandmarkTransform] via `vtkLandmarkTransform::New()`
    #[doc(alias = "vtkLandmarkTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkLandmarkTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkLandmarkTransform_new() })
    }
}
impl std::default::Default for vtkLandmarkTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkLandmarkTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkLandmarkTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkLandmarkTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkLandmarkTransform_create_drop() {
    let obj = vtkLandmarkTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// convert a matrix to a transform
///
///
/// This is a very simple class which allows a vtkMatrix4x4 to be used in
/// place of a vtkHomogeneousTransform or vtkAbstractTransform.  For example,
/// if you use it as a proxy between a matrix and vtkTransformPolyDataFilter
/// then any modifications to the matrix will automatically be reflected in
/// the output of the filter.
/// @sa
/// vtkPerspectiveTransform vtkMatrix4x4 vtkMatrixToLinearTransform
#[allow(non_camel_case_types)]
pub struct vtkMatrixToHomogeneousTransform(*mut core::ffi::c_void);
impl vtkMatrixToHomogeneousTransform {
    /// Creates a new [vtkMatrixToHomogeneousTransform] via `vtkMatrixToHomogeneousTransform::New()`
    #[doc(alias = "vtkMatrixToHomogeneousTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMatrixToHomogeneousTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMatrixToHomogeneousTransform_new() })
    }
}
impl std::default::Default for vtkMatrixToHomogeneousTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMatrixToHomogeneousTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMatrixToHomogeneousTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMatrixToHomogeneousTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMatrixToHomogeneousTransform_create_drop() {
    let obj = vtkMatrixToHomogeneousTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// convert a matrix to a transform
///
///
/// This is a very simple class which allows a vtkMatrix4x4 to be used in
/// place of a vtkLinearTransform or vtkAbstractTransform.  For example,
/// if you use it as a proxy between a matrix and vtkTransformPolyDataFilter
/// then any modifications to the matrix will automatically be reflected in
/// the output of the filter.
/// @sa
/// vtkTransform vtkMatrix4x4 vtkMatrixToHomogeneousTransform
#[allow(non_camel_case_types)]
pub struct vtkMatrixToLinearTransform(*mut core::ffi::c_void);
impl vtkMatrixToLinearTransform {
    /// Creates a new [vtkMatrixToLinearTransform] via `vtkMatrixToLinearTransform::New()`
    #[doc(alias = "vtkMatrixToLinearTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkMatrixToLinearTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkMatrixToLinearTransform_new() })
    }
}
impl std::default::Default for vtkMatrixToLinearTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkMatrixToLinearTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkMatrixToLinearTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkMatrixToLinearTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkMatrixToLinearTransform_create_drop() {
    let obj = vtkMatrixToLinearTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// describes a 4x4 matrix transformation
///
///
/// A vtkPerspectiveTransform can be used to describe the full range of
/// homogeneous transformations.  It was designed in particular
/// to describe a camera-view of a scene.
/// <P>The order in which you set up the display coordinates (via
/// AdjustZBuffer() and AdjustViewport()), the projection (via Perspective(),
/// Frustum(), or Ortho()) and the camera view (via SetupCamera()) are
/// important.  If the transform is in PreMultiply mode, which is the
/// default, set the Viewport and ZBuffer first, then the projection, and
/// finally the camera view.  Once the view is set up, the Translate
/// and Rotate methods can be used to move the camera around in world
/// coordinates.  If the Oblique() or Stereo() methods are used, they
/// should be called just before SetupCamera().
/// <P>In PostMultiply mode, you must perform all transformations
/// in the opposite order.  This is necessary, for example, if you
/// already have a perspective transformation set up but must adjust
/// the viewport.  Another example is if you have a view transformation,
/// and wish to perform translations and rotations in the camera's
/// coordinate system rather than in world coordinates.
/// <P>The SetInput and Concatenate methods can be used to create
/// a transformation pipeline with vtkPerspectiveTransform.  See vtkTransform
/// for more information on the transformation pipeline.
/// @sa
/// vtkGeneralTransform vtkTransform vtkMatrix4x4 vtkCamera
#[allow(non_camel_case_types)]
pub struct vtkPerspectiveTransform(*mut core::ffi::c_void);
impl vtkPerspectiveTransform {
    /// Creates a new [vtkPerspectiveTransform] via `vtkPerspectiveTransform::New()`
    #[doc(alias = "vtkPerspectiveTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkPerspectiveTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkPerspectiveTransform_new() })
    }
}
impl std::default::Default for vtkPerspectiveTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkPerspectiveTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkPerspectiveTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkPerspectiveTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkPerspectiveTransform_create_drop() {
    let obj = vtkPerspectiveTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// spherical to rectangular coords and back
///
///
/// vtkSphericalTransform will convert (r,phi,theta) coordinates to
/// (x,y,z) coordinates and back again.  The angles are given in radians.
/// By default, it converts spherical coordinates to rectangular, but
/// GetInverse() returns a transform that will do the opposite.  The equation
/// that is used is x = r*sin(phi)*cos(theta), y = r*sin(phi)*sin(theta),
/// z = r*cos(phi).
/// @warning
/// This transform is not well behaved along the line x=y=0 (i.e. along
/// the z-axis)
/// @sa
/// vtkCylindricalTransform vtkGeneralTransform
#[allow(non_camel_case_types)]
pub struct vtkSphericalTransform(*mut core::ffi::c_void);
impl vtkSphericalTransform {
    /// Creates a new [vtkSphericalTransform] via `vtkSphericalTransform::New()`
    #[doc(alias = "vtkSphericalTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkSphericalTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkSphericalTransform_new() })
    }
}
impl std::default::Default for vtkSphericalTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkSphericalTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkSphericalTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkSphericalTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkSphericalTransform_create_drop() {
    let obj = vtkSphericalTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// a nonlinear warp transformation
///
///
/// vtkThinPlateSplineTransform describes a nonlinear warp transform defined
/// by a set of source and target landmarks. Any point on the mesh close to a
/// source landmark will be moved to a place close to the corresponding target
/// landmark. The points in between are interpolated smoothly using
/// Bookstein's Thin Plate Spline algorithm.
///
/// To obtain a correct TPS warp, use the R2LogR kernel if your data is 2D, and
/// the R kernel if your data is 3D. Or you can specify your own RBF. (Hence this
/// class is more general than a pure TPS transform.)
/// @warning
/// 1) The inverse transform is calculated using an iterative method,
/// and is several times more expensive than the forward transform.
/// 2) Whenever you add, subtract, or set points you must call Modified()
/// on the vtkPoints object, or the transformation might not update.
/// 3) Collinear point configurations (except those that lie in the XY plane)
/// result in an unstable transformation. Forward transform can be computed
/// for any configuration by disabling bulk transform regularization.
/// @sa
/// vtkGridTransform vtkGeneralTransform
#[allow(non_camel_case_types)]
pub struct vtkThinPlateSplineTransform(*mut core::ffi::c_void);
impl vtkThinPlateSplineTransform {
    /// Creates a new [vtkThinPlateSplineTransform] via `vtkThinPlateSplineTransform::New()`
    #[doc(alias = "vtkThinPlateSplineTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkThinPlateSplineTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkThinPlateSplineTransform_new() })
    }
}
impl std::default::Default for vtkThinPlateSplineTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkThinPlateSplineTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkThinPlateSplineTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkThinPlateSplineTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkThinPlateSplineTransform_create_drop() {
    let obj = vtkThinPlateSplineTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// describes linear transformations via a 4x4 matrix
///
///
/// A vtkTransform can be used to describe the full range of linear (also
/// known as affine) coordinate transformations in three dimensions,
/// which are internally represented as a 4x4 homogeneous transformation
/// matrix.  When you create a new vtkTransform, it is always initialized
/// to the identity transformation.
/// <P>The SetInput() method allows you to set another transform,
/// instead of the identity transform, to be the base transformation.
/// There is a pipeline mechanism to ensure that when the input is
/// modified, the current transformation will be updated accordingly.
/// This pipeline mechanism is also supported by the Concatenate() method.
/// <P>Most of the methods for manipulating this transformation,
/// e.g. Translate, Rotate, and Concatenate, can operate in either
/// PreMultiply (the default) or PostMultiply mode.  In PreMultiply
/// mode, the translation, concatenation, etc. will occur before any
/// transformations which are represented by the current matrix.  In
/// PostMultiply mode, the additional transformation will occur after
/// any transformations represented by the current matrix.
/// <P>This class performs all of its operations in a right handed
/// coordinate system with right handed rotations. Some other graphics
/// libraries use left handed coordinate systems and rotations.
/// @sa
/// vtkPerspectiveTransform vtkGeneralTransform vtkMatrix4x4
/// vtkTransformCollection vtkTransformFilter vtkTransformPolyDataFilter
/// vtkImageReslice
#[allow(non_camel_case_types)]
pub struct vtkTransform(*mut core::ffi::c_void);
impl vtkTransform {
    /// Creates a new [vtkTransform] via `vtkTransform::New()`
    #[doc(alias = "vtkTransform")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTransform_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTransform_new() })
    }
}
impl std::default::Default for vtkTransform {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTransform {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTransform_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTransform_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTransform_create_drop() {
    let obj = vtkTransform::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// describes linear transformations via a 3x3 matrix
///
///
/// A vtkTransform2D can be used to describe the full range of linear (also
/// known as affine) coordinate transformations in two dimensions,
/// which are internally represented as a 3x3 homogeneous transformation
/// matrix.  When you create a new vtkTransform2D, it is always initialized
/// to the identity transformation.
///
/// All multiplicitive operations (Translate, Rotate, Scale, etc) are
/// post-multiplied in this class (i.e. add them in the reverse of the order
/// that they should be applied).
///
/// This class performs all of its operations in a right handed
/// coordinate system with right handed rotations. Some other graphics
/// libraries use left handed coordinate systems and rotations.
#[allow(non_camel_case_types)]
pub struct vtkTransform2D(*mut core::ffi::c_void);
impl vtkTransform2D {
    /// Creates a new [vtkTransform2D] via `vtkTransform2D::New()`
    #[doc(alias = "vtkTransform2D")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTransform2D_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTransform2D_new() })
    }
}
impl std::default::Default for vtkTransform2D {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTransform2D {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTransform2D_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTransform2D_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTransform2D_create_drop() {
    let obj = vtkTransform2D::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
/// maintain a list of transforms
///
///
///
/// vtkTransformCollection is an object that creates and manipulates lists of
/// objects of type vtkTransform.
///
/// @sa
/// vtkCollection vtkTransform
#[allow(non_camel_case_types)]
pub struct vtkTransformCollection(*mut core::ffi::c_void);
impl vtkTransformCollection {
    /// Creates a new [vtkTransformCollection] via `vtkTransformCollection::New()`
    #[doc(alias = "vtkTransformCollection")]
    pub fn new() -> Self {
        unsafe extern "C" {
            fn vtkTransformCollection_new() -> *mut core::ffi::c_void;
        }
        Self(unsafe { vtkTransformCollection_new() })
    }
}
impl std::default::Default for vtkTransformCollection {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for vtkTransformCollection {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn vtkTransformCollection_destructor(sself: *mut core::ffi::c_void);
        }
        unsafe { vtkTransformCollection_destructor(self.0) }
        self.0 = core::ptr::null_mut();
    }
}
#[test]
fn test_vtkTransformCollection_create_drop() {
    let obj = vtkTransformCollection::new();
    assert!(!obj.0.is_null());
    drop(obj);
}
