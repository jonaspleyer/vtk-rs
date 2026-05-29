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

// Declare exported functions
extern "C" vtkAmoebaMinimizer * vtkAmoebaMinimizer_new () ;
extern "C" void vtkAmoebaMinimizer_destructor (vtkAmoebaMinimizer * sself) ;
extern "C" void vtk_amoeba_minimizer_set_parameter_value(vtkAmoebaMinimizer* sself, const char* name, double value);
extern "C" void vtk_amoeba_minimizer_set_parameter_scale(vtkAmoebaMinimizer* sself, const char* name, double scale);
extern "C" double vtk_amoeba_minimizer_get_parameter_scale(vtkAmoebaMinimizer* sself, const char* name);
extern "C" double vtk_amoeba_minimizer_get_parameter_value(vtkAmoebaMinimizer* sself, const char* name);
extern "C" const char* vtk_amoeba_minimizer_get_parameter_name(vtkAmoebaMinimizer* sself, int i);
extern "C" int vtk_amoeba_minimizer_get_number_of_parameters(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_initialize(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_minimize(vtkAmoebaMinimizer* sself);
extern "C" int vtk_amoeba_minimizer_iterate(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_set_function_value(vtkAmoebaMinimizer* sself, double _arg);
extern "C" double vtk_amoeba_minimizer_get_function_value(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_set_contraction_ratio(vtkAmoebaMinimizer* sself, double _arg);
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio_min_value(vtkAmoebaMinimizer* sself);
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio_max_value(vtkAmoebaMinimizer* sself);
extern "C" double vtk_amoeba_minimizer_get_contraction_ratio(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_set_expansion_ratio(vtkAmoebaMinimizer* sself, double _arg);
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio_min_value(vtkAmoebaMinimizer* sself);
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio_max_value(vtkAmoebaMinimizer* sself);
extern "C" double vtk_amoeba_minimizer_get_expansion_ratio(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_set_tolerance(vtkAmoebaMinimizer* sself, double _arg);
extern "C" double vtk_amoeba_minimizer_get_tolerance(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_set_parameter_tolerance(vtkAmoebaMinimizer* sself, double _arg);
extern "C" double vtk_amoeba_minimizer_get_parameter_tolerance(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_set_max_iterations(vtkAmoebaMinimizer* sself, int _arg);
extern "C" int vtk_amoeba_minimizer_get_max_iterations(vtkAmoebaMinimizer* sself);
extern "C" int vtk_amoeba_minimizer_get_iterations(vtkAmoebaMinimizer* sself);
extern "C" int vtk_amoeba_minimizer_get_function_evaluations(vtkAmoebaMinimizer* sself);
extern "C" void vtk_amoeba_minimizer_evaluate_function(vtkAmoebaMinimizer* sself);
extern "C" vtkFFT * vtkFFT_new () ;
extern "C" void vtkFFT_destructor (vtkFFT * sself) ;
extern "C" double vtk_fft_hanning_generator(vtkFFT* sself, const size_t x, const size_t size);
extern "C" double vtk_fft_bartlett_generator(vtkFFT* sself, const size_t x, const size_t size);
extern "C" double vtk_fft_sine_generator(vtkFFT* sself, const size_t x, const size_t size);
extern "C" double vtk_fft_blackman_generator(vtkFFT* sself, const size_t x, const size_t size);
extern "C" double vtk_fft_rectangular_generator(vtkFFT* sself, const size_t x, const size_t size);
extern "C" vtkMatrix3x3 * vtkMatrix3x3_new () ;
extern "C" void vtkMatrix3x3_destructor (vtkMatrix3x3 * sself) ;
extern "C" void vtk_matrix_3_x_3_zero(vtkMatrix3x3* sself);
extern "C" void vtk_matrix_3_x_3_identity(vtkMatrix3x3* sself);
extern "C" double vtk_matrix_3_x_3_determinant(vtkMatrix3x3* sself);
extern "C" void vtk_matrix_3_x_3_set_element(vtkMatrix3x3* sself, int i, int j, double value);
extern "C" double vtk_matrix_3_x_3_get_element(vtkMatrix3x3* sself, int i, int j);
extern "C" bool vtk_matrix_3_x_3_is_identity(vtkMatrix3x3* sself);
extern "C" vtkMatrix4x4 * vtkMatrix4x4_new () ;
extern "C" void vtkMatrix4x4_destructor (vtkMatrix4x4 * sself) ;
extern "C" void vtk_matrix_4_x_4_zero(vtkMatrix4x4* sself);
extern "C" void vtk_matrix_4_x_4_identity(vtkMatrix4x4* sself);
extern "C" bool vtk_matrix_4_x_4_is_identity(vtkMatrix4x4* sself);
extern "C" double vtk_matrix_4_x_4_determinant(vtkMatrix4x4* sself);
extern "C" void vtk_matrix_4_x_4_set_element(vtkMatrix4x4* sself, int i, int j, double value);
extern "C" double vtk_matrix_4_x_4_get_element(vtkMatrix4x4* sself, int i, int j);
extern "C" vtkPolynomialSolversUnivariate * vtkPolynomialSolversUnivariate_new () ;
extern "C" void vtkPolynomialSolversUnivariate_destructor (vtkPolynomialSolversUnivariate * sself) ;
extern "C" void vtk_polynomial_solvers_univariate_set_division_tolerance(vtkPolynomialSolversUnivariate* sself, double tol);
extern "C" double vtk_polynomial_solvers_univariate_get_division_tolerance(vtkPolynomialSolversUnivariate* sself);
extern "C" vtkQuaternionInterpolator * vtkQuaternionInterpolator_new () ;
extern "C" void vtkQuaternionInterpolator_destructor (vtkQuaternionInterpolator * sself) ;
extern "C" int vtk_quaternion_interpolator_get_number_of_quaternions(vtkQuaternionInterpolator* sself);
extern "C" double vtk_quaternion_interpolator_get_minimum_t(vtkQuaternionInterpolator* sself);
extern "C" double vtk_quaternion_interpolator_get_maximum_t(vtkQuaternionInterpolator* sself);
extern "C" void vtk_quaternion_interpolator_initialize(vtkQuaternionInterpolator* sself);
extern "C" void vtk_quaternion_interpolator_remove_quaternion(vtkQuaternionInterpolator* sself, double t);
extern "C" int vtk_quaternion_interpolator_get_search_method(vtkQuaternionInterpolator* sself);
extern "C" void vtk_quaternion_interpolator_set_search_method(vtkQuaternionInterpolator* sself, int type);
extern "C" void vtk_quaternion_interpolator_set_interpolation_type(vtkQuaternionInterpolator* sself, int _arg);
extern "C" int vtk_quaternion_interpolator_get_interpolation_type_min_value(vtkQuaternionInterpolator* sself);
extern "C" int vtk_quaternion_interpolator_get_interpolation_type_max_value(vtkQuaternionInterpolator* sself);
extern "C" int vtk_quaternion_interpolator_get_interpolation_type(vtkQuaternionInterpolator* sself);
extern "C" void vtk_quaternion_interpolator_set_interpolation_type_to_linear(vtkQuaternionInterpolator* sself);
extern "C" void vtk_quaternion_interpolator_set_interpolation_type_to_spline(vtkQuaternionInterpolator* sself);
extern "C" vtkRungeKutta2 * vtkRungeKutta2_new () ;
extern "C" void vtkRungeKutta2_destructor (vtkRungeKutta2 * sself) ;
extern "C" vtkRungeKutta4 * vtkRungeKutta4_new () ;
extern "C" void vtkRungeKutta4_destructor (vtkRungeKutta4 * sself) ;
extern "C" vtkRungeKutta45 * vtkRungeKutta45_new () ;
extern "C" void vtkRungeKutta45_destructor (vtkRungeKutta45 * sself) ;
