#include "vtk_algorithm.h"
#include "vtk_algorithm.rs.h"

#include <vtkAlgorithm.h>

vtkAlgorithm* vtk_algorithm_new() {
    return vtkAlgorithm::New();
}

void vtk_algorithm_delete(vtkAlgorithm& algorithm) {
    algorithm.Delete();
}
