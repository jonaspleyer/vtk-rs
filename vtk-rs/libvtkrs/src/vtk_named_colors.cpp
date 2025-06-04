#include "vtk_named_colors.h"
#include "cxx.h"
#include "vtk_named_colors.rs.h"

#include <vtkPolyData.h>

vtkNamedColors* named_colors_new() {
    return vtkNamedColors::New();
}

void named_colors_delete(vtkNamedColors& named_colors) {
    named_colors.Delete();
}
