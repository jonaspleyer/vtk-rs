// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkColorSeries.h>
#include<vtkNamedColors.h>

// Declare exported functions
extern "C" vtkNew < vtkColorSeries > vtkColorSeries_new () ;
extern "C" void vtkColorSeries_destructor (vtkNew < vtkColorSeries > sself) ;
extern "C" void * vtkColorSeries_get_ptr (vtkNew < vtkColorSeries > sself) ;
extern "C" vtkNew < vtkNamedColors > vtkNamedColors_new () ;
extern "C" void vtkNamedColors_destructor (vtkNew < vtkNamedColors > sself) ;
extern "C" void * vtkNamedColors_get_ptr (vtkNew < vtkNamedColors > sself) ;
