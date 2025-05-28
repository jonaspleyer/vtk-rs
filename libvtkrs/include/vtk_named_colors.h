#pragma once
#include "cxx.h"

#include <vtkNamedColors.h>
#include <vtkNew.h>

vtkNamedColors* named_colors_new();
void named_colors_delete(vtkNamedColors& named_colors);
