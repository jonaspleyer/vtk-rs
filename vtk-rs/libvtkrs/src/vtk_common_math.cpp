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
extern "C" vtkNew < vtkAmoebaMinimizer > vtkAmoebaMinimizer_new () {return vtkNew < vtkAmoebaMinimizer > () ;}
extern "C" void vtkAmoebaMinimizer_destructor (vtkNew < vtkAmoebaMinimizer > sself) {sself . Reset () ; return ;}
extern "C" void * vtkAmoebaMinimizer_get_ptr (vtkNew < vtkAmoebaMinimizer > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkFFT > vtkFFT_new () {return vtkNew < vtkFFT > () ;}
extern "C" void vtkFFT_destructor (vtkNew < vtkFFT > sself) {sself . Reset () ; return ;}
extern "C" void * vtkFFT_get_ptr (vtkNew < vtkFFT > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMatrix3x3 > vtkMatrix3x3_new () {return vtkNew < vtkMatrix3x3 > () ;}
extern "C" void vtkMatrix3x3_destructor (vtkNew < vtkMatrix3x3 > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMatrix3x3_get_ptr (vtkNew < vtkMatrix3x3 > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkMatrix4x4 > vtkMatrix4x4_new () {return vtkNew < vtkMatrix4x4 > () ;}
extern "C" void vtkMatrix4x4_destructor (vtkNew < vtkMatrix4x4 > sself) {sself . Reset () ; return ;}
extern "C" void * vtkMatrix4x4_get_ptr (vtkNew < vtkMatrix4x4 > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkPolynomialSolversUnivariate > vtkPolynomialSolversUnivariate_new () {return vtkNew < vtkPolynomialSolversUnivariate > () ;}
extern "C" void vtkPolynomialSolversUnivariate_destructor (vtkNew < vtkPolynomialSolversUnivariate > sself) {sself . Reset () ; return ;}
extern "C" void * vtkPolynomialSolversUnivariate_get_ptr (vtkNew < vtkPolynomialSolversUnivariate > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkQuaternionInterpolator > vtkQuaternionInterpolator_new () {return vtkNew < vtkQuaternionInterpolator > () ;}
extern "C" void vtkQuaternionInterpolator_destructor (vtkNew < vtkQuaternionInterpolator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkQuaternionInterpolator_get_ptr (vtkNew < vtkQuaternionInterpolator > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkRungeKutta2 > vtkRungeKutta2_new () {return vtkNew < vtkRungeKutta2 > () ;}
extern "C" void vtkRungeKutta2_destructor (vtkNew < vtkRungeKutta2 > sself) {sself . Reset () ; return ;}
extern "C" void * vtkRungeKutta2_get_ptr (vtkNew < vtkRungeKutta2 > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkRungeKutta4 > vtkRungeKutta4_new () {return vtkNew < vtkRungeKutta4 > () ;}
extern "C" void vtkRungeKutta4_destructor (vtkNew < vtkRungeKutta4 > sself) {sself . Reset () ; return ;}
extern "C" void * vtkRungeKutta4_get_ptr (vtkNew < vtkRungeKutta4 > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkRungeKutta45 > vtkRungeKutta45_new () {return vtkNew < vtkRungeKutta45 > () ;}
extern "C" void vtkRungeKutta45_destructor (vtkNew < vtkRungeKutta45 > sself) {sself . Reset () ; return ;}
extern "C" void * vtkRungeKutta45_get_ptr (vtkNew < vtkRungeKutta45 > sself) {return sself . GetPointer () ;}
