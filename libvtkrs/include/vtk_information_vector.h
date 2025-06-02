#pragma once
#include "cxx.h"

#include <vtkInformationVector.h>

vtkInformationVector* vtk_information_vector_new();
void vtk_information_vector_delete(vtkInformationVector& information_vector);
