#include "vtk_information_vector.h"
#include "vtk_information_vector.rs.h"

#include <vtkInformationVector.h>

vtkInformationVector* vtk_information_vector_new() {
    return vtkInformationVector::New();
}

void vtk_information_vector_delete(vtkInformationVector& information_vector) {
    information_vector.Delete();
}
