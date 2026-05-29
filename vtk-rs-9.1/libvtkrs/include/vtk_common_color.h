// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkColorSeries.h>
#include<vtkNamedColors.h>

// Declare exported functions
extern "C" vtkColorSeries * vtkColorSeries_new () ;
extern "C" void vtkColorSeries_destructor (vtkColorSeries * sself) ;
extern "C" void vtk_color_series_set_color_scheme(vtkColorSeries* sself, int scheme);
extern "C" int vtk_color_series_get_number_of_color_schemes(vtkColorSeries* sself);
extern "C" int vtk_color_series_get_color_scheme(vtkColorSeries* sself);
extern "C" int vtk_color_series_get_number_of_colors(vtkColorSeries* sself);
extern "C" void vtk_color_series_set_number_of_colors(vtkColorSeries* sself, int numColors);
extern "C" void vtk_color_series_remove_color(vtkColorSeries* sself, int index);
extern "C" void vtk_color_series_clear_colors(vtkColorSeries* sself);
extern "C" vtkNamedColors * vtkNamedColors_new () ;
extern "C" void vtkNamedColors_destructor (vtkNamedColors * sself) ;
extern "C" int vtk_named_colors_get_number_of_colors(vtkNamedColors* sself);
extern "C" void vtk_named_colors_reset_colors(vtkNamedColors* sself);
