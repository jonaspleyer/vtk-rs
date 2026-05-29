// Include header file
#include<vtk_common_math.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkAmoebaMinimizer.h>
#include<vtkFFT.h>
#include<vtkFunctionSet.h>
#include<vtkInitialValueProblemSolver.h>
#include<vtkMatrix3x3.h>
#include<vtkMatrix4x4.h>
#include<vtkPolynomialSolversUnivariate.h>
#include<vtkQuaternion.h>
#include<vtkQuaternionInterpolator.h>
#include<vtkQuaternion.h>
#include<vtkQuaternion.h>
#include<vtkRungeKutta2.h>
#include<vtkRungeKutta4.h>
#include<vtkRungeKutta45.h>
#include<vtkTuple.h>

// Implement declared functions
extern "C" vtkAmoebaMinimizer * vtkAmoebaMinimizer_new () {return vtkAmoebaMinimizer :: New () ;}
extern "C" void vtkAmoebaMinimizer_destructor (vtkAmoebaMinimizer * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_amoeba_minimizer_set_parameter_value(vtkAmoebaMinimizer* sself, const char* name, double value) { sself->SetParameterValue(name, value); }
extern "C" void vtk_amoeba_minimizer_set_parameter_scale(vtkAmoebaMinimizer* sself, const char* name, double scale) { sself->SetParameterScale(name, scale); }
extern "C" double vtk_amoeba_minimizer_get_parameter_scale(vtkAmoebaMinimizer* sself, const char* name) { return sself->GetParameterScale(name); }
extern "C" double vtk_amoeba_minimizer_get_parameter_value(vtkAmoebaMinimizer* sself, const char* name) { return sself->GetParameterValue(name); }
extern "C" const char* vtk_amoeba_minimizer_get_parameter_name(vtkAmoebaMinimizer* sself, int i) { return sself->GetParameterName(i); }
extern "C" int vtk_amoeba_minimizer_get_number_of_parameters(vtkAmoebaMinimizer* sself) { return sself->GetNumberOfParameters(); }
extern "C" void vtk_amoeba_minimizer_initialize(vtkAmoebaMinimizer* sself) { sself->Initialize(); }
extern "C" void vtk_amoeba_minimizer_minimize(vtkAmoebaMinimizer* sself) { sself->Minimize(); }
extern "C" int vtk_amoeba_minimizer_iterate(vtkAmoebaMinimizer* sself) { return sself->Iterate(); }
extern "C" void vtk_amoeba_minimizer_set_function_value(vtkAmoebaMinimizer* sself, double _arg) { sself->SetFunctionValue(_arg); }
extern "C" double vtk_amoeba_minimizer_get_function_value(vtkAmoebaMinimizer* sself) { return sself->GetFunctionValue(); }
extern "C" void vtk_amoeba_minimizer_set_contraction_ratio(vtkAmoebaMinimizer* sself, double _arg) { sself->SetContractionRatio(_arg); }
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio_min_value(vtkAmoebaMinimizer* sself) { return sself->GetContractionRatioMinValue(); }
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio_max_value(vtkAmoebaMinimizer* sself) { return sself->GetContractionRatioMaxValue(); }
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio(vtkAmoebaMinimizer* sself) { return sself->GetContractionRatio(); }
extern "C" void vtk_amoeba_minimizer_set_expansion_ratio(vtkAmoebaMinimizer* sself, double _arg) { sself->SetExpansionRatio(_arg); }
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio_min_value(vtkAmoebaMinimizer* sself) { return sself->GetExpansionRatioMinValue(); }
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio_max_value(vtkAmoebaMinimizer* sself) { return sself->GetExpansionRatioMaxValue(); }
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio(vtkAmoebaMinimizer* sself) { return sself->GetExpansionRatio(); }
extern "C" void vtk_amoeba_minimizer_set_tolerance(vtkAmoebaMinimizer* sself, double _arg) { sself->SetTolerance(_arg); }
extern "C" double vtk_amoeba_minimizer_get_tolerance(vtkAmoebaMinimizer* sself) { return sself->GetTolerance(); }
extern "C" void vtk_amoeba_minimizer_set_parameter_tolerance(vtkAmoebaMinimizer* sself, double _arg) { sself->SetParameterTolerance(_arg); }
extern "C" double vtk_amoeba_minimizer_get_parameter_tolerance(vtkAmoebaMinimizer* sself) { return sself->GetParameterTolerance(); }
extern "C" void vtk_amoeba_minimizer_set_max_iterations(vtkAmoebaMinimizer* sself, int _arg) { sself->SetMaxIterations(_arg); }
extern "C" int vtk_amoeba_minimizer_get_max_iterations(vtkAmoebaMinimizer* sself) { return sself->GetMaxIterations(); }
extern "C" int vtk_amoeba_minimizer_get_iterations(vtkAmoebaMinimizer* sself) { return sself->GetIterations(); }
extern "C" int vtk_amoeba_minimizer_get_function_evaluations(vtkAmoebaMinimizer* sself) { return sself->GetFunctionEvaluations(); }
extern "C" void vtk_amoeba_minimizer_evaluate_function(vtkAmoebaMinimizer* sself) { sself->EvaluateFunction(); }
extern "C" vtkFFT * vtkFFT_new () {return vtkFFT :: New () ;}
extern "C" void vtkFFT_destructor (vtkFFT * sself) {sself -> Delete () ; return ;}
extern "C" double vtk_fft_hanning_generator(vtkFFT* sself, const size_t x, const size_t size) { return sself->HanningGenerator(x, size); }
extern "C" double vtk_fft_bartlett_generator(vtkFFT* sself, const size_t x, const size_t size) { return sself->BartlettGenerator(x, size); }
extern "C" double vtk_fft_sine_generator(vtkFFT* sself, const size_t x, const size_t size) { return sself->SineGenerator(x, size); }
extern "C" double vtk_fft_blackman_generator(vtkFFT* sself, const size_t x, const size_t size) { return sself->BlackmanGenerator(x, size); }
extern "C" double vtk_fft_rectangular_generator(vtkFFT* sself, const size_t x, const size_t size) { return sself->RectangularGenerator(x, size); }
extern "C" vtkMatrix3x3 * vtkMatrix3x3_new () {return vtkMatrix3x3 :: New () ;}
extern "C" void vtkMatrix3x3_destructor (vtkMatrix3x3 * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_matrix_3_x_3_zero(vtkMatrix3x3* sself) { sself->Zero(); }
extern "C" void vtk_matrix_3_x_3_identity(vtkMatrix3x3* sself) { sself->Identity(); }
extern "C" double vtk_matrix_3_x_3_determinant(vtkMatrix3x3* sself) { return sself->Determinant(); }
extern "C" void vtk_matrix_3_x_3_set_element(vtkMatrix3x3* sself, int i, int j, double value) { sself->SetElement(i, j, value); }
extern "C" double vtk_matrix_3_x_3_get_element(vtkMatrix3x3* sself, int i, int j) { return sself->GetElement(i, j); }
extern "C" bool vtk_matrix_3_x_3_is_identity(vtkMatrix3x3* sself) { return sself->IsIdentity(); }
extern "C" vtkMatrix4x4 * vtkMatrix4x4_new () {return vtkMatrix4x4 :: New () ;}
extern "C" void vtkMatrix4x4_destructor (vtkMatrix4x4 * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_matrix_4_x_4_zero(vtkMatrix4x4* sself) { sself->Zero(); }
extern "C" void vtk_matrix_4_x_4_identity(vtkMatrix4x4* sself) { sself->Identity(); }
extern "C" bool vtk_matrix_4_x_4_is_identity(vtkMatrix4x4* sself) { return sself->IsIdentity(); }
extern "C" double vtk_matrix_4_x_4_determinant(vtkMatrix4x4* sself) { return sself->Determinant(); }
extern "C" void vtk_matrix_4_x_4_set_element(vtkMatrix4x4* sself, int i, int j, double value) { sself->SetElement(i, j, value); }
extern "C" double vtk_matrix_4_x_4_get_element(vtkMatrix4x4* sself, int i, int j) { return sself->GetElement(i, j); }
extern "C" vtkPolynomialSolversUnivariate * vtkPolynomialSolversUnivariate_new () {return vtkPolynomialSolversUnivariate :: New () ;}
extern "C" void vtkPolynomialSolversUnivariate_destructor (vtkPolynomialSolversUnivariate * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_polynomial_solvers_univariate_set_division_tolerance(vtkPolynomialSolversUnivariate* sself, double tol) { sself->SetDivisionTolerance(tol); }
extern "C" double vtk_polynomial_solvers_univariate_get_division_tolerance(vtkPolynomialSolversUnivariate* sself) { return sself->GetDivisionTolerance(); }
extern "C" vtkQuaternionInterpolator * vtkQuaternionInterpolator_new () {return vtkQuaternionInterpolator :: New () ;}
extern "C" void vtkQuaternionInterpolator_destructor (vtkQuaternionInterpolator * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_quaternion_interpolator_get_number_of_quaternions(vtkQuaternionInterpolator* sself) { return sself->GetNumberOfQuaternions(); }
extern "C" double vtk_quaternion_interpolator_get_minimum_t(vtkQuaternionInterpolator* sself) { return sself->GetMinimumT(); }
extern "C" double vtk_quaternion_interpolator_get_maximum_t(vtkQuaternionInterpolator* sself) { return sself->GetMaximumT(); }
extern "C" void vtk_quaternion_interpolator_initialize(vtkQuaternionInterpolator* sself) { sself->Initialize(); }
extern "C" void vtk_quaternion_interpolator_remove_quaternion(vtkQuaternionInterpolator* sself, double t) { sself->RemoveQuaternion(t); }
extern "C" int vtk_quaternion_interpolator_get_search_method(vtkQuaternionInterpolator* sself) { return sself->GetSearchMethod(); }
extern "C" void vtk_quaternion_interpolator_set_search_method(vtkQuaternionInterpolator* sself, int type) { sself->SetSearchMethod(type); }
extern "C" void vtk_quaternion_interpolator_set_interpolation_type(vtkQuaternionInterpolator* sself, int _arg) { sself->SetInterpolationType(_arg); }
extern "C" int vtk_quaternion_interpolator_get_interpolation_type_min_value(vtkQuaternionInterpolator* sself) { return sself->GetInterpolationTypeMinValue(); }
extern "C" int vtk_quaternion_interpolator_get_interpolation_type_max_value(vtkQuaternionInterpolator* sself) { return sself->GetInterpolationTypeMaxValue(); }
extern "C" int vtk_quaternion_interpolator_get_interpolation_type(vtkQuaternionInterpolator* sself) { return sself->GetInterpolationType(); }
extern "C" void vtk_quaternion_interpolator_set_interpolation_type_to_linear(vtkQuaternionInterpolator* sself) { sself->SetInterpolationTypeToLinear(); }
extern "C" void vtk_quaternion_interpolator_set_interpolation_type_to_spline(vtkQuaternionInterpolator* sself) { sself->SetInterpolationTypeToSpline(); }
extern "C" vtkRungeKutta2 * vtkRungeKutta2_new () {return vtkRungeKutta2 :: New () ;}
extern "C" void vtkRungeKutta2_destructor (vtkRungeKutta2 * sself) {sself -> Delete () ; return ;}
extern "C" vtkRungeKutta4 * vtkRungeKutta4_new () {return vtkRungeKutta4 :: New () ;}
extern "C" void vtkRungeKutta4_destructor (vtkRungeKutta4 * sself) {sself -> Delete () ; return ;}
extern "C" vtkRungeKutta45 * vtkRungeKutta45_new () {return vtkRungeKutta45 :: New () ;}
extern "C" void vtkRungeKutta45_destructor (vtkRungeKutta45 * sself) {sself -> Delete () ; return ;}
