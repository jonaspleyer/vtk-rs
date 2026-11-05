// Include header file
#include <vtk_common_math.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkAmoebaMinimizer.h>
#include <vtkFFT.h>
#include <vtkFunctionSet.h>
#include <vtkInitialValueProblemSolver.h>
#include <vtkMatrix3x3.h>
#include <vtkMatrix4x4.h>
#include <vtkPolynomialSolversUnivariate.h>
#include <vtkQuaternion.h>
#include <vtkQuaternionInterpolator.h>
#include <vtkRungeKutta2.h>
#include <vtkRungeKutta4.h>
#include <vtkRungeKutta45.h>
#include <vtkTuple.h>

// Implement declared functions
extern "C" vtkNew<vtkAmoebaMinimizer> vtkAmoebaMinimizer_new() {
    return vtkNew<vtkAmoebaMinimizer>();
}
extern "C" void vtkAmoebaMinimizer_destructor(vtkNew<vtkAmoebaMinimizer> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkAmoebaMinimizer_get_ptr(vtkNew<vtkAmoebaMinimizer> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_amoeba_minimizer_set_parameter_value_c_int_double(
    vtkNew<vtkAmoebaMinimizer> self, int i, double value
) {
    return self->SetParameterValue(i, value);
}
extern "C" void vtk_amoeba_minimizer_set_parameter_scale_c_int_double(
    vtkNew<vtkAmoebaMinimizer> self, int i, double scale
) {
    return self->SetParameterScale(i, scale);
}
extern "C" double vtk_amoeba_minimizer_get_parameter_scale_c_int(
    vtkNew<vtkAmoebaMinimizer> self, int i
) {
    return self->GetParameterScale(i);
}
extern "C" double vtk_amoeba_minimizer_get_parameter_value_c_int(
    vtkNew<vtkAmoebaMinimizer> self, int i
) {
    return self->GetParameterValue(i);
}
extern "C" int vtk_amoeba_minimizer_get_number_of_parameters(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetNumberOfParameters();
}
extern "C" void vtk_amoeba_minimizer_initialize(vtkNew<vtkAmoebaMinimizer> self) {
    return self->Initialize();
}
extern "C" void vtk_amoeba_minimizer_minimize(vtkNew<vtkAmoebaMinimizer> self) {
    return self->Minimize();
}
extern "C" int vtk_amoeba_minimizer_iterate(vtkNew<vtkAmoebaMinimizer> self) {
    return self->Iterate();
}
extern "C" void vtk_amoeba_minimizer_set_function_value(
    vtkNew<vtkAmoebaMinimizer> self, double _arg
) {
    return self->SetFunctionValue(_arg);
}
extern "C" double vtk_amoeba_minimizer_get_function_value(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetFunctionValue();
}
extern "C" void vtk_amoeba_minimizer_set_contraction_ratio(
    vtkNew<vtkAmoebaMinimizer> self, double _arg
) {
    return self->SetContractionRatio(_arg);
}
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio_min_value(
    vtkNew<vtkAmoebaMinimizer> self
) {
    return self->GetContractionRatioMinValue();
}
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio_max_value(
    vtkNew<vtkAmoebaMinimizer> self
) {
    return self->GetContractionRatioMaxValue();
}
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetContractionRatio();
}
extern "C" void vtk_amoeba_minimizer_set_expansion_ratio(
    vtkNew<vtkAmoebaMinimizer> self, double _arg
) {
    return self->SetExpansionRatio(_arg);
}
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio_min_value(
    vtkNew<vtkAmoebaMinimizer> self
) {
    return self->GetExpansionRatioMinValue();
}
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio_max_value(
    vtkNew<vtkAmoebaMinimizer> self
) {
    return self->GetExpansionRatioMaxValue();
}
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetExpansionRatio();
}
extern "C" void vtk_amoeba_minimizer_set_tolerance(vtkNew<vtkAmoebaMinimizer> self, double _arg) {
    return self->SetTolerance(_arg);
}
extern "C" double vtk_amoeba_minimizer_get_tolerance(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetTolerance();
}
extern "C" void vtk_amoeba_minimizer_set_parameter_tolerance(
    vtkNew<vtkAmoebaMinimizer> self, double _arg
) {
    return self->SetParameterTolerance(_arg);
}
extern "C" double vtk_amoeba_minimizer_get_parameter_tolerance(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetParameterTolerance();
}
extern "C" void vtk_amoeba_minimizer_set_max_iterations(vtkNew<vtkAmoebaMinimizer> self, int _arg) {
    return self->SetMaxIterations(_arg);
}
extern "C" int vtk_amoeba_minimizer_get_max_iterations(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetMaxIterations();
}
extern "C" int vtk_amoeba_minimizer_get_iterations(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetIterations();
}
extern "C" int vtk_amoeba_minimizer_get_function_evaluations(vtkNew<vtkAmoebaMinimizer> self) {
    return self->GetFunctionEvaluations();
}
extern "C" void vtk_amoeba_minimizer_evaluate_function(vtkNew<vtkAmoebaMinimizer> self) {
    return self->EvaluateFunction();
}
extern "C" vtkNew<vtkFFT> vtkFFT_new() {
    return vtkNew<vtkFFT>();
}
extern "C" void vtkFFT_destructor(vtkNew<vtkFFT> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkFFT_get_ptr(vtkNew<vtkFFT> sself) {
    return sself.GetPointer();
}
extern "C" double vtk_fft_hanning_generator(vtkNew<vtkFFT> self, size_t x, size_t size) {
    return self->HanningGenerator(x, size);
}
extern "C" double vtk_fft_bartlett_generator(vtkNew<vtkFFT> self, size_t x, size_t size) {
    return self->BartlettGenerator(x, size);
}
extern "C" double vtk_fft_sine_generator(vtkNew<vtkFFT> self, size_t x, size_t size) {
    return self->SineGenerator(x, size);
}
extern "C" double vtk_fft_blackman_generator(vtkNew<vtkFFT> self, size_t x, size_t size) {
    return self->BlackmanGenerator(x, size);
}
extern "C" double vtk_fft_rectangular_generator(vtkNew<vtkFFT> self, size_t x, size_t size) {
    return self->RectangularGenerator(x, size);
}
extern "C" vtkNew<vtkMatrix3x3> vtkMatrix3x3_new() {
    return vtkNew<vtkMatrix3x3>();
}
extern "C" void vtkMatrix3x3_destructor(vtkNew<vtkMatrix3x3> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMatrix3x3_get_ptr(vtkNew<vtkMatrix3x3> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_matrix_3_x_3_deep_copy_double_9_const_double_9(
    vtkNew<vtkMatrix3x3> self, double* elements, const double* newElements
) {
    return self->DeepCopy(elements, newElements);
}
extern "C" void vtk_matrix_3_x_3_deep_copy_const_double_9(
    vtkNew<vtkMatrix3x3> self, const double* elements
) {
    return self->DeepCopy(elements);
}
extern "C" void vtk_matrix_3_x_3_zero(vtkNew<vtkMatrix3x3> self) {
    return self->Zero();
}
extern "C" void vtk_matrix_3_x_3_zero_double_9(vtkNew<vtkMatrix3x3> self, double* elements) {
    return self->Zero(elements);
}
extern "C" void vtk_matrix_3_x_3_identity(vtkNew<vtkMatrix3x3> self) {
    return self->Identity();
}
extern "C" void vtk_matrix_3_x_3_identity_double_9(vtkNew<vtkMatrix3x3> self, double* elements) {
    return self->Identity(elements);
}
extern "C" void vtk_matrix_3_x_3_invert(vtkNew<vtkMatrix3x3> self) {
    return self->Invert();
}
extern "C" void vtk_matrix_3_x_3_invert_const_double_9_double_9(
    vtkNew<vtkMatrix3x3> self, const double* inElements, double* outElements
) {
    return self->Invert(inElements, outElements);
}
extern "C" void vtk_matrix_3_x_3_transpose(vtkNew<vtkMatrix3x3> self) {
    return self->Transpose();
}
extern "C" void vtk_matrix_3_x_3_transpose_const_double_9_double_9(
    vtkNew<vtkMatrix3x3> self, const double* inElements, double* outElements
) {
    return self->Transpose(inElements, outElements);
}
extern "C" void vtk_matrix_3_x_3_multiply_point_const_float_3_float_3(
    vtkNew<vtkMatrix3x3> self, const float* in, float* out
) {
    return self->MultiplyPoint(in, out);
}
extern "C" void vtk_matrix_3_x_3_multiply_point_const_double_3_double_3(
    vtkNew<vtkMatrix3x3> self, const double* in, double* out
) {
    return self->MultiplyPoint(in, out);
}
extern "C" void vtk_matrix_3_x_3_multiply_point_const_double_9_const_float_3_float_3(
    vtkNew<vtkMatrix3x3> self, const double* elements, const float* in, float* out
) {
    return self->MultiplyPoint(elements, in, out);
}
extern "C" void vtk_matrix_3_x_3_multiply_point_const_double_9_const_double_3_double_3(
    vtkNew<vtkMatrix3x3> self, const double* elements, const double* in, double* out
) {
    return self->MultiplyPoint(elements, in, out);
}
extern "C" void vtk_matrix_3_x_3_multiply_3_x_3_const_double_9_const_double_9_double_9(
    vtkNew<vtkMatrix3x3> self, const double* a, const double* b, double* c
) {
    return self->Multiply3x3(a, b, c);
}
extern "C" void vtk_matrix_3_x_3_adjoint_const_double_9_double_9(
    vtkNew<vtkMatrix3x3> self, const double* inElements, double* outElements
) {
    return self->Adjoint(inElements, outElements);
}
extern "C" double vtk_matrix_3_x_3_determinant(vtkNew<vtkMatrix3x3> self) {
    return self->Determinant();
}
extern "C" double vtk_matrix_3_x_3_determinant_const_double_9(
    vtkNew<vtkMatrix3x3> self, const double* elements
) {
    return self->Determinant(elements);
}
extern "C" void vtk_matrix_3_x_3_set_element(
    vtkNew<vtkMatrix3x3> self, int i, int j, double value
) {
    return self->SetElement(i, j, value);
}
extern "C" double vtk_matrix_3_x_3_get_element(vtkNew<vtkMatrix3x3> self, int i, int j) {
    return self->GetElement(i, j);
}
extern "C" bool vtk_matrix_3_x_3_is_identity(vtkNew<vtkMatrix3x3> self) {
    return self->IsIdentity();
}
extern "C" void vtk_matrix_3_x_3_set_data(vtkNew<vtkMatrix3x3> self, const double* data) {
    return self->SetData(data);
}
extern "C" vtkNew<vtkMatrix4x4> vtkMatrix4x4_new() {
    return vtkNew<vtkMatrix4x4>();
}
extern "C" void vtkMatrix4x4_destructor(vtkNew<vtkMatrix4x4> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkMatrix4x4_get_ptr(vtkNew<vtkMatrix4x4> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_matrix_4_x_4_deep_copy_double_16_const_double_16(
    vtkNew<vtkMatrix4x4> self, double* destination, const double* source
) {
    return self->DeepCopy(destination, source);
}
extern "C" void vtk_matrix_4_x_4_deep_copy_const_double_16(
    vtkNew<vtkMatrix4x4> self, const double* elements
) {
    return self->DeepCopy(elements);
}
extern "C" void vtk_matrix_4_x_4_zero(vtkNew<vtkMatrix4x4> self) {
    return self->Zero();
}
extern "C" void vtk_matrix_4_x_4_zero_double_16(vtkNew<vtkMatrix4x4> self, double* elements) {
    return self->Zero(elements);
}
extern "C" void vtk_matrix_4_x_4_identity(vtkNew<vtkMatrix4x4> self) {
    return self->Identity();
}
extern "C" void vtk_matrix_4_x_4_identity_double_16(vtkNew<vtkMatrix4x4> self, double* elements) {
    return self->Identity(elements);
}
extern "C" bool vtk_matrix_4_x_4_is_identity(vtkNew<vtkMatrix4x4> self) {
    return self->IsIdentity();
}
extern "C" void vtk_matrix_4_x_4_invert(vtkNew<vtkMatrix4x4> self) {
    return self->Invert();
}
extern "C" void vtk_matrix_4_x_4_invert_const_double_16_double_16(
    vtkNew<vtkMatrix4x4> self, const double* inElements, double* outElements
) {
    return self->Invert(inElements, outElements);
}
extern "C" void vtk_matrix_4_x_4_transpose(vtkNew<vtkMatrix4x4> self) {
    return self->Transpose();
}
extern "C" void vtk_matrix_4_x_4_transpose_const_double_16_double_16(
    vtkNew<vtkMatrix4x4> self, const double* inElements, double* outElements
) {
    return self->Transpose(inElements, outElements);
}
extern "C" void vtk_matrix_4_x_4_matrix_from_rotation_double_double_double_double_double_16(
    vtkNew<vtkMatrix4x4> self, double angle, double x, double y, double z, double* matrix
) {
    return self->MatrixFromRotation(angle, x, y, z, matrix);
}
extern "C" void vtk_matrix_4_x_4_multiply_point_const_float_4_float_4(
    vtkNew<vtkMatrix4x4> self, const float* in, float* out
) {
    return self->MultiplyPoint(in, out);
}
extern "C" void vtk_matrix_4_x_4_multiply_point_const_double_4_double_4(
    vtkNew<vtkMatrix4x4> self, const double* in, double* out
) {
    return self->MultiplyPoint(in, out);
}
extern "C" void vtk_matrix_4_x_4_multiply_point_const_double_16_const_float_4_float_4(
    vtkNew<vtkMatrix4x4> self, const double* elements, const float* in, float* out
) {
    return self->MultiplyPoint(elements, in, out);
}
extern "C" void vtk_matrix_4_x_4_multiply_point_const_double_16_const_double_4_double_4(
    vtkNew<vtkMatrix4x4> self, const double* elements, const double* in, double* out
) {
    return self->MultiplyPoint(elements, in, out);
}
extern "C" void vtk_matrix_4_x_4_multiply_4_x_4_const_double_16_const_double_16_double_16(
    vtkNew<vtkMatrix4x4> self, const double* a, const double* b, double* c
) {
    return self->Multiply4x4(a, b, c);
}
extern "C" void vtk_matrix_4_x_4_multiply_4_x_4_const_double_16_const_double_16_float_16(
    vtkNew<vtkMatrix4x4> self, const double* a, const double* b, float* c
) {
    return self->Multiply4x4(a, b, c);
}
extern "C" void vtk_matrix_4_x_4_multiply_and_transpose_4_x_4(
    vtkNew<vtkMatrix4x4> self, const double* a, const double* b, float* c
) {
    return self->MultiplyAndTranspose4x4(a, b, c);
}
extern "C" void vtk_matrix_4_x_4_adjoint_const_double_16_double_16(
    vtkNew<vtkMatrix4x4> self, const double* inElements, double* outElements
) {
    return self->Adjoint(inElements, outElements);
}
extern "C" double vtk_matrix_4_x_4_determinant(vtkNew<vtkMatrix4x4> self) {
    return self->Determinant();
}
extern "C" double vtk_matrix_4_x_4_determinant_const_double_16(
    vtkNew<vtkMatrix4x4> self, const double* elements
) {
    return self->Determinant(elements);
}
extern "C" void vtk_matrix_4_x_4_set_element(
    vtkNew<vtkMatrix4x4> self, int i, int j, double value
) {
    return self->SetElement(i, j, value);
}
extern "C" double vtk_matrix_4_x_4_get_element(vtkNew<vtkMatrix4x4> self, int i, int j) {
    return self->GetElement(i, j);
}
extern "C" void vtk_matrix_4_x_4_set_data(vtkNew<vtkMatrix4x4> self, const double* data) {
    return self->SetData(data);
}
extern "C" vtkNew<vtkPolynomialSolversUnivariate> vtkPolynomialSolversUnivariate_new() {
    return vtkNew<vtkPolynomialSolversUnivariate>();
}
extern "C" void vtkPolynomialSolversUnivariate_destructor(
    vtkNew<vtkPolynomialSolversUnivariate> sself
) {
    sself.Reset();
    return;
}
extern "C" void* vtkPolynomialSolversUnivariate_get_ptr(
    vtkNew<vtkPolynomialSolversUnivariate> sself
) {
    return sself.GetPointer();
}
extern "C" void vtk_polynomial_solvers_univariate_set_division_tolerance(
    vtkNew<vtkPolynomialSolversUnivariate> self, double tol
) {
    return self->SetDivisionTolerance(tol);
}
extern "C" double vtk_polynomial_solvers_univariate_get_division_tolerance(
    vtkNew<vtkPolynomialSolversUnivariate> self
) {
    return self->GetDivisionTolerance();
}
extern "C" vtkNew<vtkQuaternionInterpolator> vtkQuaternionInterpolator_new() {
    return vtkNew<vtkQuaternionInterpolator>();
}
extern "C" void vtkQuaternionInterpolator_destructor(vtkNew<vtkQuaternionInterpolator> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkQuaternionInterpolator_get_ptr(vtkNew<vtkQuaternionInterpolator> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_quaternion_interpolator_get_number_of_quaternions(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->GetNumberOfQuaternions();
}
extern "C" double vtk_quaternion_interpolator_get_minimum_t(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->GetMinimumT();
}
extern "C" double vtk_quaternion_interpolator_get_maximum_t(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->GetMaximumT();
}
extern "C" void vtk_quaternion_interpolator_initialize(vtkNew<vtkQuaternionInterpolator> self) {
    return self->Initialize();
}
extern "C" void vtk_quaternion_interpolator_add_quaternion_double_double_4(
    vtkNew<vtkQuaternionInterpolator> self, double t, double* q
) {
    return self->AddQuaternion(t, q);
}
extern "C" void vtk_quaternion_interpolator_remove_quaternion(
    vtkNew<vtkQuaternionInterpolator> self, double t
) {
    return self->RemoveQuaternion(t);
}
extern "C" void vtk_quaternion_interpolator_interpolate_quaternion_double_double_4(
    vtkNew<vtkQuaternionInterpolator> self, double t, double* q
) {
    return self->InterpolateQuaternion(t, q);
}
extern "C" int vtk_quaternion_interpolator_get_search_method(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->GetSearchMethod();
}
extern "C" void vtk_quaternion_interpolator_set_search_method(
    vtkNew<vtkQuaternionInterpolator> self, int type
) {
    return self->SetSearchMethod(type);
}
extern "C" void vtk_quaternion_interpolator_set_interpolation_type(
    vtkNew<vtkQuaternionInterpolator> self, int _arg
) {
    return self->SetInterpolationType(_arg);
}
extern "C" int vtk_quaternion_interpolator_get_interpolation_type_min_value(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->GetInterpolationTypeMinValue();
}
extern "C" int vtk_quaternion_interpolator_get_interpolation_type_max_value(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->GetInterpolationTypeMaxValue();
}
extern "C" int vtk_quaternion_interpolator_get_interpolation_type(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->GetInterpolationType();
}
extern "C" void vtk_quaternion_interpolator_set_interpolation_type_to_linear(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->SetInterpolationTypeToLinear();
}
extern "C" void vtk_quaternion_interpolator_set_interpolation_type_to_spline(
    vtkNew<vtkQuaternionInterpolator> self
) {
    return self->SetInterpolationTypeToSpline();
}
extern "C" vtkNew<vtkRungeKutta2> vtkRungeKutta2_new() {
    return vtkNew<vtkRungeKutta2>();
}
extern "C" void vtkRungeKutta2_destructor(vtkNew<vtkRungeKutta2> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkRungeKutta2_get_ptr(vtkNew<vtkRungeKutta2> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkRungeKutta4> vtkRungeKutta4_new() {
    return vtkNew<vtkRungeKutta4>();
}
extern "C" void vtkRungeKutta4_destructor(vtkNew<vtkRungeKutta4> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkRungeKutta4_get_ptr(vtkNew<vtkRungeKutta4> sself) {
    return sself.GetPointer();
}
extern "C" vtkNew<vtkRungeKutta45> vtkRungeKutta45_new() {
    return vtkNew<vtkRungeKutta45>();
}
extern "C" void vtkRungeKutta45_destructor(vtkNew<vtkRungeKutta45> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkRungeKutta45_get_ptr(vtkNew<vtkRungeKutta45> sself) {
    return sself.GetPointer();
}
