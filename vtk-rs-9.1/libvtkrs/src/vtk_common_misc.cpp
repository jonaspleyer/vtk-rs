// Include header file
#include<vtk_common_misc.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkContourValues.h>
#include<vtkErrorCode.h>
#include<vtkExprTkFunctionParser.h>
#include<vtkFunctionParser.h>
#include<vtkHeap.h>
#include<vtkPolygonBuilder.h>
#include<vtkResourceFileLocator.h>

// Implement declared functions
extern "C" vtkNew < vtkContourValues > vtkContourValues_new () {return vtkNew < vtkContourValues > () ;}
extern "C" void vtkContourValues_destructor (vtkNew < vtkContourValues > sself) {sself . Reset () ; return ;}
extern "C" void * vtkContourValues_get_ptr (vtkNew < vtkContourValues > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkExprTkFunctionParser > vtkExprTkFunctionParser_new () {return vtkNew < vtkExprTkFunctionParser > () ;}
extern "C" void vtkExprTkFunctionParser_destructor (vtkNew < vtkExprTkFunctionParser > sself) {sself . Reset () ; return ;}
extern "C" void * vtkExprTkFunctionParser_get_ptr (vtkNew < vtkExprTkFunctionParser > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkFunctionParser > vtkFunctionParser_new () {return vtkNew < vtkFunctionParser > () ;}
extern "C" void vtkFunctionParser_destructor (vtkNew < vtkFunctionParser > sself) {sself . Reset () ; return ;}
extern "C" void * vtkFunctionParser_get_ptr (vtkNew < vtkFunctionParser > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkHeap > vtkHeap_new () {return vtkNew < vtkHeap > () ;}
extern "C" void vtkHeap_destructor (vtkNew < vtkHeap > sself) {sself . Reset () ; return ;}
extern "C" void * vtkHeap_get_ptr (vtkNew < vtkHeap > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkResourceFileLocator > vtkResourceFileLocator_new () {return vtkNew < vtkResourceFileLocator > () ;}
extern "C" void vtkResourceFileLocator_destructor (vtkNew < vtkResourceFileLocator > sself) {sself . Reset () ; return ;}
extern "C" void * vtkResourceFileLocator_get_ptr (vtkNew < vtkResourceFileLocator > sself) {return sself . GetPointer () ;}
