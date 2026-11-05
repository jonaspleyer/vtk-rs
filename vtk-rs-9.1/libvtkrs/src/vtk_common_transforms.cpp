// Include header file
#include <vtk_common_transforms.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkAbstractTransform.h>
#include <vtkCylindricalTransform.h>
#include <vtkGeneralTransform.h>
#include <vtkHomogeneousTransform.h>
#include <vtkIdentityTransform.h>
#include <vtkLandmarkTransform.h>
#include <vtkLinearTransform.h>
#include <vtkMatrixToHomogeneousTransform.h>
#include <vtkMatrixToLinearTransform.h>
#include <vtkPerspectiveTransform.h>
#include <vtkSphericalTransform.h>
#include <vtkThinPlateSplineTransform.h>
#include <vtkTransform.h>
#include <vtkTransform2D.h>
#include <vtkTransformCollection.h>
#include <vtkWarpTransform.h>

// Implement declared functions
extern "C" vtkNew<vtkCylindricalTransform> vtkCylindricalTransform_new() {
    return vtkNew<vtkCylindricalTransform>();
}
extern "C" void vtkCylindricalTransform_destructor(vtkNew<vtkCylindricalTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkCylindricalTransform_get_ptr(vtkNew<vtkCylindricalTransform> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkGeneralTransform> vtkGeneralTransform_new() {
    return vtkNew<vtkGeneralTransform>();
}
extern "C" void vtkGeneralTransform_destructor(vtkNew<vtkGeneralTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkGeneralTransform_get_ptr(vtkNew<vtkGeneralTransform> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_general_transform_identity(vtkNew<vtkGeneralTransform> self) {
    return self->Identity();
}
extern "C" void vtk_general_transform_inverse(vtkNew<vtkGeneralTransform> self) {
    return self->Inverse();
}
extern "C" void vtk_general_transform_translate_double_double_double(
    vtkNew<vtkGeneralTransform> self, double x, double y, double z
) {
    return self->Translate(x, y, z);
}
extern "C" void vtk_general_transform_translate_const_double_3(
    vtkNew<vtkGeneralTransform> self, const double* x
) {
    return self->Translate(x);
}
extern "C" void vtk_general_transform_translate_const_float_3(
    vtkNew<vtkGeneralTransform> self, const float* x
) {
    return self->Translate(x);
}
extern "C" void vtk_general_transform_rotate_wxyz_double_double_double_double(
    vtkNew<vtkGeneralTransform> self, double angle, double x, double y, double z
) {
    return self->RotateWXYZ(angle, x, y, z);
}
extern "C" void vtk_general_transform_rotate_wxyz_double_const_double_3(
    vtkNew<vtkGeneralTransform> self, double angle, const double* axis
) {
    return self->RotateWXYZ(angle, axis);
}
extern "C" void vtk_general_transform_rotate_wxyz_double_const_float_3(
    vtkNew<vtkGeneralTransform> self, double angle, const float* axis
) {
    return self->RotateWXYZ(angle, axis);
}
extern "C" void vtk_general_transform_rotate_x(vtkNew<vtkGeneralTransform> self, double angle) {
    return self->RotateX(angle);
}
extern "C" void vtk_general_transform_rotate_y(vtkNew<vtkGeneralTransform> self, double angle) {
    return self->RotateY(angle);
}
extern "C" void vtk_general_transform_rotate_z(vtkNew<vtkGeneralTransform> self, double angle) {
    return self->RotateZ(angle);
}
extern "C" void vtk_general_transform_scale_double_double_double(
    vtkNew<vtkGeneralTransform> self, double x, double y, double z
) {
    return self->Scale(x, y, z);
}
extern "C" void vtk_general_transform_scale_const_double_3(
    vtkNew<vtkGeneralTransform> self, const double* s
) {
    return self->Scale(s);
}
extern "C" void vtk_general_transform_scale_const_float_3(
    vtkNew<vtkGeneralTransform> self, const float* s
) {
    return self->Scale(s);
}
extern "C" void vtk_general_transform_concatenate_const_double_16(
    vtkNew<vtkGeneralTransform> self, const double* elements
) {
    return self->Concatenate(elements);
}
extern "C" void vtk_general_transform_pre_multiply(vtkNew<vtkGeneralTransform> self) {
    return self->PreMultiply();
}
extern "C" void vtk_general_transform_post_multiply(vtkNew<vtkGeneralTransform> self) {
    return self->PostMultiply();
}
extern "C" int vtk_general_transform_get_number_of_concatenated_transforms(
    vtkNew<vtkGeneralTransform> self
) {
    return self->GetNumberOfConcatenatedTransforms();
}
extern "C" int vtk_general_transform_get_inverse_flag(vtkNew<vtkGeneralTransform> self) {
    return self->GetInverseFlag();
}
extern "C" void vtk_general_transform_push(vtkNew<vtkGeneralTransform> self) {
    return self->Push();
}
extern "C" void vtk_general_transform_pop(vtkNew<vtkGeneralTransform> self) {
    return self->Pop();
}
extern "C" void vtk_general_transform_internal_transform_point_const_float_3_float_3(
    vtkNew<vtkGeneralTransform> self, const float* in, float* out
) {
    return self->InternalTransformPoint(in, out);
}
extern "C" void vtk_general_transform_internal_transform_point_const_double_3_double_3(
    vtkNew<vtkGeneralTransform> self, const double* in, double* out
) {
    return self->InternalTransformPoint(in, out);
}
extern "C" unsigned long vtk_general_transform_get_m_time(vtkNew<vtkGeneralTransform> self) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkIdentityTransform> vtkIdentityTransform_new() {
    return vtkNew<vtkIdentityTransform>();
}
extern "C" void vtkIdentityTransform_destructor(vtkNew<vtkIdentityTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkIdentityTransform_get_ptr(vtkNew<vtkIdentityTransform> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_identity_transform_inverse(vtkNew<vtkIdentityTransform> self) {
    return self->Inverse();
}
extern "C" vtkNew<vtkLandmarkTransform> vtkLandmarkTransform_new() {
    return vtkNew<vtkLandmarkTransform>();
}
extern "C" void vtkLandmarkTransform_destructor(vtkNew<vtkLandmarkTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkLandmarkTransform_get_ptr(vtkNew<vtkLandmarkTransform> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_landmark_transform_set_mode(vtkNew<vtkLandmarkTransform> self, int _arg) {
    return self->SetMode(_arg);
}
extern "C" void vtk_landmark_transform_set_mode_to_rigid_body(vtkNew<vtkLandmarkTransform> self) {
    return self->SetModeToRigidBody();
}
extern "C" void vtk_landmark_transform_set_mode_to_similarity(vtkNew<vtkLandmarkTransform> self) {
    return self->SetModeToSimilarity();
}
extern "C" void vtk_landmark_transform_set_mode_to_affine(vtkNew<vtkLandmarkTransform> self) {
    return self->SetModeToAffine();
}
extern "C" int vtk_landmark_transform_get_mode(vtkNew<vtkLandmarkTransform> self) {
    return self->GetMode();
}
extern "C" void vtk_landmark_transform_inverse(vtkNew<vtkLandmarkTransform> self) {
    return self->Inverse();
}
extern "C" unsigned long vtk_landmark_transform_get_m_time(vtkNew<vtkLandmarkTransform> self) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkMatrixToHomogeneousTransform> vtkMatrixToHomogeneousTransform_new() {
    return vtkNew<vtkMatrixToHomogeneousTransform>();
}
extern "C" void vtkMatrixToHomogeneousTransform_destructor(
    vtkNew<vtkMatrixToHomogeneousTransform> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkMatrixToHomogeneousTransform_get_ptr(
    vtkNew<vtkMatrixToHomogeneousTransform> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_matrix_to_homogeneous_transform_inverse(
    vtkNew<vtkMatrixToHomogeneousTransform> self
) {
    return self->Inverse();
}
extern "C" unsigned long vtk_matrix_to_homogeneous_transform_get_m_time(
    vtkNew<vtkMatrixToHomogeneousTransform> self
) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkMatrixToLinearTransform> vtkMatrixToLinearTransform_new() {
    return vtkNew<vtkMatrixToLinearTransform>();
}
extern "C" void vtkMatrixToLinearTransform_destructor(vtkNew<vtkMatrixToLinearTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMatrixToLinearTransform_get_ptr(vtkNew<vtkMatrixToLinearTransform> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_matrix_to_linear_transform_inverse(vtkNew<vtkMatrixToLinearTransform> self) {
    return self->Inverse();
}
extern "C" unsigned long vtk_matrix_to_linear_transform_get_m_time(
    vtkNew<vtkMatrixToLinearTransform> self
) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkPerspectiveTransform> vtkPerspectiveTransform_new() {
    return vtkNew<vtkPerspectiveTransform>();
}
extern "C" void vtkPerspectiveTransform_destructor(vtkNew<vtkPerspectiveTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkPerspectiveTransform_get_ptr(vtkNew<vtkPerspectiveTransform> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_perspective_transform_identity(vtkNew<vtkPerspectiveTransform> self) {
    return self->Identity();
}
extern "C" void vtk_perspective_transform_inverse(vtkNew<vtkPerspectiveTransform> self) {
    return self->Inverse();
}
extern "C" void vtk_perspective_transform_adjust_viewport(
    vtkNew<vtkPerspectiveTransform> self,
    double oldXMin,
    double oldXMax,
    double oldYMin,
    double oldYMax,
    double newXMin,
    double newXMax,
    double newYMin,
    double newYMax
) {
    return self->AdjustViewport(
        oldXMin, oldXMax, oldYMin, oldYMax, newXMin, newXMax, newYMin, newYMax
    );
}
extern "C" void vtk_perspective_transform_adjust_z_buffer(
    vtkNew<vtkPerspectiveTransform> self,
    double oldNearZ,
    double oldFarZ,
    double newNearZ,
    double newFarZ
) {
    return self->AdjustZBuffer(oldNearZ, oldFarZ, newNearZ, newFarZ);
}
extern "C" void vtk_perspective_transform_ortho(
    vtkNew<vtkPerspectiveTransform> self,
    double xmin,
    double xmax,
    double ymin,
    double ymax,
    double znear,
    double zfar
) {
    return self->Ortho(xmin, xmax, ymin, ymax, znear, zfar);
}
extern "C" void vtk_perspective_transform_frustum(
    vtkNew<vtkPerspectiveTransform> self,
    double xmin,
    double xmax,
    double ymin,
    double ymax,
    double znear,
    double zfar
) {
    return self->Frustum(xmin, xmax, ymin, ymax, znear, zfar);
}
extern "C" void vtk_perspective_transform_perspective(
    vtkNew<vtkPerspectiveTransform> self, double angle, double aspect, double znear, double zfar
) {
    return self->Perspective(angle, aspect, znear, zfar);
}
extern "C" void vtk_perspective_transform_shear(
    vtkNew<vtkPerspectiveTransform> self, double dxdz, double dydz, double zplane
) {
    return self->Shear(dxdz, dydz, zplane);
}
extern "C" void vtk_perspective_transform_stereo(
    vtkNew<vtkPerspectiveTransform> self, double angle, double focaldistance
) {
    return self->Stereo(angle, focaldistance);
}
extern "C" void vtk_perspective_transform_setup_camera_const_double_3_const_double_3_const_double_3(
    vtkNew<vtkPerspectiveTransform> self,
    const double* position,
    const double* focalpoint,
    const double* viewup
) {
    return self->SetupCamera(position, focalpoint, viewup);
}
extern "C" void vtk_perspective_transform_setup_camera_double_double_double_double_double_double_double_double_double(
    vtkNew<vtkPerspectiveTransform> self,
    double p0,
    double p1,
    double p2,
    double fp0,
    double fp1,
    double fp2,
    double vup0,
    double vup1,
    double vup2
) {
    return self->SetupCamera(p0, p1, p2, fp0, fp1, fp2, vup0, vup1, vup2);
}
extern "C" void vtk_perspective_transform_translate_double_double_double(
    vtkNew<vtkPerspectiveTransform> self, double x, double y, double z
) {
    return self->Translate(x, y, z);
}
extern "C" void vtk_perspective_transform_translate_const_double_3(
    vtkNew<vtkPerspectiveTransform> self, const double* x
) {
    return self->Translate(x);
}
extern "C" void vtk_perspective_transform_translate_const_float_3(
    vtkNew<vtkPerspectiveTransform> self, const float* x
) {
    return self->Translate(x);
}
extern "C" void vtk_perspective_transform_rotate_wxyz_double_double_double_double(
    vtkNew<vtkPerspectiveTransform> self, double angle, double x, double y, double z
) {
    return self->RotateWXYZ(angle, x, y, z);
}
extern "C" void vtk_perspective_transform_rotate_wxyz_double_const_double_3(
    vtkNew<vtkPerspectiveTransform> self, double angle, const double* axis
) {
    return self->RotateWXYZ(angle, axis);
}
extern "C" void vtk_perspective_transform_rotate_wxyz_double_const_float_3(
    vtkNew<vtkPerspectiveTransform> self, double angle, const float* axis
) {
    return self->RotateWXYZ(angle, axis);
}
extern "C" void vtk_perspective_transform_rotate_x(
    vtkNew<vtkPerspectiveTransform> self, double angle
) {
    return self->RotateX(angle);
}
extern "C" void vtk_perspective_transform_rotate_y(
    vtkNew<vtkPerspectiveTransform> self, double angle
) {
    return self->RotateY(angle);
}
extern "C" void vtk_perspective_transform_rotate_z(
    vtkNew<vtkPerspectiveTransform> self, double angle
) {
    return self->RotateZ(angle);
}
extern "C" void vtk_perspective_transform_scale_double_double_double(
    vtkNew<vtkPerspectiveTransform> self, double x, double y, double z
) {
    return self->Scale(x, y, z);
}
extern "C" void vtk_perspective_transform_scale_const_double_3(
    vtkNew<vtkPerspectiveTransform> self, const double* s
) {
    return self->Scale(s);
}
extern "C" void vtk_perspective_transform_scale_const_float_3(
    vtkNew<vtkPerspectiveTransform> self, const float* s
) {
    return self->Scale(s);
}
extern "C" void vtk_perspective_transform_set_matrix_const_double_16(
    vtkNew<vtkPerspectiveTransform> self, const double* elements
) {
    return self->SetMatrix(elements);
}
extern "C" void vtk_perspective_transform_concatenate_const_double_16(
    vtkNew<vtkPerspectiveTransform> self, const double* elements
) {
    return self->Concatenate(elements);
}
extern "C" void vtk_perspective_transform_pre_multiply(vtkNew<vtkPerspectiveTransform> self) {
    return self->PreMultiply();
}
extern "C" void vtk_perspective_transform_post_multiply(vtkNew<vtkPerspectiveTransform> self) {
    return self->PostMultiply();
}
extern "C" int vtk_perspective_transform_get_number_of_concatenated_transforms(
    vtkNew<vtkPerspectiveTransform> self
) {
    return self->GetNumberOfConcatenatedTransforms();
}
extern "C" int vtk_perspective_transform_get_inverse_flag(vtkNew<vtkPerspectiveTransform> self) {
    return self->GetInverseFlag();
}
extern "C" void vtk_perspective_transform_push(vtkNew<vtkPerspectiveTransform> self) {
    return self->Push();
}
extern "C" void vtk_perspective_transform_pop(vtkNew<vtkPerspectiveTransform> self) {
    return self->Pop();
}
extern "C" unsigned long vtk_perspective_transform_get_m_time(
    vtkNew<vtkPerspectiveTransform> self
) {
    return self->GetMTime();
}
extern "C" vtkNew<vtkSphericalTransform> vtkSphericalTransform_new() {
    return vtkNew<vtkSphericalTransform>();
}
extern "C" void vtkSphericalTransform_destructor(vtkNew<vtkSphericalTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkSphericalTransform_get_ptr(vtkNew<vtkSphericalTransform> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkThinPlateSplineTransform> vtkThinPlateSplineTransform_new() {
    return vtkNew<vtkThinPlateSplineTransform>();
}
extern "C" void vtkThinPlateSplineTransform_destructor(vtkNew<vtkThinPlateSplineTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkThinPlateSplineTransform_get_ptr(vtkNew<vtkThinPlateSplineTransform> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_thin_plate_spline_transform_get_sigma(
    vtkNew<vtkThinPlateSplineTransform> self
) {
    return self->GetSigma();
}
extern "C" void vtk_thin_plate_spline_transform_set_sigma(
    vtkNew<vtkThinPlateSplineTransform> self, double _arg
) {
    return self->SetSigma(_arg);
}
extern "C" void vtk_thin_plate_spline_transform_set_basis(
    vtkNew<vtkThinPlateSplineTransform> self, int basis
) {
    return self->SetBasis(basis);
}
extern "C" int vtk_thin_plate_spline_transform_get_basis(vtkNew<vtkThinPlateSplineTransform> self) {
    return self->GetBasis();
}
extern "C" void vtk_thin_plate_spline_transform_set_basis_to_r(
    vtkNew<vtkThinPlateSplineTransform> self
) {
    return self->SetBasisToR();
}
extern "C" void vtk_thin_plate_spline_transform_set_basis_to_r_2_log_r(
    vtkNew<vtkThinPlateSplineTransform> self
) {
    return self->SetBasisToR2LogR();
}
extern "C" unsigned long vtk_thin_plate_spline_transform_get_m_time(
    vtkNew<vtkThinPlateSplineTransform> self
) {
    return self->GetMTime();
}
extern "C" bool vtk_thin_plate_spline_transform_get_regularize_bulk_transform(
    vtkNew<vtkThinPlateSplineTransform> self
) {
    return self->GetRegularizeBulkTransform();
}
extern "C" void vtk_thin_plate_spline_transform_set_regularize_bulk_transform(
    vtkNew<vtkThinPlateSplineTransform> self, bool _arg
) {
    return self->SetRegularizeBulkTransform(_arg);
}
extern "C" void vtk_thin_plate_spline_transform_regularize_bulk_transform_on(
    vtkNew<vtkThinPlateSplineTransform> self
) {
    return self->RegularizeBulkTransformOn();
}
extern "C" void vtk_thin_plate_spline_transform_regularize_bulk_transform_off(
    vtkNew<vtkThinPlateSplineTransform> self
) {
    return self->RegularizeBulkTransformOff();
}
extern "C" vtkNew<vtkTransform> vtkTransform_new() {
    return vtkNew<vtkTransform>();
}
extern "C" void vtkTransform_destructor(vtkNew<vtkTransform> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTransform_get_ptr(vtkNew<vtkTransform> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_transform_identity(vtkNew<vtkTransform> self) {
    return self->Identity();
}
extern "C" void vtk_transform_inverse(vtkNew<vtkTransform> self) {
    return self->Inverse();
}
extern "C" void vtk_transform_translate_double_double_double(
    vtkNew<vtkTransform> self, double x, double y, double z
) {
    return self->Translate(x, y, z);
}
extern "C" void vtk_transform_translate_const_double_3(vtkNew<vtkTransform> self, const double* x) {
    return self->Translate(x);
}
extern "C" void vtk_transform_translate_const_float_3(vtkNew<vtkTransform> self, const float* x) {
    return self->Translate(x);
}
extern "C" void vtk_transform_rotate_wxyz_double_double_double_double(
    vtkNew<vtkTransform> self, double angle, double x, double y, double z
) {
    return self->RotateWXYZ(angle, x, y, z);
}
extern "C" void vtk_transform_rotate_wxyz_double_const_double_3(
    vtkNew<vtkTransform> self, double angle, const double* axis
) {
    return self->RotateWXYZ(angle, axis);
}
extern "C" void vtk_transform_rotate_wxyz_double_const_float_3(
    vtkNew<vtkTransform> self, double angle, const float* axis
) {
    return self->RotateWXYZ(angle, axis);
}
extern "C" void vtk_transform_rotate_x(vtkNew<vtkTransform> self, double angle) {
    return self->RotateX(angle);
}
extern "C" void vtk_transform_rotate_y(vtkNew<vtkTransform> self, double angle) {
    return self->RotateY(angle);
}
extern "C" void vtk_transform_rotate_z(vtkNew<vtkTransform> self, double angle) {
    return self->RotateZ(angle);
}
extern "C" void vtk_transform_scale_double_double_double(
    vtkNew<vtkTransform> self, double x, double y, double z
) {
    return self->Scale(x, y, z);
}
extern "C" void vtk_transform_scale_const_double_3(vtkNew<vtkTransform> self, const double* s) {
    return self->Scale(s);
}
extern "C" void vtk_transform_scale_const_float_3(vtkNew<vtkTransform> self, const float* s) {
    return self->Scale(s);
}
extern "C" void vtk_transform_set_matrix_const_double_16(
    vtkNew<vtkTransform> self, const double* elements
) {
    return self->SetMatrix(elements);
}
extern "C" void vtk_transform_concatenate_const_double_16(
    vtkNew<vtkTransform> self, const double* elements
) {
    return self->Concatenate(elements);
}
extern "C" void vtk_transform_pre_multiply(vtkNew<vtkTransform> self) {
    return self->PreMultiply();
}
extern "C" void vtk_transform_post_multiply(vtkNew<vtkTransform> self) {
    return self->PostMultiply();
}
extern "C" int vtk_transform_get_number_of_concatenated_transforms(vtkNew<vtkTransform> self) {
    return self->GetNumberOfConcatenatedTransforms();
}
extern "C" void vtk_transform_get_orientation_double_3(vtkNew<vtkTransform> self, double* orient) {
    return self->GetOrientation(orient);
}
extern "C" void vtk_transform_get_orientation_float_3(vtkNew<vtkTransform> self, float* orient) {
    return self->GetOrientation(orient);
}
extern "C" void vtk_transform_get_orientation_wxyz_double_4(
    vtkNew<vtkTransform> self, double* wxyz
) {
    return self->GetOrientationWXYZ(wxyz);
}
extern "C" void vtk_transform_get_orientation_wxyz_float_4(vtkNew<vtkTransform> self, float* wxyz) {
    return self->GetOrientationWXYZ(wxyz);
}
extern "C" void vtk_transform_get_position_double_3(vtkNew<vtkTransform> self, double* pos) {
    return self->GetPosition(pos);
}
extern "C" void vtk_transform_get_position_float_3(vtkNew<vtkTransform> self, float* pos) {
    return self->GetPosition(pos);
}
extern "C" void vtk_transform_get_scale_double_3(vtkNew<vtkTransform> self, double* scale) {
    return self->GetScale(scale);
}
extern "C" void vtk_transform_get_scale_float_3(vtkNew<vtkTransform> self, float* scale) {
    return self->GetScale(scale);
}
extern "C" int vtk_transform_get_inverse_flag(vtkNew<vtkTransform> self) {
    return self->GetInverseFlag();
}
extern "C" void vtk_transform_push(vtkNew<vtkTransform> self) {
    return self->Push();
}
extern "C" void vtk_transform_pop(vtkNew<vtkTransform> self) {
    return self->Pop();
}
extern "C" unsigned long vtk_transform_get_m_time(vtkNew<vtkTransform> self) {
    return self->GetMTime();
}
extern "C" void vtk_transform_multiply_point_const_float_4_float_4(
    vtkNew<vtkTransform> self, const float* in, float* out
) {
    return self->MultiplyPoint(in, out);
}
extern "C" void vtk_transform_multiply_point_const_double_4_double_4(
    vtkNew<vtkTransform> self, const double* in, double* out
) {
    return self->MultiplyPoint(in, out);
}
extern "C" vtkNew<vtkTransform2D> vtkTransform2D_new() {
    return vtkNew<vtkTransform2D>();
}
extern "C" void vtkTransform2D_destructor(vtkNew<vtkTransform2D> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTransform2D_get_ptr(vtkNew<vtkTransform2D> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_transform_2_d_identity(vtkNew<vtkTransform2D> self) {
    return self->Identity();
}
extern "C" void vtk_transform_2_d_inverse(vtkNew<vtkTransform2D> self) {
    return self->Inverse();
}
extern "C" void vtk_transform_2_d_translate_double_double(
    vtkNew<vtkTransform2D> self, double x, double y
) {
    return self->Translate(x, y);
}
extern "C" void vtk_transform_2_d_translate_const_double_2(
    vtkNew<vtkTransform2D> self, const double* x
) {
    return self->Translate(x);
}
extern "C" void vtk_transform_2_d_translate_const_float_2(
    vtkNew<vtkTransform2D> self, const float* x
) {
    return self->Translate(x);
}
extern "C" void vtk_transform_2_d_rotate(vtkNew<vtkTransform2D> self, double angle) {
    return self->Rotate(angle);
}
extern "C" void vtk_transform_2_d_scale_double_double(
    vtkNew<vtkTransform2D> self, double x, double y
) {
    return self->Scale(x, y);
}
extern "C" void vtk_transform_2_d_scale_const_double_2(
    vtkNew<vtkTransform2D> self, const double* s
) {
    return self->Scale(s);
}
extern "C" void vtk_transform_2_d_scale_const_float_2(vtkNew<vtkTransform2D> self, const float* s) {
    return self->Scale(s);
}
extern "C" void vtk_transform_2_d_set_matrix_const_double_9(
    vtkNew<vtkTransform2D> self, const double* elements
) {
    return self->SetMatrix(elements);
}
extern "C" void vtk_transform_2_d_get_position_double_2(vtkNew<vtkTransform2D> self, double* pos) {
    return self->GetPosition(pos);
}
extern "C" void vtk_transform_2_d_get_position_float_2(vtkNew<vtkTransform2D> self, float* pos) {
    return self->GetPosition(pos);
}
extern "C" void vtk_transform_2_d_get_scale_double_2(vtkNew<vtkTransform2D> self, double* scale) {
    return self->GetScale(scale);
}
extern "C" void vtk_transform_2_d_get_scale_float_2(vtkNew<vtkTransform2D> self, float* pos) {
    return self->GetScale(pos);
}
extern "C" unsigned long vtk_transform_2_d_get_m_time(vtkNew<vtkTransform2D> self) {
    return self->GetMTime();
}
extern "C" void vtk_transform_2_d_multiply_point_const_float_3_float_3(
    vtkNew<vtkTransform2D> self, const float* in, float* out
) {
    return self->MultiplyPoint(in, out);
}
extern "C" void vtk_transform_2_d_multiply_point_const_double_3_double_3(
    vtkNew<vtkTransform2D> self, const double* in, double* out
) {
    return self->MultiplyPoint(in, out);
}
extern "C" vtkNew<vtkTransformCollection> vtkTransformCollection_new() {
    return vtkNew<vtkTransformCollection>();
}
extern "C" void vtkTransformCollection_destructor(vtkNew<vtkTransformCollection> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkTransformCollection_get_ptr(vtkNew<vtkTransformCollection> sself) {
    return sself.GetPointer();
}
