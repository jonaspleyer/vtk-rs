#include "vtk_algorithm.h"
#include "vtk_algorithm.rs.h"

#include <vtkAlgorithm.h>

vtkAlgorithm* algorithm_new() {
    return vtkAlgorithm::New();
}

void algorithm_delete(vtkAlgorithm& algorithm) {
    algorithm.Delete();
}
