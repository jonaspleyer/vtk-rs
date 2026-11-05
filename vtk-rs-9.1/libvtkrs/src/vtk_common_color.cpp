// Include header file
#include <vtk_common_color.h>

// Default include in all modules
#include <vtkNew.h>
#include <vtkObjectBase.h>

// Include objects of this module
#include <vtkColorSeries.h>
#include <vtkNamedColors.h>

// Implement declared functions
extern "C" vtkNew<vtkColorSeries> vtkColorSeries_new() {
    return vtkNew<vtkColorSeries>();
}
extern "C" void vtkColorSeries_destructor(vtkNew<vtkColorSeries> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkColorSeries_get_ptr(vtkNew<vtkColorSeries> sself) {
    return sself.GetPointer();
}
extern "C" void vtk_color_series_set_color_scheme(vtkNew<vtkColorSeries> self, int scheme) {
    return self->SetColorScheme(scheme);
}
extern "C" int vtk_color_series_get_number_of_color_schemes(vtkNew<vtkColorSeries> self) {
    return self->GetNumberOfColorSchemes();
}
extern "C" int vtk_color_series_get_color_scheme(vtkNew<vtkColorSeries> self) {
    return self->GetColorScheme();
}
extern "C" int vtk_color_series_get_number_of_colors(vtkNew<vtkColorSeries> self) {
    return self->GetNumberOfColors();
}
extern "C" void vtk_color_series_set_number_of_colors(vtkNew<vtkColorSeries> self, int numColors) {
    return self->SetNumberOfColors(numColors);
}
extern "C" void vtk_color_series_remove_color(vtkNew<vtkColorSeries> self, int index) {
    return self->RemoveColor(index);
}
extern "C" void vtk_color_series_clear_colors(vtkNew<vtkColorSeries> self) {
    return self->ClearColors();
}
extern "C" vtkNew<vtkNamedColors> vtkNamedColors_new() {
    return vtkNew<vtkNamedColors>();
}
extern "C" void vtkNamedColors_destructor(vtkNew<vtkNamedColors> sself) {
    sself.Reset();
    return;
}
extern "C" void* vtkNamedColors_get_ptr(vtkNew<vtkNamedColors> sself) {
    return sself.GetPointer();
}
extern "C" int vtk_named_colors_get_number_of_colors(vtkNew<vtkNamedColors> self) {
    return self->GetNumberOfColors();
}
extern "C" void vtk_named_colors_reset_colors(vtkNew<vtkNamedColors> self) {
    return self->ResetColors();
}
