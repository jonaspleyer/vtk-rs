#include "vtk_poly_data.h"
#include "cxx.h"
#include "vtk_poly_data.rs.h"

#include <vtkPolyData.h>

vtkPolyData* poly_data_new() {
    return vtkPolyData::New();
}

void poly_data_delete(vtkPolyData& poly_data) {
    poly_data.Delete();
}
