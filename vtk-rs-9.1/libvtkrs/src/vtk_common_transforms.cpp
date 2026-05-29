// Include header file
#include<vtk_common_transforms.h>

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

// Implement declared functions
extern "C" vtkCylindricalTransform * vtkCylindricalTransform_new () {return vtkCylindricalTransform :: New () ;}
extern "C" void vtkCylindricalTransform_destructor (vtkCylindricalTransform * sself) {sself -> Delete () ; return ;}
extern "C" vtkGeneralTransform * vtkGeneralTransform_new () {return vtkGeneralTransform :: New () ;}
extern "C" void vtkGeneralTransform_destructor (vtkGeneralTransform * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_general_transform_identity(vtkGeneralTransform* sself) { sself->Identity(); }
extern "C" void vtk_general_transform_inverse(vtkGeneralTransform* sself) { sself->Inverse(); }
extern "C" void vtk_general_transform_translate(vtkGeneralTransform* sself, double x, double y, double z) { sself->Translate(x, y, z); }
extern "C" void vtk_general_transform_rotate_wxyz(vtkGeneralTransform* sself, double angle, double x, double y, double z) { sself->RotateWXYZ(angle, x, y, z); }
extern "C" void vtk_general_transform_rotate_x(vtkGeneralTransform* sself, double angle) { sself->RotateX(angle); }
extern "C" void vtk_general_transform_rotate_y(vtkGeneralTransform* sself, double angle) { sself->RotateY(angle); }
extern "C" void vtk_general_transform_rotate_z(vtkGeneralTransform* sself, double angle) { sself->RotateZ(angle); }
extern "C" void vtk_general_transform_scale(vtkGeneralTransform* sself, double x, double y, double z) { sself->Scale(x, y, z); }
extern "C" void vtk_general_transform_pre_multiply(vtkGeneralTransform* sself) { sself->PreMultiply(); }
extern "C" void vtk_general_transform_post_multiply(vtkGeneralTransform* sself) { sself->PostMultiply(); }
extern "C" int vtk_general_transform_get_number_of_concatenated_transforms(vtkGeneralTransform* sself) { return sself->GetNumberOfConcatenatedTransforms(); }
extern "C" int vtk_general_transform_get_inverse_flag(vtkGeneralTransform* sself) { return sself->GetInverseFlag(); }
extern "C" void vtk_general_transform_push(vtkGeneralTransform* sself) { sself->Push(); }
extern "C" void vtk_general_transform_pop(vtkGeneralTransform* sself) { sself->Pop(); }
extern "C" unsigned long vtk_general_transform_get_m_time(vtkGeneralTransform* sself) { return sself->GetMTime(); }
extern "C" vtkIdentityTransform * vtkIdentityTransform_new () {return vtkIdentityTransform :: New () ;}
extern "C" void vtkIdentityTransform_destructor (vtkIdentityTransform * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_identity_transform_inverse(vtkIdentityTransform* sself) { sself->Inverse(); }
extern "C" vtkLandmarkTransform * vtkLandmarkTransform_new () {return vtkLandmarkTransform :: New () ;}
extern "C" void vtkLandmarkTransform_destructor (vtkLandmarkTransform * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_landmark_transform_set_mode(vtkLandmarkTransform* sself, int _arg) { sself->SetMode(_arg); }
extern "C" void vtk_landmark_transform_set_mode_to_rigid_body(vtkLandmarkTransform* sself) { sself->SetModeToRigidBody(); }
extern "C" void vtk_landmark_transform_set_mode_to_similarity(vtkLandmarkTransform* sself) { sself->SetModeToSimilarity(); }
extern "C" void vtk_landmark_transform_set_mode_to_affine(vtkLandmarkTransform* sself) { sself->SetModeToAffine(); }
extern "C" int vtk_landmark_transform_get_mode(vtkLandmarkTransform* sself) { return sself->GetMode(); }
extern "C" const char* vtk_landmark_transform_get_mode_as_string(vtkLandmarkTransform* sself) { return sself->GetModeAsString(); }
extern "C" void vtk_landmark_transform_inverse(vtkLandmarkTransform* sself) { sself->Inverse(); }
extern "C" unsigned long vtk_landmark_transform_get_m_time(vtkLandmarkTransform* sself) { return sself->GetMTime(); }
extern "C" vtkMatrixToHomogeneousTransform * vtkMatrixToHomogeneousTransform_new () {return vtkMatrixToHomogeneousTransform :: New () ;}
extern "C" void vtkMatrixToHomogeneousTransform_destructor (vtkMatrixToHomogeneousTransform * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_matrix_to_homogeneous_transform_inverse(vtkMatrixToHomogeneousTransform* sself) { sself->Inverse(); }
extern "C" unsigned long vtk_matrix_to_homogeneous_transform_get_m_time(vtkMatrixToHomogeneousTransform* sself) { return sself->GetMTime(); }
extern "C" vtkMatrixToLinearTransform * vtkMatrixToLinearTransform_new () {return vtkMatrixToLinearTransform :: New () ;}
extern "C" void vtkMatrixToLinearTransform_destructor (vtkMatrixToLinearTransform * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_matrix_to_linear_transform_inverse(vtkMatrixToLinearTransform* sself) { sself->Inverse(); }
extern "C" unsigned long vtk_matrix_to_linear_transform_get_m_time(vtkMatrixToLinearTransform* sself) { return sself->GetMTime(); }
extern "C" vtkPerspectiveTransform * vtkPerspectiveTransform_new () {return vtkPerspectiveTransform :: New () ;}
extern "C" void vtkPerspectiveTransform_destructor (vtkPerspectiveTransform * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_perspective_transform_identity(vtkPerspectiveTransform* sself) { sself->Identity(); }
extern "C" void vtk_perspective_transform_inverse(vtkPerspectiveTransform* sself) { sself->Inverse(); }
extern "C" void vtk_perspective_transform_adjust_viewport(vtkPerspectiveTransform* sself, double oldXMin, double oldXMax, double oldYMin, double oldYMax, double newXMin, double newXMax, double newYMin, double newYMax) { sself->AdjustViewport(oldXMin, oldXMax, oldYMin, oldYMax, newXMin, newXMax, newYMin, newYMax); }
extern "C" void vtk_perspective_transform_adjust_z_buffer(vtkPerspectiveTransform* sself, double oldNearZ, double oldFarZ, double newNearZ, double newFarZ) { sself->AdjustZBuffer(oldNearZ, oldFarZ, newNearZ, newFarZ); }
extern "C" void vtk_perspective_transform_ortho(vtkPerspectiveTransform* sself, double xmin, double xmax, double ymin, double ymax, double znear, double zfar) { sself->Ortho(xmin, xmax, ymin, ymax, znear, zfar); }
extern "C" void vtk_perspective_transform_frustum(vtkPerspectiveTransform* sself, double xmin, double xmax, double ymin, double ymax, double znear, double zfar) { sself->Frustum(xmin, xmax, ymin, ymax, znear, zfar); }
extern "C" void vtk_perspective_transform_perspective(vtkPerspectiveTransform* sself, double angle, double aspect, double znear, double zfar) { sself->Perspective(angle, aspect, znear, zfar); }
extern "C" void vtk_perspective_transform_shear(vtkPerspectiveTransform* sself, double dxdz, double dydz, double zplane) { sself->Shear(dxdz, dydz, zplane); }
extern "C" void vtk_perspective_transform_stereo(vtkPerspectiveTransform* sself, double angle, double focaldistance) { sself->Stereo(angle, focaldistance); }
extern "C" void vtk_perspective_transform_setup_camera(vtkPerspectiveTransform* sself, double p0, double p1, double p2, double fp0, double fp1, double fp2, double vup0, double vup1, double vup2) { sself->SetupCamera(p0, p1, p2, fp0, fp1, fp2, vup0, vup1, vup2); }
extern "C" void vtk_perspective_transform_translate(vtkPerspectiveTransform* sself, double x, double y, double z) { sself->Translate(x, y, z); }
extern "C" void vtk_perspective_transform_rotate_wxyz(vtkPerspectiveTransform* sself, double angle, double x, double y, double z) { sself->RotateWXYZ(angle, x, y, z); }
extern "C" void vtk_perspective_transform_rotate_x(vtkPerspectiveTransform* sself, double angle) { sself->RotateX(angle); }
extern "C" void vtk_perspective_transform_rotate_y(vtkPerspectiveTransform* sself, double angle) { sself->RotateY(angle); }
extern "C" void vtk_perspective_transform_rotate_z(vtkPerspectiveTransform* sself, double angle) { sself->RotateZ(angle); }
extern "C" void vtk_perspective_transform_scale(vtkPerspectiveTransform* sself, double x, double y, double z) { sself->Scale(x, y, z); }
extern "C" void vtk_perspective_transform_pre_multiply(vtkPerspectiveTransform* sself) { sself->PreMultiply(); }
extern "C" void vtk_perspective_transform_post_multiply(vtkPerspectiveTransform* sself) { sself->PostMultiply(); }
extern "C" int vtk_perspective_transform_get_number_of_concatenated_transforms(vtkPerspectiveTransform* sself) { return sself->GetNumberOfConcatenatedTransforms(); }
extern "C" int vtk_perspective_transform_get_inverse_flag(vtkPerspectiveTransform* sself) { return sself->GetInverseFlag(); }
extern "C" void vtk_perspective_transform_push(vtkPerspectiveTransform* sself) { sself->Push(); }
extern "C" void vtk_perspective_transform_pop(vtkPerspectiveTransform* sself) { sself->Pop(); }
extern "C" unsigned long vtk_perspective_transform_get_m_time(vtkPerspectiveTransform* sself) { return sself->GetMTime(); }
extern "C" vtkSphericalTransform * vtkSphericalTransform_new () {return vtkSphericalTransform :: New () ;}
extern "C" void vtkSphericalTransform_destructor (vtkSphericalTransform * sself) {sself -> Delete () ; return ;}
extern "C" vtkThinPlateSplineTransform * vtkThinPlateSplineTransform_new () {return vtkThinPlateSplineTransform :: New () ;}
extern "C" void vtkThinPlateSplineTransform_destructor (vtkThinPlateSplineTransform * sself) {sself -> Delete () ; return ;}
extern "C" double vtk_thin_plate_spline_transform_get_sigma(vtkThinPlateSplineTransform* sself) { return sself->GetSigma(); }
extern "C" void vtk_thin_plate_spline_transform_set_sigma(vtkThinPlateSplineTransform* sself, double _arg) { sself->SetSigma(_arg); }
extern "C" void vtk_thin_plate_spline_transform_set_basis(vtkThinPlateSplineTransform* sself, int basis) { sself->SetBasis(basis); }
extern "C" int vtk_thin_plate_spline_transform_get_basis(vtkThinPlateSplineTransform* sself) { return sself->GetBasis(); }
extern "C" void vtk_thin_plate_spline_transform_set_basis_to_r(vtkThinPlateSplineTransform* sself) { sself->SetBasisToR(); }
extern "C" void vtk_thin_plate_spline_transform_set_basis_to_r_2_log_r(vtkThinPlateSplineTransform* sself) { sself->SetBasisToR2LogR(); }
extern "C" const char* vtk_thin_plate_spline_transform_get_basis_as_string(vtkThinPlateSplineTransform* sself) { return sself->GetBasisAsString(); }
extern "C" unsigned long vtk_thin_plate_spline_transform_get_m_time(vtkThinPlateSplineTransform* sself) { return sself->GetMTime(); }
extern "C" bool vtk_thin_plate_spline_transform_get_regularize_bulk_transform(vtkThinPlateSplineTransform* sself) { return sself->GetRegularizeBulkTransform(); }
extern "C" void vtk_thin_plate_spline_transform_set_regularize_bulk_transform(vtkThinPlateSplineTransform* sself, bool _arg) { sself->SetRegularizeBulkTransform(_arg); }
extern "C" void vtk_thin_plate_spline_transform_regularize_bulk_transform_on(vtkThinPlateSplineTransform* sself) { sself->RegularizeBulkTransformOn(); }
extern "C" void vtk_thin_plate_spline_transform_regularize_bulk_transform_off(vtkThinPlateSplineTransform* sself) { sself->RegularizeBulkTransformOff(); }
extern "C" vtkTransform * vtkTransform_new () {return vtkTransform :: New () ;}
extern "C" void vtkTransform_destructor (vtkTransform * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_transform_identity(vtkTransform* sself) { sself->Identity(); }
extern "C" void vtk_transform_inverse(vtkTransform* sself) { sself->Inverse(); }
extern "C" void vtk_transform_translate(vtkTransform* sself, double x, double y, double z) { sself->Translate(x, y, z); }
extern "C" void vtk_transform_rotate_wxyz(vtkTransform* sself, double angle, double x, double y, double z) { sself->RotateWXYZ(angle, x, y, z); }
extern "C" void vtk_transform_rotate_x(vtkTransform* sself, double angle) { sself->RotateX(angle); }
extern "C" void vtk_transform_rotate_y(vtkTransform* sself, double angle) { sself->RotateY(angle); }
extern "C" void vtk_transform_rotate_z(vtkTransform* sself, double angle) { sself->RotateZ(angle); }
extern "C" void vtk_transform_scale(vtkTransform* sself, double x, double y, double z) { sself->Scale(x, y, z); }
extern "C" void vtk_transform_pre_multiply(vtkTransform* sself) { sself->PreMultiply(); }
extern "C" void vtk_transform_post_multiply(vtkTransform* sself) { sself->PostMultiply(); }
extern "C" int vtk_transform_get_number_of_concatenated_transforms(vtkTransform* sself) { return sself->GetNumberOfConcatenatedTransforms(); }
extern "C" int vtk_transform_get_inverse_flag(vtkTransform* sself) { return sself->GetInverseFlag(); }
extern "C" void vtk_transform_push(vtkTransform* sself) { sself->Push(); }
extern "C" void vtk_transform_pop(vtkTransform* sself) { sself->Pop(); }
extern "C" unsigned long vtk_transform_get_m_time(vtkTransform* sself) { return sself->GetMTime(); }
extern "C" vtkTransform2D * vtkTransform2D_new () {return vtkTransform2D :: New () ;}
extern "C" void vtkTransform2D_destructor (vtkTransform2D * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_transform_2_d_identity(vtkTransform2D* sself) { sself->Identity(); }
extern "C" void vtk_transform_2_d_inverse(vtkTransform2D* sself) { sself->Inverse(); }
extern "C" void vtk_transform_2_d_translate(vtkTransform2D* sself, double x, double y) { sself->Translate(x, y); }
extern "C" void vtk_transform_2_d_rotate(vtkTransform2D* sself, double angle) { sself->Rotate(angle); }
extern "C" void vtk_transform_2_d_scale(vtkTransform2D* sself, double x, double y) { sself->Scale(x, y); }
extern "C" unsigned long vtk_transform_2_d_get_m_time(vtkTransform2D* sself) { return sself->GetMTime(); }
extern "C" vtkTransformCollection * vtkTransformCollection_new () {return vtkTransformCollection :: New () ;}
extern "C" void vtkTransformCollection_destructor (vtkTransformCollection * sself) {sself -> Delete () ; return ;}
