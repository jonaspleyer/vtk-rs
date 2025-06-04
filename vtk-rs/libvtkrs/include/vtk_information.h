#pragma once
#include "cxx.h"

#include <vtkInformation.h>

vtkInformation* vtk_information_new();
void vtk_information_delete(vtkInformation& information);
