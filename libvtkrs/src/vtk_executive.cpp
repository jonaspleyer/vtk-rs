#include "vtk_executive.h"
#include "vtk_executive.rs.h"

#include <vtkExecutive.h>

void vtk_executive_delete(vtkExecutive& executive) {
    executive.Delete();
}
