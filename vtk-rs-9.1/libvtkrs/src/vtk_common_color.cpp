// Include header file
#include<vtk_common_color.h>

// Default include in all modules
#include<vtkNew.h>
#include<vtkObjectBase.h>

// Include objects of this module
#include<vtkColorSeries.h>
#include<vtkNamedColors.h>

// Implement declared functions
extern "C" vtkColorSeries * vtkColorSeries_new () {return vtkColorSeries :: New () ;}
extern "C" void vtkColorSeries_destructor (vtkColorSeries * sself) {sself -> Delete () ; return ;}
extern "C" void vtk_color_series_set_color_scheme(vtkColorSeries* sself, int scheme) { sself->SetColorScheme(scheme); }
extern "C" int vtk_color_series_get_number_of_color_schemes(vtkColorSeries* sself) { return sself->GetNumberOfColorSchemes(); }
extern "C" int vtk_color_series_get_color_scheme(vtkColorSeries* sself) { return sself->GetColorScheme(); }
extern "C" int vtk_color_series_get_number_of_colors(vtkColorSeries* sself) { return sself->GetNumberOfColors(); }
extern "C" void vtk_color_series_set_number_of_colors(vtkColorSeries* sself, int numColors) { sself->SetNumberOfColors(numColors); }
extern "C" void vtk_color_series_remove_color(vtkColorSeries* sself, int index) { sself->RemoveColor(index); }
extern "C" void vtk_color_series_clear_colors(vtkColorSeries* sself) { sself->ClearColors(); }
extern "C" vtkNamedColors * vtkNamedColors_new () {return vtkNamedColors :: New () ;}
extern "C" void vtkNamedColors_destructor (vtkNamedColors * sself) {sself -> Delete () ; return ;}
extern "C" int vtk_named_colors_get_number_of_colors(vtkNamedColors* sself) { return sself->GetNumberOfColors(); }
extern "C" void vtk_named_colors_reset_colors(vtkNamedColors* sself) { sself->ResetColors(); }
