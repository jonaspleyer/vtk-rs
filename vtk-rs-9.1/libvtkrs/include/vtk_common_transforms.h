// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkAbstractTransform.h>
#include<vtkCylindricalTransform.h>
#include<vtkGeneralTransform.h>
#include<vtkHomogeneousTransform.h>
#include<vtkIdentityTransform.h>
#include<vtkLandmarkTransform.h>
#include<vtkLinearTransform.h>
#include<vtkMatrixToHomogeneousTransform.h>
#include<vtkMatrixToLinearTransform.h>
#include<vtkPerspectiveTransform.h>
#include<vtkSphericalTransform.h>
#include<vtkThinPlateSplineTransform.h>
#include<vtkTransform.h>
#include<vtkTransform2D.h>
#include<vtkTransformCollection.h>
#include<vtkAbstractTransform.h>
#include<vtkAbstractTransform.h>
#include<vtkAbstractTransform.h>
#include<vtkWarpTransform.h>

// Declare exported functions
extern "C" vtkCylindricalTransform * vtkCylindricalTransform_new () ;
extern "C" void vtkCylindricalTransform_destructor (vtkCylindricalTransform * sself) ;
extern "C" vtkGeneralTransform * vtkGeneralTransform_new () ;
extern "C" void vtkGeneralTransform_destructor (vtkGeneralTransform * sself) ;
extern "C" void vtk_general_transform_identity(vtkGeneralTransform* sself);
extern "C" void vtk_general_transform_inverse(vtkGeneralTransform* sself);
extern "C" void vtk_general_transform_translate(vtkGeneralTransform* sself, double x, double y, double z);
extern "C" void vtk_general_transform_rotate_wxyz(vtkGeneralTransform* sself, double angle, double x, double y, double z);
extern "C" void vtk_general_transform_rotate_x(vtkGeneralTransform* sself, double angle);
extern "C" void vtk_general_transform_rotate_y(vtkGeneralTransform* sself, double angle);
extern "C" void vtk_general_transform_rotate_z(vtkGeneralTransform* sself, double angle);
extern "C" void vtk_general_transform_scale(vtkGeneralTransform* sself, double x, double y, double z);
extern "C" void vtk_general_transform_pre_multiply(vtkGeneralTransform* sself);
extern "C" void vtk_general_transform_post_multiply(vtkGeneralTransform* sself);
extern "C" int vtk_general_transform_get_number_of_concatenated_transforms(vtkGeneralTransform* sself);
extern "C" int vtk_general_transform_get_inverse_flag(vtkGeneralTransform* sself);
extern "C" void vtk_general_transform_push(vtkGeneralTransform* sself);
extern "C" void vtk_general_transform_pop(vtkGeneralTransform* sself);
extern "C" unsigned long vtk_general_transform_get_m_time(vtkGeneralTransform* sself);
extern "C" vtkIdentityTransform * vtkIdentityTransform_new () ;
extern "C" void vtkIdentityTransform_destructor (vtkIdentityTransform * sself) ;
extern "C" void vtk_identity_transform_inverse(vtkIdentityTransform* sself);
extern "C" vtkLandmarkTransform * vtkLandmarkTransform_new () ;
extern "C" void vtkLandmarkTransform_destructor (vtkLandmarkTransform * sself) ;
extern "C" void vtk_landmark_transform_set_mode(vtkLandmarkTransform* sself, int _arg);
extern "C" void vtk_landmark_transform_set_mode_to_rigid_body(vtkLandmarkTransform* sself);
extern "C" void vtk_landmark_transform_set_mode_to_similarity(vtkLandmarkTransform* sself);
extern "C" void vtk_landmark_transform_set_mode_to_affine(vtkLandmarkTransform* sself);
extern "C" int vtk_landmark_transform_get_mode(vtkLandmarkTransform* sself);
extern "C" const char* vtk_landmark_transform_get_mode_as_string(vtkLandmarkTransform* sself);
extern "C" void vtk_landmark_transform_inverse(vtkLandmarkTransform* sself);
extern "C" unsigned long vtk_landmark_transform_get_m_time(vtkLandmarkTransform* sself);
extern "C" vtkMatrixToHomogeneousTransform * vtkMatrixToHomogeneousTransform_new () ;
extern "C" void vtkMatrixToHomogeneousTransform_destructor (vtkMatrixToHomogeneousTransform * sself) ;
extern "C" void vtk_matrix_to_homogeneous_transform_inverse(vtkMatrixToHomogeneousTransform* sself);
extern "C" unsigned long vtk_matrix_to_homogeneous_transform_get_m_time(vtkMatrixToHomogeneousTransform* sself);
extern "C" vtkMatrixToLinearTransform * vtkMatrixToLinearTransform_new () ;
extern "C" void vtkMatrixToLinearTransform_destructor (vtkMatrixToLinearTransform * sself) ;
extern "C" void vtk_matrix_to_linear_transform_inverse(vtkMatrixToLinearTransform* sself);
extern "C" unsigned long vtk_matrix_to_linear_transform_get_m_time(vtkMatrixToLinearTransform* sself);
extern "C" vtkPerspectiveTransform * vtkPerspectiveTransform_new () ;
extern "C" void vtkPerspectiveTransform_destructor (vtkPerspectiveTransform * sself) ;
extern "C" void vtk_perspective_transform_identity(vtkPerspectiveTransform* sself);
extern "C" void vtk_perspective_transform_inverse(vtkPerspectiveTransform* sself);
extern "C" void vtk_perspective_transform_adjust_viewport(vtkPerspectiveTransform* sself, double oldXMin, double oldXMax, double oldYMin, double oldYMax, double newXMin, double newXMax, double newYMin, double newYMax);
extern "C" void vtk_perspective_transform_adjust_z_buffer(vtkPerspectiveTransform* sself, double oldNearZ, double oldFarZ, double newNearZ, double newFarZ);
extern "C" void vtk_perspective_transform_ortho(vtkPerspectiveTransform* sself, double xmin, double xmax, double ymin, double ymax, double znear, double zfar);
extern "C" void vtk_perspective_transform_frustum(vtkPerspectiveTransform* sself, double xmin, double xmax, double ymin, double ymax, double znear, double zfar);
extern "C" void vtk_perspective_transform_perspective(vtkPerspectiveTransform* sself, double angle, double aspect, double znear, double zfar);
extern "C" void vtk_perspective_transform_shear(vtkPerspectiveTransform* sself, double dxdz, double dydz, double zplane);
extern "C" void vtk_perspective_transform_stereo(vtkPerspectiveTransform* sself, double angle, double focaldistance);
extern "C" void vtk_perspective_transform_setup_camera(vtkPerspectiveTransform* sself, double p0, double p1, double p2, double fp0, double fp1, double fp2, double vup0, double vup1, double vup2);
extern "C" void vtk_perspective_transform_translate(vtkPerspectiveTransform* sself, double x, double y, double z);
extern "C" void vtk_perspective_transform_rotate_wxyz(vtkPerspectiveTransform* sself, double angle, double x, double y, double z);
extern "C" void vtk_perspective_transform_rotate_x(vtkPerspectiveTransform* sself, double angle);
extern "C" void vtk_perspective_transform_rotate_y(vtkPerspectiveTransform* sself, double angle);
extern "C" void vtk_perspective_transform_rotate_z(vtkPerspectiveTransform* sself, double angle);
extern "C" void vtk_perspective_transform_scale(vtkPerspectiveTransform* sself, double x, double y, double z);
extern "C" void vtk_perspective_transform_pre_multiply(vtkPerspectiveTransform* sself);
extern "C" void vtk_perspective_transform_post_multiply(vtkPerspectiveTransform* sself);
extern "C" int vtk_perspective_transform_get_number_of_concatenated_transforms(vtkPerspectiveTransform* sself);
extern "C" int vtk_perspective_transform_get_inverse_flag(vtkPerspectiveTransform* sself);
extern "C" void vtk_perspective_transform_push(vtkPerspectiveTransform* sself);
extern "C" void vtk_perspective_transform_pop(vtkPerspectiveTransform* sself);
extern "C" unsigned long vtk_perspective_transform_get_m_time(vtkPerspectiveTransform* sself);
extern "C" vtkSphericalTransform * vtkSphericalTransform_new () ;
extern "C" void vtkSphericalTransform_destructor (vtkSphericalTransform * sself) ;
extern "C" vtkThinPlateSplineTransform * vtkThinPlateSplineTransform_new () ;
extern "C" void vtkThinPlateSplineTransform_destructor (vtkThinPlateSplineTransform * sself) ;
extern "C" double vtk_thin_plate_spline_transform_get_sigma(vtkThinPlateSplineTransform* sself);
extern "C" void vtk_thin_plate_spline_transform_set_sigma(vtkThinPlateSplineTransform* sself, double _arg);
extern "C" void vtk_thin_plate_spline_transform_set_basis(vtkThinPlateSplineTransform* sself, int basis);
extern "C" int vtk_thin_plate_spline_transform_get_basis(vtkThinPlateSplineTransform* sself);
extern "C" void vtk_thin_plate_spline_transform_set_basis_to_r(vtkThinPlateSplineTransform* sself);
extern "C" void vtk_thin_plate_spline_transform_set_basis_to_r_2_log_r(vtkThinPlateSplineTransform* sself);
extern "C" const char* vtk_thin_plate_spline_transform_get_basis_as_string(vtkThinPlateSplineTransform* sself);
extern "C" unsigned long vtk_thin_plate_spline_transform_get_m_time(vtkThinPlateSplineTransform* sself);
extern "C" bool vtk_thin_plate_spline_transform_get_regularize_bulk_transform(vtkThinPlateSplineTransform* sself);
extern "C" void vtk_thin_plate_spline_transform_set_regularize_bulk_transform(vtkThinPlateSplineTransform* sself, bool _arg);
extern "C" void vtk_thin_plate_spline_transform_regularize_bulk_transform_on(vtkThinPlateSplineTransform* sself);
extern "C" void vtk_thin_plate_spline_transform_regularize_bulk_transform_off(vtkThinPlateSplineTransform* sself);
extern "C" vtkTransform * vtkTransform_new () ;
extern "C" void vtkTransform_destructor (vtkTransform * sself) ;
extern "C" void vtk_transform_identity(vtkTransform* sself);
extern "C" void vtk_transform_inverse(vtkTransform* sself);
extern "C" void vtk_transform_translate(vtkTransform* sself, double x, double y, double z);
extern "C" void vtk_transform_rotate_wxyz(vtkTransform* sself, double angle, double x, double y, double z);
extern "C" void vtk_transform_rotate_x(vtkTransform* sself, double angle);
extern "C" void vtk_transform_rotate_y(vtkTransform* sself, double angle);
extern "C" void vtk_transform_rotate_z(vtkTransform* sself, double angle);
extern "C" void vtk_transform_scale(vtkTransform* sself, double x, double y, double z);
extern "C" void vtk_transform_pre_multiply(vtkTransform* sself);
extern "C" void vtk_transform_post_multiply(vtkTransform* sself);
extern "C" int vtk_transform_get_number_of_concatenated_transforms(vtkTransform* sself);
extern "C" int vtk_transform_get_inverse_flag(vtkTransform* sself);
extern "C" void vtk_transform_push(vtkTransform* sself);
extern "C" void vtk_transform_pop(vtkTransform* sself);
extern "C" unsigned long vtk_transform_get_m_time(vtkTransform* sself);
extern "C" vtkTransform2D * vtkTransform2D_new () ;
extern "C" void vtkTransform2D_destructor (vtkTransform2D * sself) ;
extern "C" void vtk_transform_2_d_identity(vtkTransform2D* sself);
extern "C" void vtk_transform_2_d_inverse(vtkTransform2D* sself);
extern "C" void vtk_transform_2_d_translate(vtkTransform2D* sself, double x, double y);
extern "C" void vtk_transform_2_d_rotate(vtkTransform2D* sself, double angle);
extern "C" void vtk_transform_2_d_scale(vtkTransform2D* sself, double x, double y);
extern "C" unsigned long vtk_transform_2_d_get_m_time(vtkTransform2D* sself);
extern "C" vtkTransformCollection * vtkTransformCollection_new () ;
extern "C" void vtkTransformCollection_destructor (vtkTransformCollection * sself) ;
