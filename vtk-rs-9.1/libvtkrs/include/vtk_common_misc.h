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

// Declare exported functions
extern "C" vtkNew < vtkContourValues > vtkContourValues_new () ;
extern "C" void vtkContourValues_destructor (vtkNew < vtkContourValues > sself) ;
extern "C" void * vtkContourValues_get_ptr (vtkNew < vtkContourValues > sself) ;
extern "C" vtkNew < vtkExprTkFunctionParser > vtkExprTkFunctionParser_new () ;
extern "C" void vtkExprTkFunctionParser_destructor (vtkNew < vtkExprTkFunctionParser > sself) ;
extern "C" void * vtkExprTkFunctionParser_get_ptr (vtkNew < vtkExprTkFunctionParser > sself) ;
extern "C" vtkNew < vtkFunctionParser > vtkFunctionParser_new () ;
extern "C" void vtkFunctionParser_destructor (vtkNew < vtkFunctionParser > sself) ;
extern "C" void * vtkFunctionParser_get_ptr (vtkNew < vtkFunctionParser > sself) ;
extern "C" vtkNew < vtkHeap > vtkHeap_new () ;
extern "C" void vtkHeap_destructor (vtkNew < vtkHeap > sself) ;
extern "C" void * vtkHeap_get_ptr (vtkNew < vtkHeap > sself) ;
extern "C" vtkNew < vtkResourceFileLocator > vtkResourceFileLocator_new () ;
extern "C" void vtkResourceFileLocator_destructor (vtkNew < vtkResourceFileLocator > sself) ;
extern "C" void * vtkResourceFileLocator_get_ptr (vtkNew < vtkResourceFileLocator > sself) ;
