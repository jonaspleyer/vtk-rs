#include "vtk_information.h"
#include "vtk_information.rs.h"

#include <vtkInformation.h>

vtkInformation* vtk_information_new() {
    return vtkInformation::New();
}

void vtk_information_delete(vtkInformation& information) {
    information.Delete();
}
