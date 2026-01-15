// Include header file
#include<vtk_common_color.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkColorSeries.h>
#include<vtkNamedColors.h>

// Implement declared functions
extern "C" vtkNew < vtkColorSeries > vtkColorSeries_new () {return vtkNew < vtkColorSeries > () ;}
extern "C" void vtkColorSeries_destructor (vtkNew < vtkColorSeries > sself) {sself . Reset () ; return ;}
extern "C" void * vtkColorSeries_get_ptr (vtkNew < vtkColorSeries > sself) {return sself . GetPointer () ;}
extern "C" vtkNew < vtkNamedColors > vtkNamedColors_new () {return vtkNew < vtkNamedColors > () ;}
extern "C" void vtkNamedColors_destructor (vtkNew < vtkNamedColors > sself) {sself . Reset () ; return ;}
extern "C" void * vtkNamedColors_get_ptr (vtkNew < vtkNamedColors > sself) {return sself . GetPointer () ;}
