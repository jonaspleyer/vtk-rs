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
extern "C" vtkNew < vtkAmoebaMinimizer > vtkAmoebaMinimizer_new () ;
extern "C" void vtkAmoebaMinimizer_destructor (vtkNew < vtkAmoebaMinimizer > sself) ;
extern "C" void * vtkAmoebaMinimizer_get_ptr (vtkNew < vtkAmoebaMinimizer > sself) ;
extern "C" vtkNew < vtkFFT > vtkFFT_new () ;
extern "C" void vtkFFT_destructor (vtkNew < vtkFFT > sself) ;
extern "C" void * vtkFFT_get_ptr (vtkNew < vtkFFT > sself) ;
extern "C" vtkNew < vtkMatrix3x3 > vtkMatrix3x3_new () ;
extern "C" void vtkMatrix3x3_destructor (vtkNew < vtkMatrix3x3 > sself) ;
extern "C" void * vtkMatrix3x3_get_ptr (vtkNew < vtkMatrix3x3 > sself) ;
extern "C" vtkNew < vtkMatrix4x4 > vtkMatrix4x4_new () ;
extern "C" void vtkMatrix4x4_destructor (vtkNew < vtkMatrix4x4 > sself) ;
extern "C" void * vtkMatrix4x4_get_ptr (vtkNew < vtkMatrix4x4 > sself) ;
extern "C" vtkNew < vtkPolynomialSolversUnivariate > vtkPolynomialSolversUnivariate_new () ;
extern "C" void vtkPolynomialSolversUnivariate_destructor (vtkNew < vtkPolynomialSolversUnivariate > sself) ;
extern "C" void * vtkPolynomialSolversUnivariate_get_ptr (vtkNew < vtkPolynomialSolversUnivariate > sself) ;
extern "C" vtkNew < vtkQuaternionInterpolator > vtkQuaternionInterpolator_new () ;
extern "C" void vtkQuaternionInterpolator_destructor (vtkNew < vtkQuaternionInterpolator > sself) ;
extern "C" void * vtkQuaternionInterpolator_get_ptr (vtkNew < vtkQuaternionInterpolator > sself) ;
extern "C" vtkNew < vtkRungeKutta2 > vtkRungeKutta2_new () ;
extern "C" void vtkRungeKutta2_destructor (vtkNew < vtkRungeKutta2 > sself) ;
extern "C" void * vtkRungeKutta2_get_ptr (vtkNew < vtkRungeKutta2 > sself) ;
extern "C" vtkNew < vtkRungeKutta4 > vtkRungeKutta4_new () ;
extern "C" void vtkRungeKutta4_destructor (vtkNew < vtkRungeKutta4 > sself) ;
extern "C" void * vtkRungeKutta4_get_ptr (vtkNew < vtkRungeKutta4 > sself) ;
extern "C" vtkNew < vtkRungeKutta45 > vtkRungeKutta45_new () ;
extern "C" void vtkRungeKutta45_destructor (vtkNew < vtkRungeKutta45 > sself) ;
extern "C" void * vtkRungeKutta45_get_ptr (vtkNew < vtkRungeKutta45 > sself) ;
